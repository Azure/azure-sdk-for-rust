#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Android Policy entity for Intune MAM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AndroidMamPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Intune MAM iOS Policy Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AndroidMamPolicyProperties>,
}
impl AndroidMamPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidMamPolicyCollection {
    pub value: Vec<AndroidMamPolicy>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for AndroidMamPolicyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl AndroidMamPolicyCollection {
    pub fn new(value: Vec<AndroidMamPolicy>) -> Self {
        Self { value, nextlink: None }
    }
}
#[doc = "Intune MAM iOS Policy Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidMamPolicyProperties {
    #[serde(flatten)]
    pub mam_policy_properties: MamPolicyProperties,
    #[serde(rename = "screenCapture", default, skip_serializing_if = "Option::is_none")]
    pub screen_capture: Option<android_mam_policy_properties::ScreenCapture>,
    #[serde(rename = "fileEncryption", default, skip_serializing_if = "Option::is_none")]
    pub file_encryption: Option<android_mam_policy_properties::FileEncryption>,
}
impl AndroidMamPolicyProperties {
    pub fn new(mam_policy_properties: MamPolicyProperties) -> Self {
        Self {
            mam_policy_properties,
            screen_capture: None,
            file_encryption: None,
        }
    }
}
pub mod android_mam_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScreenCapture {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "block")]
        Block,
    }
    impl Default for ScreenCapture {
        fn default() -> Self {
            Self::Allow
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FileEncryption {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "notRequired")]
        NotRequired,
    }
    impl Default for FileEncryption {
        fn default() -> Self {
            Self::Required
        }
    }
}
#[doc = "Application entity for Intune MAM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Application {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationProperties>,
}
impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCollection {
    pub value: Vec<Application>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for ApplicationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl ApplicationCollection {
    pub fn new(value: Vec<Application>) -> Self {
        Self { value, nextlink: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    pub platform: application_properties::Platform,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}
impl ApplicationProperties {
    pub fn new(friendly_name: String, platform: application_properties::Platform) -> Self {
        Self {
            friendly_name,
            platform,
            app_id: None,
        }
    }
}
pub mod application_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Platform {
        #[serde(rename = "ios")]
        Ios,
        #[serde(rename = "android")]
        Android,
        #[serde(rename = "windows")]
        Windows,
    }
    impl Default for Platform {
        fn default() -> Self {
            Self::Ios
        }
    }
}
#[doc = "Device entity for Intune."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Device {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeviceProperties>,
}
impl Device {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCollection {
    pub value: Vec<Device>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for DeviceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl DeviceCollection {
    pub fn new(value: Vec<Device>) -> Self {
        Self { value, nextlink: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceProperties {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    pub platform: String,
    #[serde(rename = "platformVersion")]
    pub platform_version: String,
    #[serde(rename = "deviceType")]
    pub device_type: String,
}
impl DeviceProperties {
    pub fn new(user_id: String, friendly_name: String, platform: String, platform_version: String, device_type: String) -> Self {
        Self {
            user_id,
            friendly_name,
            platform,
            platform_version,
            device_type,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Flagged Enrolled App for the given tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlaggedEnrolledApp {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FlaggedEnrolledAppProperties>,
}
impl FlaggedEnrolledApp {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Flagged Enrolled App collection for the given tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlaggedEnrolledAppCollection {
    pub value: Vec<FlaggedEnrolledApp>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for FlaggedEnrolledAppCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl FlaggedEnrolledAppCollection {
    pub fn new(value: Vec<FlaggedEnrolledApp>) -> Self {
        Self { value, nextlink: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlaggedEnrolledAppError {
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}
impl FlaggedEnrolledAppError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlaggedEnrolledAppProperties {
    #[serde(rename = "deviceType", default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<FlaggedEnrolledAppError>,
}
impl FlaggedEnrolledAppProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Flagged user for the given tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlaggedUser {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FlaggedUserProperties>,
}
impl FlaggedUser {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Flagged user collection for the given tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlaggedUserCollection {
    pub value: Vec<FlaggedUser>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for FlaggedUserCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl FlaggedUserCollection {
    pub fn new(value: Vec<FlaggedUser>) -> Self {
        Self { value, nextlink: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlaggedUserProperties {
    #[serde(rename = "errorCount", default, skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl FlaggedUserProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Group entity for Intune MAM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupItem {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GroupProperties>,
}
impl GroupItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupProperties {
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
}
impl GroupProperties {
    pub fn new(friendly_name: String) -> Self {
        Self { friendly_name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupsCollection {
    pub value: Vec<GroupItem>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for GroupsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl GroupsCollection {
    pub fn new(value: Vec<GroupItem>) -> Self {
        Self { value, nextlink: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosmamPolicyCollection {
    pub value: Vec<IOsmamPolicy>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for IosmamPolicyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl IosmamPolicyCollection {
    pub fn new(value: Vec<IOsmamPolicy>) -> Self {
        Self { value, nextlink: None }
    }
}
#[doc = "Location entity for given tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Location {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LocationProperties>,
}
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationCollection {
    pub value: Vec<Location>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for LocationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl LocationCollection {
    pub fn new(value: Vec<Location>) -> Self {
        Self { value, nextlink: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationProperties {
    #[serde(rename = "hostName")]
    pub host_name: String,
}
impl LocationProperties {
    pub fn new(host_name: String) -> Self {
        Self { host_name }
    }
}
#[doc = "MAM Policy request body for properties Intune MAM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MamPolicyAppIdOrGroupIdPayload {
    #[doc = "Android Policy request body for Intune MAM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MamPolicyAppOrGroupIdProperties>,
}
impl MamPolicyAppIdOrGroupIdPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Android Policy request body for Intune MAM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MamPolicyAppOrGroupIdProperties {
    pub url: String,
}
impl MamPolicyAppOrGroupIdProperties {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MamPolicyProperties {
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "appSharingFromLevel", default, skip_serializing_if = "Option::is_none")]
    pub app_sharing_from_level: Option<mam_policy_properties::AppSharingFromLevel>,
    #[serde(rename = "appSharingToLevel", default, skip_serializing_if = "Option::is_none")]
    pub app_sharing_to_level: Option<mam_policy_properties::AppSharingToLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<mam_policy_properties::Authentication>,
    #[serde(rename = "clipboardSharingLevel", default, skip_serializing_if = "Option::is_none")]
    pub clipboard_sharing_level: Option<mam_policy_properties::ClipboardSharingLevel>,
    #[serde(rename = "dataBackup", default, skip_serializing_if = "Option::is_none")]
    pub data_backup: Option<mam_policy_properties::DataBackup>,
    #[serde(rename = "fileSharingSaveAs", default, skip_serializing_if = "Option::is_none")]
    pub file_sharing_save_as: Option<mam_policy_properties::FileSharingSaveAs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin: Option<mam_policy_properties::Pin>,
    #[serde(rename = "pinNumRetry", default, skip_serializing_if = "Option::is_none")]
    pub pin_num_retry: Option<i64>,
    #[serde(rename = "deviceCompliance", default, skip_serializing_if = "Option::is_none")]
    pub device_compliance: Option<mam_policy_properties::DeviceCompliance>,
    #[serde(rename = "managedBrowser", default, skip_serializing_if = "Option::is_none")]
    pub managed_browser: Option<mam_policy_properties::ManagedBrowser>,
    #[serde(rename = "accessRecheckOfflineTimeout", default, skip_serializing_if = "Option::is_none")]
    pub access_recheck_offline_timeout: Option<String>,
    #[serde(rename = "accessRecheckOnlineTimeout", default, skip_serializing_if = "Option::is_none")]
    pub access_recheck_online_timeout: Option<String>,
    #[serde(rename = "offlineWipeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub offline_wipe_timeout: Option<String>,
    #[serde(rename = "numOfApps", default, skip_serializing_if = "Option::is_none")]
    pub num_of_apps: Option<i64>,
    #[serde(rename = "groupStatus", default, skip_serializing_if = "Option::is_none")]
    pub group_status: Option<mam_policy_properties::GroupStatus>,
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
}
impl MamPolicyProperties {
    pub fn new(friendly_name: String) -> Self {
        Self {
            friendly_name,
            description: None,
            app_sharing_from_level: None,
            app_sharing_to_level: None,
            authentication: None,
            clipboard_sharing_level: None,
            data_backup: None,
            file_sharing_save_as: None,
            pin: None,
            pin_num_retry: None,
            device_compliance: None,
            managed_browser: None,
            access_recheck_offline_timeout: None,
            access_recheck_online_timeout: None,
            offline_wipe_timeout: None,
            num_of_apps: None,
            group_status: None,
            last_modified_time: None,
        }
    }
}
pub mod mam_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AppSharingFromLevel {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "policyManagedApps")]
        PolicyManagedApps,
        #[serde(rename = "allApps")]
        AllApps,
    }
    impl Default for AppSharingFromLevel {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AppSharingToLevel {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "policyManagedApps")]
        PolicyManagedApps,
        #[serde(rename = "allApps")]
        AllApps,
    }
    impl Default for AppSharingToLevel {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Authentication {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "notRequired")]
        NotRequired,
    }
    impl Default for Authentication {
        fn default() -> Self {
            Self::Required
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ClipboardSharingLevel {
        #[serde(rename = "blocked")]
        Blocked,
        #[serde(rename = "policyManagedApps")]
        PolicyManagedApps,
        #[serde(rename = "policyManagedAppsWithPasteIn")]
        PolicyManagedAppsWithPasteIn,
        #[serde(rename = "allApps")]
        AllApps,
    }
    impl Default for ClipboardSharingLevel {
        fn default() -> Self {
            Self::Blocked
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataBackup {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "block")]
        Block,
    }
    impl Default for DataBackup {
        fn default() -> Self {
            Self::Allow
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FileSharingSaveAs {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "block")]
        Block,
    }
    impl Default for FileSharingSaveAs {
        fn default() -> Self {
            Self::Allow
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Pin {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "notRequired")]
        NotRequired,
    }
    impl Default for Pin {
        fn default() -> Self {
            Self::Required
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceCompliance {
        #[serde(rename = "enable")]
        Enable,
        #[serde(rename = "disable")]
        Disable,
    }
    impl Default for DeviceCompliance {
        fn default() -> Self {
            Self::Enable
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ManagedBrowser {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "notRequired")]
        NotRequired,
    }
    impl Default for ManagedBrowser {
        fn default() -> Self {
            Self::Required
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GroupStatus {
        #[serde(rename = "notTargeted")]
        NotTargeted,
        #[serde(rename = "targeted")]
        Targeted,
    }
    impl Default for GroupStatus {
        fn default() -> Self {
            Self::NotTargeted
        }
    }
}
#[doc = "OperationResult entity for Intune."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationResultProperties>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResultCollection {
    pub value: Vec<OperationResult>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for OperationResultCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl OperationResultCollection {
    pub fn new(value: Vec<OperationResult>) -> Self {
        Self { value, nextlink: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResultProperties {
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "operationMetadata")]
    pub operation_metadata: Vec<OperationMetadataProperties>,
}
impl OperationResultProperties {
    pub fn new(friendly_name: String, operation_metadata: Vec<OperationMetadataProperties>) -> Self {
        Self {
            friendly_name,
            category: None,
            last_modified_time: None,
            state: None,
            operation_metadata,
        }
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
    #[doc = "Resource Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource Location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Default Statuses entity for the given tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusesDefault {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StatusesProperties>,
    #[doc = "Gets the URL to get the next set of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nextlink: Option<String>,
}
impl azure_core::Continuable for StatusesDefault {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.nextlink.clone()
    }
}
impl StatusesDefault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusesProperties {
    #[serde(rename = "deployedPolicies", default, skip_serializing_if = "Option::is_none")]
    pub deployed_policies: Option<i64>,
    #[serde(rename = "enrolledUsers", default, skip_serializing_if = "Option::is_none")]
    pub enrolled_users: Option<i64>,
    #[serde(rename = "flaggedUsers", default, skip_serializing_if = "Option::is_none")]
    pub flagged_users: Option<i64>,
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[serde(rename = "policyAppliedUsers", default, skip_serializing_if = "Option::is_none")]
    pub policy_applied_users: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "wipeFailedApps", default, skip_serializing_if = "Option::is_none")]
    pub wipe_failed_apps: Option<i64>,
    #[serde(rename = "wipePendingApps", default, skip_serializing_if = "Option::is_none")]
    pub wipe_pending_apps: Option<i64>,
    #[serde(rename = "wipeSucceededApps", default, skip_serializing_if = "Option::is_none")]
    pub wipe_succeeded_apps: Option<i64>,
}
impl StatusesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Device entity for Intune."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WipeDeviceOperationResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WipeDeviceOperationResultProperties>,
}
impl WipeDeviceOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WipeDeviceOperationResultProperties {
    pub value: String,
}
impl WipeDeviceOperationResultProperties {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "iOS Policy entity for Intune MAM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IOsmamPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Intune MAM iOS Policy Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IOsmamPolicyProperties>,
}
impl IOsmamPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Intune MAM iOS Policy Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IOsmamPolicyProperties {
    #[serde(flatten)]
    pub mam_policy_properties: MamPolicyProperties,
    #[serde(rename = "fileEncryptionLevel", default, skip_serializing_if = "Option::is_none")]
    pub file_encryption_level: Option<i_osmam_policy_properties::FileEncryptionLevel>,
    #[serde(rename = "touchId", default, skip_serializing_if = "Option::is_none")]
    pub touch_id: Option<i_osmam_policy_properties::TouchId>,
}
impl IOsmamPolicyProperties {
    pub fn new(mam_policy_properties: MamPolicyProperties) -> Self {
        Self {
            mam_policy_properties,
            file_encryption_level: None,
            touch_id: None,
        }
    }
}
pub mod i_osmam_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FileEncryptionLevel {
        #[serde(rename = "deviceLocked")]
        DeviceLocked,
        #[serde(rename = "deviceLockedExceptFilesOpen")]
        DeviceLockedExceptFilesOpen,
        #[serde(rename = "afterDeviceRestart")]
        AfterDeviceRestart,
        #[serde(rename = "useDeviceSettings")]
        UseDeviceSettings,
    }
    impl Default for FileEncryptionLevel {
        fn default() -> Self {
            Self::DeviceLocked
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TouchId {
        #[serde(rename = "enable")]
        Enable,
        #[serde(rename = "disable")]
        Disable,
    }
    impl Default for TouchId {
        fn default() -> Self {
            Self::Enable
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetadataProperties {
    pub name: String,
    pub value: String,
}
impl OperationMetadataProperties {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
