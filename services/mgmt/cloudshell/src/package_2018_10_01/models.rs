#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Cloud shell console"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudShellConsole {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Cloud shell console properties."]
    pub properties: ConsoleProperties,
}
impl CloudShellConsole {
    pub fn new(properties: ConsoleProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Cloud shell patch operation user settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudShellPatchUserSettings {
    #[doc = "The cloud shell user settings properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserProperties>,
}
impl CloudShellPatchUserSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cloud shell user settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudShellUserSettings {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The cloud shell user settings properties."]
    pub properties: UserProperties,
}
impl CloudShellUserSettings {
    pub fn new(properties: UserProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Cloud shell properties for creating a console."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsoleCreateProperties {
    #[doc = "The operating system type of the cloud shell."]
    #[serde(rename = "osType")]
    pub os_type: console_create_properties::OsType,
    #[doc = "Provisioning state of the console."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<console_create_properties::ProvisioningState>,
    #[doc = "Uri of the console."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl ConsoleCreateProperties {
    pub fn new(os_type: console_create_properties::OsType) -> Self {
        Self {
            os_type,
            provisioning_state: None,
            uri: None,
        }
    }
}
pub mod console_create_properties {
    use super::*;
    #[doc = "The operating system type of the cloud shell."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Linux,
        Windows,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OsType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OsType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OsType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Linux => serializer.serialize_unit_variant("OsType", 0u32, "Linux"),
                Self::Windows => serializer.serialize_unit_variant("OsType", 1u32, "Windows"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the console."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Pending,
        Updating,
        Creating,
        Repairing,
        Failed,
        Canceled,
        Succeeded,
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
                Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Pending"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::Repairing => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Repairing"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Canceled"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Console definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsoleDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Cloud shell properties for creating a console."]
    pub properties: ConsoleCreateProperties,
}
impl ConsoleDefinition {
    pub fn new(properties: ConsoleCreateProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Cloud shell console properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsoleProperties {
    #[doc = "The operating system type of the cloud shell."]
    #[serde(rename = "osType")]
    pub os_type: console_properties::OsType,
    #[doc = "Provisioning state of the console."]
    #[serde(rename = "provisioningState")]
    pub provisioning_state: console_properties::ProvisioningState,
    #[doc = "Uri of the console."]
    pub uri: String,
}
impl ConsoleProperties {
    pub fn new(os_type: console_properties::OsType, provisioning_state: console_properties::ProvisioningState, uri: String) -> Self {
        Self {
            os_type,
            provisioning_state,
            uri,
        }
    }
}
pub mod console_properties {
    use super::*;
    #[doc = "The operating system type of the cloud shell."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Linux,
        Windows,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OsType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OsType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OsType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Linux => serializer.serialize_unit_variant("OsType", 0u32, "Linux"),
                Self::Windows => serializer.serialize_unit_variant("OsType", 1u32, "Windows"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the console."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Pending,
        Updating,
        Creating,
        Repairing,
        Failed,
        Canceled,
        Succeeded,
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
                Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Pending"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::Repairing => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Repairing"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Canceled"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "The error's code."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Contains details when the response code indicates an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}
impl ErrorResponse {
    pub fn new(error: ErrorDetail) -> Self {
        Self { error }
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage profile of the user settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "Full resource ID of storage account."]
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[doc = "Name of the mounted file share. 63 characters or less, lowercase alphabet, numbers, and -"]
    #[serde(rename = "fileShareName", default, skip_serializing_if = "Option::is_none")]
    pub file_share_name: Option<String>,
    #[doc = "Size of file share"]
    #[serde(rename = "diskSizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_in_gb: Option<i32>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings for terminal appearance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TerminalSettings {
    #[doc = "Size of terminal font."]
    #[serde(rename = "fontSize", default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<terminal_settings::FontSize>,
    #[doc = "Style of terminal font."]
    #[serde(rename = "fontStyle", default, skip_serializing_if = "Option::is_none")]
    pub font_style: Option<terminal_settings::FontStyle>,
}
impl TerminalSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod terminal_settings {
    use super::*;
    #[doc = "Size of terminal font."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FontSize")]
    pub enum FontSize {
        NotSpecified,
        Small,
        Medium,
        Large,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FontSize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FontSize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FontSize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("FontSize", 0u32, "NotSpecified"),
                Self::Small => serializer.serialize_unit_variant("FontSize", 1u32, "Small"),
                Self::Medium => serializer.serialize_unit_variant("FontSize", 2u32, "Medium"),
                Self::Large => serializer.serialize_unit_variant("FontSize", 3u32, "Large"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Style of terminal font."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FontStyle")]
    pub enum FontStyle {
        NotSpecified,
        Monospace,
        Courier,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FontStyle {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FontStyle {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FontStyle {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("FontStyle", 0u32, "NotSpecified"),
                Self::Monospace => serializer.serialize_unit_variant("FontStyle", 1u32, "Monospace"),
                Self::Courier => serializer.serialize_unit_variant("FontStyle", 2u32, "Courier"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The cloud shell user settings properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProperties {
    #[doc = "The operating system type of the cloud shell. Deprecated, use preferredShellType."]
    #[serde(rename = "preferredOsType")]
    pub preferred_os_type: user_properties::PreferredOsType,
    #[doc = "The preferred location of the cloud shell."]
    #[serde(rename = "preferredLocation")]
    pub preferred_location: String,
    #[doc = "The storage profile of the user settings."]
    #[serde(rename = "storageProfile")]
    pub storage_profile: StorageProfile,
    #[doc = "Settings for terminal appearance."]
    #[serde(rename = "terminalSettings")]
    pub terminal_settings: TerminalSettings,
    #[doc = "The shell type of the cloud shell."]
    #[serde(rename = "preferredShellType")]
    pub preferred_shell_type: user_properties::PreferredShellType,
}
impl UserProperties {
    pub fn new(
        preferred_os_type: user_properties::PreferredOsType,
        preferred_location: String,
        storage_profile: StorageProfile,
        terminal_settings: TerminalSettings,
        preferred_shell_type: user_properties::PreferredShellType,
    ) -> Self {
        Self {
            preferred_os_type,
            preferred_location,
            storage_profile,
            terminal_settings,
            preferred_shell_type,
        }
    }
}
pub mod user_properties {
    use super::*;
    #[doc = "The operating system type of the cloud shell. Deprecated, use preferredShellType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredOsType")]
    pub enum PreferredOsType {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredOsType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredOsType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredOsType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("PreferredOsType", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("PreferredOsType", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The shell type of the cloud shell."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredShellType")]
    pub enum PreferredShellType {
        #[serde(rename = "bash")]
        Bash,
        #[serde(rename = "pwsh")]
        Pwsh,
        #[serde(rename = "powershell")]
        Powershell,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredShellType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredShellType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredShellType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Bash => serializer.serialize_unit_variant("PreferredShellType", 0u32, "bash"),
                Self::Pwsh => serializer.serialize_unit_variant("PreferredShellType", 1u32, "pwsh"),
                Self::Powershell => serializer.serialize_unit_variant("PreferredShellType", 2u32, "powershell"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response to get user settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSettingsResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The cloud shell user settings properties."]
    pub properties: UserProperties,
}
impl UserSettingsResponse {
    pub fn new(properties: UserProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
