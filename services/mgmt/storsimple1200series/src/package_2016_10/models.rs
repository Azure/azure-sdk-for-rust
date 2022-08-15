#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The access control record"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlRecord {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Properties of access control record"]
    pub properties: AccessControlRecordProperties,
}
impl AccessControlRecord {
    pub fn new(properties: AccessControlRecordProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Collection of AccessControlRecords"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlRecordList {
    #[doc = "The value."]
    pub value: Vec<AccessControlRecord>,
}
impl azure_core::Continuable for AccessControlRecordList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AccessControlRecordList {
    pub fn new(value: Vec<AccessControlRecord>) -> Self {
        Self { value }
    }
}
#[doc = "Properties of access control record"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlRecordProperties {
    #[doc = "The Iscsi initiator name (IQN)"]
    #[serde(rename = "initiatorName")]
    pub initiator_name: String,
}
impl AccessControlRecordProperties {
    pub fn new(initiator_name: String) -> Self {
        Self { initiator_name }
    }
}
#[doc = "Alert class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Properties of alert"]
    pub properties: AlertProperties,
}
impl Alert {
    pub fn new(properties: AlertProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Error details for the alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertErrorDetails {
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Error Message"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Number of occurrences."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurences: Option<i32>,
}
impl AlertErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Filters that can be specified on the alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertFilter {
    #[doc = "Status of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<alert_filter::Status>,
    #[doc = "Severity of the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<alert_filter::Severity>,
    #[doc = "Source of the alert"]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<alert_filter::SourceType>,
    #[doc = "Source name of the alert"]
    #[serde(rename = "sourceName", default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[doc = "UTC time on which the alert appeared"]
    #[serde(rename = "appearedOnTime", with = "azure_core::date::rfc3339::option")]
    pub appeared_on_time: Option<time::OffsetDateTime>,
}
impl AlertFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_filter {
    use super::*;
    #[doc = "Status of the alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Cleared,
    }
    #[doc = "Severity of the alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Informational,
        Warning,
        Critical,
    }
    #[doc = "Source of the alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SourceType {
        Resource,
        Device,
    }
}
#[doc = "Collection of Alerts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertList {
    #[doc = "The value."]
    pub value: Vec<Alert>,
    #[doc = "Id of the next page of alerts"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AlertList {
    pub fn new(value: Vec<Alert>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProperties {
    #[doc = "Title of the alert"]
    pub title: String,
    #[doc = "Device or Resource alert"]
    pub scope: alert_properties::Scope,
    #[doc = "Type of the alert"]
    #[serde(rename = "alertType")]
    pub alert_type: String,
    #[doc = "UTC time at which the alert appeared"]
    #[serde(rename = "appearedAtTime", with = "azure_core::date::rfc3339")]
    pub appeared_at_time: time::OffsetDateTime,
    #[doc = "UTC time at which the alert appeared on the source"]
    #[serde(rename = "appearedAtSourceTime", with = "azure_core::date::rfc3339")]
    pub appeared_at_source_time: time::OffsetDateTime,
    #[doc = "UTC time at which the alert got cleared"]
    #[serde(rename = "clearedAtTime", with = "azure_core::date::rfc3339::option")]
    pub cleared_at_time: Option<time::OffsetDateTime>,
    #[doc = "UTC time at which the alert was cleared on the source"]
    #[serde(rename = "clearedAtSourceTime", with = "azure_core::date::rfc3339::option")]
    pub cleared_at_source_time: Option<time::OffsetDateTime>,
    #[doc = "source at which the alert can be raised"]
    pub source: AlertSource,
    #[doc = "Recommendation for acting on the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[doc = "Reason for resolving the alert"]
    #[serde(rename = "resolutionReason", default, skip_serializing_if = "Option::is_none")]
    pub resolution_reason: Option<String>,
    #[doc = "Severity of the alert"]
    pub severity: alert_properties::Severity,
    #[doc = "Current status of the alert"]
    pub status: alert_properties::Status,
    #[doc = "Error details for the alert"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<AlertErrorDetails>,
    #[doc = "Other information about the alert"]
    #[serde(rename = "detailedInformation", default, skip_serializing_if = "Option::is_none")]
    pub detailed_information: Option<serde_json::Value>,
}
impl AlertProperties {
    pub fn new(
        title: String,
        scope: alert_properties::Scope,
        alert_type: String,
        appeared_at_time: time::OffsetDateTime,
        appeared_at_source_time: time::OffsetDateTime,
        source: AlertSource,
        severity: alert_properties::Severity,
        status: alert_properties::Status,
    ) -> Self {
        Self {
            title,
            scope,
            alert_type,
            appeared_at_time,
            appeared_at_source_time,
            cleared_at_time: None,
            cleared_at_source_time: None,
            source,
            recommendation: None,
            resolution_reason: None,
            severity,
            status,
            error_details: None,
            detailed_information: None,
        }
    }
}
pub mod alert_properties {
    use super::*;
    #[doc = "Device or Resource alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        Resource,
        Device,
    }
    #[doc = "Severity of the alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Informational,
        Warning,
        Critical,
    }
    #[doc = "Current status of the alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Cleared,
    }
}
#[doc = "AlertSettings on the device which represents how alerts will be processed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertSettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Class containing the properties of AlertSettings"]
    pub properties: AlertSettingsProperties,
}
impl AlertSettings {
    pub fn new(properties: AlertSettingsProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Class containing the properties of AlertSettings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertSettingsProperties {
    #[doc = "Value indicating whether user/admins will receive emails when an alert condition occurs on the system"]
    #[serde(rename = "emailNotification")]
    pub email_notification: alert_settings_properties::EmailNotification,
    #[doc = "Value indicating whether service owners will receive emails when an alert condition occurs on the system. Applicable only if emailNotification flag is Enabled."]
    #[serde(rename = "notificationToServiceOwners")]
    pub notification_to_service_owners: alert_settings_properties::NotificationToServiceOwners,
    #[doc = "Culture setting to be used while building alert emails. For eg: \"en-US\""]
    #[serde(rename = "alertNotificationCulture")]
    pub alert_notification_culture: String,
    #[doc = "List of email addresses (apart from admin/co-admin of subscription) to whom the alert emails need to be sent"]
    #[serde(rename = "additionalRecipientEmailList", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_recipient_email_list: Vec<String>,
}
impl AlertSettingsProperties {
    pub fn new(
        email_notification: alert_settings_properties::EmailNotification,
        notification_to_service_owners: alert_settings_properties::NotificationToServiceOwners,
        alert_notification_culture: String,
    ) -> Self {
        Self {
            email_notification,
            notification_to_service_owners,
            alert_notification_culture,
            additional_recipient_email_list: Vec::new(),
        }
    }
}
pub mod alert_settings_properties {
    use super::*;
    #[doc = "Value indicating whether user/admins will receive emails when an alert condition occurs on the system"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EmailNotification {
        Enabled,
        Disabled,
    }
    #[doc = "Value indicating whether service owners will receive emails when an alert condition occurs on the system. Applicable only if emailNotification flag is Enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NotificationToServiceOwners {
        Enabled,
        Disabled,
    }
}
#[doc = "source at which the alert can be raised"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertSource {
    #[doc = "Name of the source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The time zone."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "Source type of the alert."]
    #[serde(rename = "alertSourceType", default, skip_serializing_if = "Option::is_none")]
    pub alert_source_type: Option<alert_source::AlertSourceType>,
}
impl AlertSource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_source {
    use super::*;
    #[doc = "Source type of the alert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AlertSourceType {
        Resource,
        Device,
    }
}
#[doc = "This class can be used as the Type for any secret entity represented as Password, CertThumbprint, Algorithm. This class is intended to be used when the secret is encrypted with an asymmetric key pair. The encryptionAlgorithm field is mainly for future usage to potentially allow different entities encrypted using different algorithms."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsymmetricEncryptedSecret {
    #[doc = "The value of the secret itself. If the secret is in plaintext then EncryptionAlgorithm will be none and EncryptionCertThumbprint will be null."]
    pub value: String,
    #[doc = "Thumbprint certificate that was used to encrypt \"Value\""]
    #[serde(rename = "encryptionCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub encryption_certificate_thumbprint: Option<String>,
    #[doc = "Algorithm used to encrypt \"Value\""]
    #[serde(rename = "encryptionAlgorithm")]
    pub encryption_algorithm: asymmetric_encrypted_secret::EncryptionAlgorithm,
}
impl AsymmetricEncryptedSecret {
    pub fn new(value: String, encryption_algorithm: asymmetric_encrypted_secret::EncryptionAlgorithm) -> Self {
        Self {
            value,
            encryption_certificate_thumbprint: None,
            encryption_algorithm,
        }
    }
}
pub mod asymmetric_encrypted_secret {
    use super::*;
    #[doc = "Algorithm used to encrypt \"Value\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionAlgorithm {
        None,
        #[serde(rename = "AES256")]
        Aes256,
        #[serde(rename = "RSAES_PKCS1_v_1_5")]
        RsaesPkcs1V15,
    }
}
#[doc = "Class represents provider operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperation {
    #[doc = "Gets or sets the name of the operation being performed on this particular object\r\nReturn value format: \"{resourceProviderNamespace}/{resourceType}/{read|write|deletion|action}\"\r\nEg: Microsoft.StorSimple/managers/devices/fileServers/read\r\n    Microsoft.StorSimple/managers/devices/alerts/clearAlerts/action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contains the localized display information for this particular operation / action. \r\nThese value will be used by several clients for \r\n(1) custom role definitions for RBAC; \r\n(2) complex query filters for the event service; and (3) audit history / records for management operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableProviderOperationDisplay>,
    #[doc = "Gets or sets Origin\r\nThe intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX.\r\nDefault value is “user,system”"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Class represents Properties in AvailableProviderOperations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableProviderOperationProperties>,
}
impl AvailableProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the localized display information for this particular operation / action. \r\nThese value will be used by several clients for \r\n(1) custom role definitions for RBAC; \r\n(2) complex query filters for the event service; and (3) audit history / records for management operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperationDisplay {
    #[doc = "Gets or sets Provider\r\nThe localized friendly form of the resource provider name – it is expected to also include the publisher/company responsible. \r\nIt should use Title Casing and begin with “Microsoft” for 1st party services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Gets or sets Resource\r\nThe localized friendly form of the resource type related to this action/operation – it should match the public documentation for the resource provider. \r\nIt should use Title Casing – for examples, please refer to the “name” section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Gets or sets Operation\r\nThe localized friendly name for the operation, as it should be shown to the user. \r\nIt should be concise (to fit in drop downs) but clear (i.e. self-documenting). It should use Title Casing and include the entity/resource to which it applies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Gets or sets Description\r\nThe localized friendly description for the operation, as it should be shown to the user. \r\nIt should be thorough, yet concise – it will be used in tool tips and detailed views."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AvailableProviderOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class represents Properties in AvailableProviderOperations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperationProperties {}
impl AvailableProviderOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for set of operations used for discovery of available provider operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableProviderOperations {
    #[doc = "The value."]
    pub value: Vec<AvailableProviderOperation>,
    #[doc = "The NextLink."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableProviderOperations {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailableProviderOperations {
    pub fn new(value: Vec<AvailableProviderOperation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Backup {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Class represents Backup properties"]
    pub properties: BackupProperties,
}
impl Backup {
    pub fn new(properties: BackupProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Class represents BackupElement"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupElement {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The backup element properties"]
    pub properties: BackupElementProperties,
}
impl BackupElement {
    pub fn new(properties: BackupElementProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The backup element properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupElementProperties {
    #[doc = "The size in bytes."]
    #[serde(rename = "sizeInBytes")]
    pub size_in_bytes: i64,
    #[doc = "The name of the endpoint."]
    #[serde(rename = "endpointName")]
    pub endpoint_name: String,
    #[doc = "The data policy of backed up endpoint."]
    #[serde(rename = "dataPolicy")]
    pub data_policy: backup_element_properties::DataPolicy,
}
impl BackupElementProperties {
    pub fn new(size_in_bytes: i64, endpoint_name: String, data_policy: backup_element_properties::DataPolicy) -> Self {
        Self {
            size_in_bytes,
            endpoint_name,
            data_policy,
        }
    }
}
pub mod backup_element_properties {
    use super::*;
    #[doc = "The data policy of backed up endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataPolicy {
        Invalid,
        Local,
        Tiered,
        Cloud,
    }
}
#[doc = "Backup OData filter class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupFilter {
    #[doc = "Gets or sets InitiatedBy"]
    #[serde(rename = "initiatedBy", default, skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<backup_filter::InitiatedBy>,
    #[doc = "Gets or sets CreatedTime"]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
}
impl BackupFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_filter {
    use super::*;
    #[doc = "Gets or sets InitiatedBy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InitiatedBy {
        Manual,
        Scheduled,
    }
}
#[doc = "Collection of backups"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupList {
    #[doc = "The value."]
    pub value: Vec<Backup>,
    #[doc = "The NextLink."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BackupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BackupList {
    pub fn new(value: Vec<Backup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class represents Backup properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupProperties {
    #[doc = "The path id of the target FileServer or IscsiServer for which the backup was taken."]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[doc = "Type of target, FileServer or IscsiServer"]
    #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[doc = "The backup size in bytes."]
    #[serde(rename = "sizeInBytes")]
    pub size_in_bytes: i64,
    #[doc = "The time when the backup was created."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the backup will expire."]
    #[serde(rename = "expirationTime", with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates how the backup was initiated \"Manual | Scheduled\"."]
    #[serde(rename = "initiatedBy")]
    pub initiated_by: backup_properties::InitiatedBy,
    #[doc = "The Device Identifier."]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "The backup elements."]
    pub elements: Vec<BackupElement>,
}
impl BackupProperties {
    pub fn new(size_in_bytes: i64, initiated_by: backup_properties::InitiatedBy, device_id: String, elements: Vec<BackupElement>) -> Self {
        Self {
            target_id: None,
            target_type: None,
            size_in_bytes,
            created_time: None,
            expiration_time: None,
            initiated_by,
            device_id,
            elements,
        }
    }
}
pub mod backup_properties {
    use super::*;
    #[doc = "Indicates how the backup was initiated \"Manual | Scheduled\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InitiatedBy {
        Manual,
        Scheduled,
    }
}
#[doc = "The Backup Schedule Group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupScheduleGroup {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The Backup Schedule Group Properties"]
    pub properties: BackupScheduleGroupProperties,
}
impl BackupScheduleGroup {
    pub fn new(properties: BackupScheduleGroupProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The list response of backup schedule groups"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupScheduleGroupList {
    #[doc = "The value."]
    pub value: Vec<BackupScheduleGroup>,
}
impl azure_core::Continuable for BackupScheduleGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BackupScheduleGroupList {
    pub fn new(value: Vec<BackupScheduleGroup>) -> Self {
        Self { value }
    }
}
#[doc = "The Backup Schedule Group Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupScheduleGroupProperties {
    #[doc = "The Time."]
    #[serde(rename = "startTime")]
    pub start_time: Time,
}
impl BackupScheduleGroupProperties {
    pub fn new(start_time: Time) -> Self {
        Self { start_time }
    }
}
#[doc = "Base class for models"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseModel {
    #[doc = "The identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl BaseModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Chap properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChapProperties {
    #[doc = "This class can be used as the Type for any secret entity represented as Password, CertThumbprint, Algorithm. This class is intended to be used when the secret is encrypted with an asymmetric key pair. The encryptionAlgorithm field is mainly for future usage to potentially allow different entities encrypted using different algorithms."]
    pub password: AsymmetricEncryptedSecret,
}
impl ChapProperties {
    pub fn new(password: AsymmetricEncryptedSecret) -> Self {
        Self { password }
    }
}
#[doc = "Challenge-Handshake Authentication Protocol (CHAP) setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChapSettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Chap properties"]
    pub properties: ChapProperties,
}
impl ChapSettings {
    pub fn new(properties: ChapProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Collection of Chap setting entities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChapSettingsList {
    #[doc = "The chap settings entity collection"]
    pub value: Vec<ChapSettings>,
}
impl azure_core::Continuable for ChapSettingsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ChapSettingsList {
    pub fn new(value: Vec<ChapSettings>) -> Self {
        Self { value }
    }
}
#[doc = "Request for clearing the alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearAlertRequest {
    #[doc = "Resolution message while clearing the request"]
    #[serde(rename = "resolutionMessage", default, skip_serializing_if = "Option::is_none")]
    pub resolution_message: Option<String>,
    #[doc = "List of alert Ids to be cleared"]
    pub alerts: Vec<String>,
}
impl ClearAlertRequest {
    pub fn new(alerts: Vec<String>) -> Self {
        Self {
            resolution_message: None,
            alerts,
        }
    }
}
#[doc = "Clone Job Request Model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloneRequest {
    #[doc = "Properties of CloneRequest"]
    pub properties: CloneRequestProperties,
}
impl CloneRequest {
    pub fn new(properties: CloneRequestProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Properties of CloneRequest"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloneRequestProperties {
    #[doc = "DeviceId of the device which will act as the Clone target"]
    #[serde(rename = "targetDeviceId")]
    pub target_device_id: String,
    #[doc = "Access point Id on which clone job will performed."]
    #[serde(rename = "targetAccessPointId")]
    pub target_access_point_id: String,
    #[doc = "Name of new endpoint which will created as part of clone job."]
    #[serde(rename = "newEndpointName")]
    pub new_endpoint_name: String,
    #[doc = "The File Share."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share: Option<FileShare>,
    #[doc = "The iSCSI disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk: Option<IscsiDisk>,
}
impl CloneRequestProperties {
    pub fn new(target_device_id: String, target_access_point_id: String, new_endpoint_name: String) -> Self {
        Self {
            target_device_id,
            target_access_point_id,
            new_endpoint_name,
            share: None,
            disk: None,
        }
    }
}
#[doc = "Represents a StorSimple device object along with its properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Device {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Encases all the properties of the Device"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeviceProperties>,
}
impl Device {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class containing more granular details about the device"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceDetails {
    #[doc = "Total number of endpoints that are currently on the device ( i.e. number of shares on FileServer or number of volumes on IscsiServer)"]
    #[serde(rename = "endpointCount", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_count: Option<i32>,
    #[doc = "Total storage available on the device in bytes."]
    #[serde(rename = "totalStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_storage_in_bytes: Option<i64>,
    #[doc = "Total local storage capacity in device in bytes."]
    #[serde(rename = "totalLocalStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_local_storage_in_bytes: Option<i64>,
    #[doc = "Storage in bytes that has been provisioned on the device including both local and cloud"]
    #[serde(rename = "provisionedStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_storage_in_bytes: Option<i64>,
    #[doc = "Storage in bytes that has been provisioned locally on the device"]
    #[serde(rename = "provisionedLocalStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_local_storage_in_bytes: Option<i64>,
    #[doc = "Storage that is being currently used in bytes including both local and cloud"]
    #[serde(rename = "usingStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub using_storage_in_bytes: Option<i64>,
    #[doc = "Local Storage that is being currently used in bytes"]
    #[serde(rename = "usingLocalStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub using_local_storage_in_bytes: Option<i64>,
    #[doc = "Total size taken up by backups in bytes"]
    #[serde(rename = "totalBackupSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_backup_size_in_bytes: Option<i64>,
    #[doc = "Total pending available storage on the device in bytes"]
    #[serde(rename = "availableStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_storage_in_bytes: Option<i64>,
    #[doc = "Local pending storage available on the device in bytes"]
    #[serde(rename = "availableLocalStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_local_storage_in_bytes: Option<i64>,
}
impl DeviceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Devices"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceList {
    #[doc = "The value."]
    pub value: Vec<Device>,
}
impl azure_core::Continuable for DeviceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DeviceList {
    pub fn new(value: Vec<Device>) -> Self {
        Self { value }
    }
}
#[doc = "Class that represents the Input for the PATCH call on Device. Currently the only patchable property on device is \"DeviceDescription\""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevicePatch {
    #[doc = "Short description given for the device"]
    #[serde(rename = "deviceDescription", default, skip_serializing_if = "Option::is_none")]
    pub device_description: Option<String>,
}
impl DevicePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encases all the properties of the Device"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceProperties {
    #[doc = "The UTC time at which the device was activated"]
    #[serde(rename = "activationTime", with = "azure_core::date::rfc3339::option")]
    pub activation_time: Option<time::OffsetDateTime>,
    #[doc = "Operations that are allowed on the device based on its current state"]
    #[serde(rename = "allowedDeviceOperations", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_device_operations: Vec<String>,
    #[doc = "Language culture setting on the device. For eg: \"en-US\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub culture: Option<String>,
    #[doc = "Device can be configured either as FileServer or IscsiServer"]
    #[serde(rename = "deviceCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub device_capabilities: Vec<String>,
    #[doc = "Short description given for the device"]
    #[serde(rename = "deviceDescription", default, skip_serializing_if = "Option::is_none")]
    pub device_description: Option<String>,
    #[doc = "Fully qualified name of the domain to which the device is attached"]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "Version number of the software running on the device"]
    #[serde(rename = "deviceSoftwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub device_software_version: Option<String>,
    #[doc = "Friendly name for the software version"]
    #[serde(rename = "friendlySoftwareName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_software_name: Option<String>,
    #[doc = "\"Complete\" if the device has been successfully registered as File/IscsiServer and the creation of share/volume is complete, \"Pending\" if the device is only registered but the creation of share/volume is complete is still pending"]
    #[serde(rename = "deviceConfigurationStatus", default, skip_serializing_if = "Option::is_none")]
    pub device_configuration_status: Option<device_properties::DeviceConfigurationStatus>,
    #[doc = "Name of the device model"]
    #[serde(rename = "modelDescription", default, skip_serializing_if = "Option::is_none")]
    pub model_description: Option<String>,
    #[doc = "Current status of the device"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<device_properties::Status>,
    #[doc = "Type of the device"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<device_properties::Type>,
    #[doc = "Class containing more granular details about the device"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<DeviceDetails>,
}
impl DeviceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod device_properties {
    use super::*;
    #[doc = "\"Complete\" if the device has been successfully registered as File/IscsiServer and the creation of share/volume is complete, \"Pending\" if the device is only registered but the creation of share/volume is complete is still pending"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceConfigurationStatus {
        Complete,
        Pending,
    }
    #[doc = "Current status of the device"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unknown,
        Online,
        Offline,
        RequiresAttention,
        MaintenanceMode,
        Creating,
        Provisioning,
        Deleted,
        ReadyToSetup,
        Deactivated,
        Deactivating,
    }
    #[doc = "Type of the device"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Invalid,
        Appliance,
        VirtualAppliance,
        Series9000OnPremVirtualAppliance,
        Series9000VirtualAppliance,
        Series9000PhysicalAppliance,
    }
}
#[doc = "The EncryptionSettings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of EncryptionSettings"]
    pub properties: EncryptionSettingsProperties,
}
impl EncryptionSettings {
    pub fn new(properties: EncryptionSettingsProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The properties of EncryptionSettings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettingsProperties {
    #[doc = "The encryption status which indicates if encryption is enabled or not."]
    #[serde(rename = "encryptionStatus")]
    pub encryption_status: encryption_settings_properties::EncryptionStatus,
    #[doc = "The key rollover status which indicates if key rollover is required or not. If secrets encryption has been upgraded, then it requires key rollover."]
    #[serde(rename = "keyRolloverStatus")]
    pub key_rollover_status: encryption_settings_properties::KeyRolloverStatus,
}
impl EncryptionSettingsProperties {
    pub fn new(
        encryption_status: encryption_settings_properties::EncryptionStatus,
        key_rollover_status: encryption_settings_properties::KeyRolloverStatus,
    ) -> Self {
        Self {
            encryption_status,
            key_rollover_status,
        }
    }
}
pub mod encryption_settings_properties {
    use super::*;
    #[doc = "The encryption status which indicates if encryption is enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionStatus {
        Enabled,
        Disabled,
    }
    #[doc = "The key rollover status which indicates if key rollover is required or not. If secrets encryption has been upgraded, then it requires key rollover."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyRolloverStatus {
        Required,
        NotRequired,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "errorCode")]
    pub error_code: String,
    pub message: Message,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<Item>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new(error_code: String, message: Message) -> Self {
        Self {
            error_code,
            message,
            values: Vec::new(),
        }
    }
}
#[doc = "The Failover request object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverRequest {
    #[serde(rename = "accesspointIds", default, skip_serializing_if = "Vec::is_empty")]
    pub accesspoint_ids: Vec<String>,
    #[serde(rename = "targetDeviceId", default, skip_serializing_if = "Option::is_none")]
    pub target_device_id: Option<String>,
    #[serde(rename = "skipValidation", default, skip_serializing_if = "Option::is_none")]
    pub skip_validation: Option<bool>,
    #[serde(rename = "keepSourceDevice", default, skip_serializing_if = "Option::is_none")]
    pub keep_source_device: Option<bool>,
}
impl FailoverRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The file server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileServer {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The file server properties."]
    pub properties: FileServerProperties,
}
impl FileServer {
    pub fn new(properties: FileServerProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Collection of file servers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileServerList {
    #[doc = "The value."]
    pub value: Vec<FileServer>,
}
impl azure_core::Continuable for FileServerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl FileServerList {
    pub fn new(value: Vec<FileServer>) -> Self {
        Self { value }
    }
}
#[doc = "The file server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileServerProperties {
    #[doc = "Domain of the file server"]
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[doc = "The storage domain id."]
    #[serde(rename = "storageDomainId")]
    pub storage_domain_id: String,
    #[doc = "The backup policy id."]
    #[serde(rename = "backupScheduleGroupId")]
    pub backup_schedule_group_id: String,
    #[doc = "The description of the file server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl FileServerProperties {
    pub fn new(domain_name: String, storage_domain_id: String, backup_schedule_group_id: String) -> Self {
        Self {
            domain_name,
            storage_domain_id,
            backup_schedule_group_id,
            description: None,
        }
    }
}
#[doc = "The File Share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileShare {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The File Share."]
    pub properties: FileShareProperties,
}
impl FileShare {
    pub fn new(properties: FileShareProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Collection of file shares"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileShareList {
    #[doc = "The value."]
    pub value: Vec<FileShare>,
}
impl azure_core::Continuable for FileShareList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl FileShareList {
    pub fn new(value: Vec<FileShare>) -> Self {
        Self { value }
    }
}
#[doc = "The File Share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileShareProperties {
    #[doc = "Description for file share"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Share Status"]
    #[serde(rename = "shareStatus")]
    pub share_status: file_share_properties::ShareStatus,
    #[doc = "The data policy"]
    #[serde(rename = "dataPolicy")]
    pub data_policy: file_share_properties::DataPolicy,
    #[doc = "The user/group who will have full permission in this share. Active directory email address. Example: xyz@contoso.com or Contoso\\xyz."]
    #[serde(rename = "adminUser")]
    pub admin_user: String,
    #[doc = "The total provisioned capacity in Bytes"]
    #[serde(rename = "provisionedCapacityInBytes")]
    pub provisioned_capacity_in_bytes: i64,
    #[doc = "The used capacity in Bytes."]
    #[serde(rename = "usedCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_capacity_in_bytes: Option<i64>,
    #[doc = "The local used capacity in Bytes."]
    #[serde(rename = "localUsedCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub local_used_capacity_in_bytes: Option<i64>,
    #[doc = "The monitoring status"]
    #[serde(rename = "monitoringStatus")]
    pub monitoring_status: file_share_properties::MonitoringStatus,
}
impl FileShareProperties {
    pub fn new(
        share_status: file_share_properties::ShareStatus,
        data_policy: file_share_properties::DataPolicy,
        admin_user: String,
        provisioned_capacity_in_bytes: i64,
        monitoring_status: file_share_properties::MonitoringStatus,
    ) -> Self {
        Self {
            description: None,
            share_status,
            data_policy,
            admin_user,
            provisioned_capacity_in_bytes,
            used_capacity_in_bytes: None,
            local_used_capacity_in_bytes: None,
            monitoring_status,
        }
    }
}
pub mod file_share_properties {
    use super::*;
    #[doc = "The Share Status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ShareStatus {
        Online,
        Offline,
    }
    #[doc = "The data policy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataPolicy {
        Invalid,
        Local,
        Tiered,
        Cloud,
    }
    #[doc = "The monitoring status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitoringStatus {
        Enabled,
        Disabled,
    }
}
#[doc = "Details related to the IP address configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpConfig {
    #[doc = "The IP address of the network adapter, either ipv4 or ipv6."]
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[doc = "The prefix length of the network adapter."]
    #[serde(rename = "prefixLength", default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
    #[doc = "The gateway of the network adapter."]
    pub gateway: String,
}
impl IpConfig {
    pub fn new(ip_address: String, gateway: String) -> Self {
        Self {
            ip_address,
            prefix_length: None,
            gateway,
        }
    }
}
#[doc = "The iSCSI disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiDisk {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The iSCSI disk properties."]
    pub properties: IscsiDiskProperties,
}
impl IscsiDisk {
    pub fn new(properties: IscsiDiskProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Collection of Iscsi disk"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiDiskList {
    #[doc = "The value."]
    pub value: Vec<IscsiDisk>,
}
impl azure_core::Continuable for IscsiDiskList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl IscsiDiskList {
    pub fn new(value: Vec<IscsiDisk>) -> Self {
        Self { value }
    }
}
#[doc = "The iSCSI disk properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiDiskProperties {
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The disk status."]
    #[serde(rename = "diskStatus")]
    pub disk_status: iscsi_disk_properties::DiskStatus,
    #[doc = "The access control records."]
    #[serde(rename = "accessControlRecords")]
    pub access_control_records: Vec<String>,
    #[doc = "The data policy."]
    #[serde(rename = "dataPolicy")]
    pub data_policy: iscsi_disk_properties::DataPolicy,
    #[doc = "The provisioned capacity in bytes."]
    #[serde(rename = "provisionedCapacityInBytes")]
    pub provisioned_capacity_in_bytes: i64,
    #[doc = "The used capacity in bytes."]
    #[serde(rename = "usedCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_capacity_in_bytes: Option<i64>,
    #[doc = "The local used capacity in bytes."]
    #[serde(rename = "localUsedCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub local_used_capacity_in_bytes: Option<i64>,
    #[doc = "The monitoring."]
    #[serde(rename = "monitoringStatus")]
    pub monitoring_status: iscsi_disk_properties::MonitoringStatus,
}
impl IscsiDiskProperties {
    pub fn new(
        disk_status: iscsi_disk_properties::DiskStatus,
        access_control_records: Vec<String>,
        data_policy: iscsi_disk_properties::DataPolicy,
        provisioned_capacity_in_bytes: i64,
        monitoring_status: iscsi_disk_properties::MonitoringStatus,
    ) -> Self {
        Self {
            description: None,
            disk_status,
            access_control_records,
            data_policy,
            provisioned_capacity_in_bytes,
            used_capacity_in_bytes: None,
            local_used_capacity_in_bytes: None,
            monitoring_status,
        }
    }
}
pub mod iscsi_disk_properties {
    use super::*;
    #[doc = "The disk status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DiskStatus {
        Online,
        Offline,
    }
    #[doc = "The data policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataPolicy {
        Invalid,
        Local,
        Tiered,
        Cloud,
    }
    #[doc = "The monitoring."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitoringStatus {
        Enabled,
        Disabled,
    }
}
#[doc = "The iSCSI server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiServer {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The iSCSI server properties."]
    pub properties: IscsiServerProperties,
}
impl IscsiServer {
    pub fn new(properties: IscsiServerProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Collection of Iscsi servers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiServerList {
    #[doc = "The value."]
    pub value: Vec<IscsiServer>,
}
impl azure_core::Continuable for IscsiServerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl IscsiServerList {
    pub fn new(value: Vec<IscsiServer>) -> Self {
        Self { value }
    }
}
#[doc = "The iSCSI server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiServerProperties {
    #[doc = "The storage domain id."]
    #[serde(rename = "storageDomainId")]
    pub storage_domain_id: String,
    #[doc = "The backup policy id."]
    #[serde(rename = "backupScheduleGroupId")]
    pub backup_schedule_group_id: String,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The chap id."]
    #[serde(rename = "chapId", default, skip_serializing_if = "Option::is_none")]
    pub chap_id: Option<String>,
    #[doc = "The reverse chap id."]
    #[serde(rename = "reverseChapId", default, skip_serializing_if = "Option::is_none")]
    pub reverse_chap_id: Option<String>,
}
impl IscsiServerProperties {
    pub fn new(storage_domain_id: String, backup_schedule_group_id: String) -> Self {
        Self {
            storage_domain_id,
            backup_schedule_group_id,
            description: None,
            chap_id: None,
            reverse_chap_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub key: String,
    pub value: String,
}
impl Item {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
#[doc = "The Job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Current status of the job"]
    pub status: job::Status,
    #[doc = "The UTC time at which the job was started"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC time at which the job completed"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The percentage of the job that is already complete"]
    #[serde(rename = "percentComplete")]
    pub percent_complete: i32,
    #[doc = "The job error information containing List of JobErrorItem."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JobErrorDetails>,
    #[doc = "properties for the job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl Job {
    pub fn new(status: job::Status, percent_complete: i32) -> Self {
        Self {
            base_model: BaseModel::default(),
            status,
            start_time: None,
            end_time: None,
            percent_complete,
            error: None,
            properties: None,
        }
    }
}
pub mod job {
    use super::*;
    #[doc = "Current status of the job"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Invalid,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Paused,
        Scheduled,
    }
}
#[doc = "The job error information containing List of JobErrorItem."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobErrorDetails {
    #[doc = "The error details."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<JobErrorItem>,
    #[doc = "The code intended for programmatic access"]
    pub code: String,
    #[doc = "The message intended to describe the error in detail"]
    pub message: String,
}
impl JobErrorDetails {
    pub fn new(code: String, message: String) -> Self {
        Self {
            error_details: Vec::new(),
            code,
            message,
        }
    }
}
#[doc = "The job error items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobErrorItem {
    #[doc = "The recommended actions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
    #[doc = "The code intended for programmatic access"]
    pub code: String,
    #[doc = "The message intended to describe the error in detail"]
    pub message: String,
}
impl JobErrorItem {
    pub fn new(code: String, message: String) -> Self {
        Self {
            recommendations: Vec::new(),
            code,
            message,
        }
    }
}
#[doc = "Filters that can be specified for the job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobFilter {
    #[doc = "The job type."]
    #[serde(rename = "jobType")]
    pub job_type: job_filter::JobType,
    #[doc = "The job status."]
    pub status: job_filter::Status,
    #[doc = "The start time of the job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl JobFilter {
    pub fn new(job_type: job_filter::JobType, status: job_filter::Status) -> Self {
        Self {
            job_type,
            status,
            start_time: None,
        }
    }
}
pub mod job_filter {
    use super::*;
    #[doc = "The job type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobType {
        Backup,
        Clone,
        Failover,
        DownloadUpdates,
        InstallUpdates,
    }
    #[doc = "The job status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Invalid,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Paused,
        Scheduled,
    }
}
#[doc = "Collection of jobs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobList {
    #[doc = "The value."]
    pub value: Vec<Job>,
    #[doc = "The NextLink."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobList {
    pub fn new(value: Vec<Job>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "properties for the job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[doc = "Type of the job"]
    #[serde(rename = "jobType")]
    pub job_type: job_properties::JobType,
    #[doc = "Id of the object that is created by the job"]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[doc = "The entity identifier for which the job ran."]
    #[serde(rename = "entityId", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[doc = "The entity type for which the job ran."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The job stages."]
    #[serde(rename = "jobStages", default, skip_serializing_if = "Vec::is_empty")]
    pub job_stages: Vec<JobStage>,
    #[doc = "The device id in which the job is currently running"]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Represents whether the job is cancellable or not"]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
    #[doc = "Stats that are available for all jobs in common"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<JobStats>,
    #[doc = "The target type of the backup."]
    #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<job_properties::TargetType>,
    #[doc = "The source device identifier of the failover job."]
    #[serde(rename = "sourceDeviceId", default, skip_serializing_if = "Option::is_none")]
    pub source_device_id: Option<String>,
    #[doc = "The time of the backup used for the failover."]
    #[serde(rename = "backupPointInTime", with = "azure_core::date::rfc3339::option")]
    pub backup_point_in_time: Option<time::OffsetDateTime>,
    #[doc = "details available during the download"]
    #[serde(rename = "downloadProgress", default, skip_serializing_if = "Option::is_none")]
    pub download_progress: Option<UpdateDownloadProgress>,
    #[doc = "Class representing the progress during installation of updates"]
    #[serde(rename = "installProgress", default, skip_serializing_if = "Option::is_none")]
    pub install_progress: Option<UpdateInstallProgress>,
}
impl JobProperties {
    pub fn new(job_type: job_properties::JobType) -> Self {
        Self {
            job_type,
            target_id: None,
            entity_id: None,
            entity_type: None,
            job_stages: Vec::new(),
            device_id: None,
            is_cancellable: None,
            stats: None,
            target_type: None,
            source_device_id: None,
            backup_point_in_time: None,
            download_progress: None,
            install_progress: None,
        }
    }
}
pub mod job_properties {
    use super::*;
    #[doc = "Type of the job"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobType {
        Backup,
        Clone,
        Failover,
        DownloadUpdates,
        InstallUpdates,
    }
    #[doc = "The target type of the backup."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetType {
        FileServer,
        DiskServer,
    }
}
#[doc = "Contains details about the multiple job stages of a job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStage {
    #[doc = "The message of the job stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The stage status."]
    #[serde(rename = "stageStatus")]
    pub stage_status: job_stage::StageStatus,
    #[doc = "The details of the stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[doc = "The error code of the stage if any."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}
impl JobStage {
    pub fn new(stage_status: job_stage::StageStatus) -> Self {
        Self {
            message: None,
            stage_status,
            detail: None,
            error_code: None,
        }
    }
}
pub mod job_stage {
    use super::*;
    #[doc = "The stage status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageStatus {
        Invalid,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Paused,
        Scheduled,
    }
}
#[doc = "Stats that are available for all jobs in common"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStats {
    #[doc = "Completed work item count."]
    #[serde(rename = "completedWorkItemCount", default, skip_serializing_if = "Option::is_none")]
    pub completed_work_item_count: Option<i32>,
    #[doc = "Total work item count."]
    #[serde(rename = "totalWorkItemCount", default, skip_serializing_if = "Option::is_none")]
    pub total_work_item_count: Option<i32>,
    #[doc = "The estimated time remaining."]
    #[serde(rename = "estimatedTimeRemaining", default, skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining: Option<i32>,
}
impl JobStats {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The StorSimple Manager"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manager {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the Manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagerProperties>,
    #[doc = "ETag of the Manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Manager {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            etag: None,
        }
    }
}
#[doc = "The extended info of the manager."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagerExtendedInfo {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Properties of the ManagerExtendedInfo"]
    pub properties: ManagerExtendedInfoProperties,
    #[doc = "ETag of the Resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ManagerExtendedInfo {
    pub fn new(properties: ManagerExtendedInfoProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
            etag: None,
        }
    }
}
#[doc = "Properties of the ManagerExtendedInfo"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagerExtendedInfoProperties {
    #[doc = "Represents the version of the ExtendedInfo object being persisted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Represents the CIK of the resource"]
    #[serde(rename = "integrityKey")]
    pub integrity_key: String,
    #[doc = "Represents the CEK of the resource"]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[doc = "Represents the Cert thumbprint that was used to encrypt the CEK"]
    #[serde(rename = "encryptionKeyThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_thumbprint: Option<String>,
    #[doc = "Represents the portal thumbprint which can be used optionally to encrypt the entire data before storing it."]
    #[serde(rename = "portalCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub portal_certificate_thumbprint: Option<String>,
    #[doc = "Represents the encryption algorithm used to encrypt the other keys. None - if EncryptionKey is saved in plain text format. AlgorithmName - if encryption is used"]
    pub algorithm: String,
}
impl ManagerExtendedInfoProperties {
    pub fn new(integrity_key: String, algorithm: String) -> Self {
        Self {
            version: None,
            integrity_key,
            encryption_key: None,
            encryption_key_thumbprint: None,
            portal_certificate_thumbprint: None,
            algorithm,
        }
    }
}
#[doc = "Intrinsic settings which refers to the type of the StorSimple manager"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagerIntrinsicSettings {
    #[doc = "Refers to the type of the StorSimple Manager"]
    #[serde(rename = "type")]
    pub type_: manager_intrinsic_settings::Type,
}
impl ManagerIntrinsicSettings {
    pub fn new(type_: manager_intrinsic_settings::Type) -> Self {
        Self { type_ }
    }
}
pub mod manager_intrinsic_settings {
    use super::*;
    #[doc = "Refers to the type of the StorSimple Manager"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        GardaV1,
        HelsinkiV1,
    }
}
#[doc = "List of StorSimple Managers under a particular resourceGroup"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagerList {
    #[doc = "List of Managers"]
    pub value: Vec<Manager>,
}
impl azure_core::Continuable for ManagerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ManagerList {
    pub fn new(value: Vec<Manager>) -> Self {
        Self { value }
    }
}
#[doc = "The StorSimple Manager patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagerPatch {
    #[doc = "The tags attached to the StorSimple Manager."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagerPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Manager"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagerProperties {
    #[doc = "Intrinsic settings which refers to the type of the StorSimple manager"]
    #[serde(rename = "cisIntrinsicSettings", default, skip_serializing_if = "Option::is_none")]
    pub cis_intrinsic_settings: Option<ManagerIntrinsicSettings>,
    #[doc = "The Sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ManagerSku>,
    #[doc = "Specifies the state of the resource as it is getting provisioned. Value of \"Succeeded\" means the Manager was successfully created"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ManagerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagerSku {
    #[doc = "Refers to the sku name which should be \"Standard\""]
    pub name: manager_sku::Name,
}
impl ManagerSku {
    pub fn new(name: manager_sku::Name) -> Self {
        Self { name }
    }
}
pub mod manager_sku {
    use super::*;
    #[doc = "Refers to the sku name which should be \"Standard\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Message {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Message {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric availability specifies the time grain (aggregation interval or frequency) and the retention period for that time grain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAvailablity {
    #[doc = "The time grain, specifies the aggregation interval for the metric."]
    #[serde(rename = "timeGrain")]
    pub time_grain: String,
    #[doc = "The retention period for the metric at the specified timegrain"]
    pub retention: String,
}
impl MetricAvailablity {
    pub fn new(time_grain: String, retention: String) -> Self {
        Self { time_grain, retention }
    }
}
#[doc = "The metric data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricData {
    #[doc = "The time when the metric data is fetched"]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339")]
    pub time_stamp: time::OffsetDateTime,
    #[doc = "The sum of all the samples available in the given instance of time for the specific metric data"]
    pub sum: f64,
    #[doc = "The count of samples of the metric data available for the given instance"]
    pub count: i32,
    #[doc = "The average of all sample for the given instance"]
    pub average: f64,
    #[doc = "The minimum of all samples available"]
    pub minimum: f64,
    #[doc = "The maximum of all samples available"]
    pub maximum: f64,
}
impl MetricData {
    pub fn new(time_stamp: time::OffsetDateTime, sum: f64, count: i32, average: f64, minimum: f64, maximum: f64) -> Self {
        Self {
            time_stamp,
            sum,
            count,
            average,
            minimum,
            maximum,
        }
    }
}
#[doc = "Monitoring metric definition represents the metadata of the metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinition {
    #[doc = "The name of the metric"]
    pub name: MetricName,
    #[doc = "The metric unit"]
    pub unit: metric_definition::Unit,
    #[doc = "The metric aggregation type"]
    #[serde(rename = "primaryAggregationType")]
    pub primary_aggregation_type: metric_definition::PrimaryAggregationType,
    #[doc = "The metric source id"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The available metric granularities"]
    #[serde(rename = "metricAvailabilities")]
    pub metric_availabilities: Vec<MetricAvailablity>,
    #[doc = "The supported dimensions"]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "The metric definition type"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl MetricDefinition {
    pub fn new(
        name: MetricName,
        unit: metric_definition::Unit,
        primary_aggregation_type: metric_definition::PrimaryAggregationType,
        resource_id: String,
        metric_availabilities: Vec<MetricAvailablity>,
        dimensions: Vec<MetricDimension>,
        type_: String,
    ) -> Self {
        Self {
            name,
            unit,
            primary_aggregation_type,
            resource_id,
            metric_availabilities,
            dimensions,
            type_,
        }
    }
}
pub mod metric_definition {
    use super::*;
    #[doc = "The metric unit"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Bytes,
        BytesPerSecond,
        Count,
        CountPerSecond,
        Percent,
        Seconds,
    }
    #[doc = "The metric aggregation type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrimaryAggregationType {
        Average,
        Last,
        Maximum,
        Minimum,
        None,
        Total,
    }
}
#[doc = "List of metric definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinitionList {
    #[doc = "The list of metric definition"]
    pub value: Vec<MetricDefinition>,
}
impl azure_core::Continuable for MetricDefinitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricDefinitionList {
    pub fn new(value: Vec<MetricDefinition>) -> Self {
        Self { value }
    }
}
#[doc = "Metric dimension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    #[doc = "Metric dimension name"]
    pub name: String,
    #[doc = "Metric dimension values"]
    pub value: String,
}
impl MetricDimension {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "class representing the filters to be passed while fetching metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricFilter {
    #[doc = "Class representing the name filter to be passed while fetching metrics"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricNameFilter>,
    #[doc = "starttime for fetching metrics"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "endtime for fetching metrics"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "timegrain of the metrics"]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
}
impl MetricFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricList {
    #[doc = "The value."]
    pub value: Vec<Metrics>,
}
impl azure_core::Continuable for MetricList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricList {
    pub fn new(value: Vec<Metrics>) -> Self {
        Self { value }
    }
}
#[doc = "The name of the metric"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricName {
    #[doc = "Name of the metric"]
    pub value: String,
    #[doc = "Localized name of the metric"]
    #[serde(rename = "localizedValue")]
    pub localized_value: String,
}
impl MetricName {
    pub fn new(value: String, localized_value: String) -> Self {
        Self { value, localized_value }
    }
}
#[doc = "Class representing the name filter to be passed while fetching metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricNameFilter {
    #[doc = "The value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl MetricNameFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Monitoring metric"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[doc = "The id of metric source"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The metric start time"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "The metric end time"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
    #[doc = "The time grain, time grain indicates frequency of the metric data"]
    #[serde(rename = "timeGrain")]
    pub time_grain: String,
    #[doc = "The metric aggregation type"]
    #[serde(rename = "primaryAggregation")]
    pub primary_aggregation: metrics::PrimaryAggregation,
    #[doc = "The name of the metric"]
    pub name: MetricName,
    #[doc = "The Metric dimension which indicates the source of the metric"]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "The unit of the metric data"]
    pub unit: metrics::Unit,
    #[doc = "The Type of the metric data"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The metric data"]
    pub values: Vec<MetricData>,
}
impl Metrics {
    pub fn new(
        resource_id: String,
        start_time: time::OffsetDateTime,
        end_time: time::OffsetDateTime,
        time_grain: String,
        primary_aggregation: metrics::PrimaryAggregation,
        name: MetricName,
        dimensions: Vec<MetricDimension>,
        unit: metrics::Unit,
        type_: String,
        values: Vec<MetricData>,
    ) -> Self {
        Self {
            resource_id,
            start_time,
            end_time,
            time_grain,
            primary_aggregation,
            name,
            dimensions,
            unit,
            type_,
            values,
        }
    }
}
pub mod metrics {
    use super::*;
    #[doc = "The metric aggregation type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrimaryAggregation {
        Average,
        Last,
        Maximum,
        Minimum,
        None,
        Total,
    }
    #[doc = "The unit of the metric data"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Bytes,
        BytesPerSecond,
        Count,
        CountPerSecond,
        Percent,
        Seconds,
    }
}
#[doc = "Represents a networkAdapter in a particular node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkAdapter {
    #[doc = "The name of the network adapter."]
    #[serde(rename = "networkAdapterName")]
    pub network_adapter_name: String,
    #[doc = "Details related to the IP address configuration"]
    #[serde(rename = "iPv4Info", default, skip_serializing_if = "Option::is_none")]
    pub i_pv4_info: Option<IpConfig>,
    #[doc = "Details related to the IP address configuration"]
    #[serde(rename = "iPv6Info", default, skip_serializing_if = "Option::is_none")]
    pub i_pv6_info: Option<IpConfig>,
    #[doc = "Represents state of DHCP."]
    #[serde(rename = "dhcpStatus")]
    pub dhcp_status: network_adapter::DhcpStatus,
    #[doc = "The speed of the network adapter."]
    #[serde(rename = "linkSpeed", default, skip_serializing_if = "Option::is_none")]
    pub link_speed: Option<i64>,
}
impl NetworkAdapter {
    pub fn new(network_adapter_name: String, dhcp_status: network_adapter::DhcpStatus) -> Self {
        Self {
            network_adapter_name,
            i_pv4_info: None,
            i_pv6_info: None,
            dhcp_status,
            link_speed: None,
        }
    }
}
pub mod network_adapter {
    use super::*;
    #[doc = "Represents state of DHCP."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DhcpStatus {
        Enabled,
        Disabled,
    }
}
#[doc = "The NetworkSettings of a device"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of NetworkSettings"]
    pub properties: NetworkSettingsProperties,
}
impl NetworkSettings {
    pub fn new(properties: NetworkSettingsProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The properties of NetworkSettings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSettingsProperties {
    #[doc = "The Primary DNS server for the device"]
    #[serde(rename = "primaryDnsServer")]
    pub primary_dns_server: String,
    #[doc = "The secondary DNS server for the device"]
    #[serde(rename = "secondaryDnsServer", default, skip_serializing_if = "Option::is_none")]
    pub secondary_dns_server: Option<String>,
    #[doc = "The NetworkAdapters under each node of the device."]
    #[serde(rename = "nodeNetworks")]
    pub node_networks: Vec<NodeNetwork>,
}
impl NetworkSettingsProperties {
    pub fn new(primary_dns_server: String, node_networks: Vec<NodeNetwork>) -> Self {
        Self {
            primary_dns_server,
            secondary_dns_server: None,
            node_networks,
        }
    }
}
#[doc = "Represents a single node in a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeNetwork {
    #[doc = "The array of network adapters in the node."]
    #[serde(rename = "networkAdapters")]
    pub network_adapters: Vec<NetworkAdapter>,
}
impl NodeNetwork {
    pub fn new(network_adapters: Vec<NetworkAdapter>) -> Self {
        Self { network_adapters }
    }
}
#[doc = "Raw Certificate Data From IDM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawCertificateData {
    #[doc = "Specify the Authentication type"]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<raw_certificate_data::AuthType>,
    #[doc = "Gets or sets the base64 encoded certificate raw data string"]
    pub certificate: String,
}
impl RawCertificateData {
    pub fn new(certificate: String) -> Self {
        Self {
            auth_type: None,
            certificate,
        }
    }
}
pub mod raw_certificate_data {
    use super::*;
    #[doc = "Specify the Authentication type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthType {
        Invalid,
        AccessControlService,
        AzureActiveDirectory,
    }
}
#[doc = "The Azure Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The Geo location of the Manager"]
    pub location: String,
    #[doc = "Tags attached to the Manager"]
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
#[doc = "Resource Certificate And AAD Details from IDM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateAndAadDetails {
    #[doc = "Specify the Authentication type"]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<resource_certificate_and_aad_details::AuthType>,
    #[doc = "Gets or sets the base64 encoded certificate raw data string"]
    pub certificate: String,
    #[doc = "Gets or Sets the ResourceId"]
    #[serde(rename = "resourceId")]
    pub resource_id: i64,
    #[doc = "AAD tenant authority"]
    #[serde(rename = "aadAuthority")]
    pub aad_authority: String,
    #[doc = "AAD tenant Id"]
    #[serde(rename = "aadTenantId")]
    pub aad_tenant_id: String,
    #[doc = "AAD service principal clientId"]
    #[serde(rename = "servicePrincipalClientId")]
    pub service_principal_client_id: String,
    #[doc = "AAD service principal ObjectId"]
    #[serde(rename = "servicePrincipalObjectId")]
    pub service_principal_object_id: String,
    #[doc = "Azure Management Endpoint Audience"]
    #[serde(rename = "azureManagementEndpointAudience")]
    pub azure_management_endpoint_audience: String,
    #[doc = "Certificate Subject Name"]
    pub subject: String,
    #[doc = "Certificate Validity start Date time"]
    #[serde(rename = "validFrom", with = "azure_core::date::rfc3339")]
    pub valid_from: time::OffsetDateTime,
    #[doc = "Certificate Validity End Date time"]
    #[serde(rename = "validTo", with = "azure_core::date::rfc3339")]
    pub valid_to: time::OffsetDateTime,
    #[doc = "Certificate thumbprint"]
    pub thumbprint: String,
    #[doc = "Certificate friendly name"]
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[doc = "Certificate issuer"]
    pub issuer: String,
}
impl ResourceCertificateAndAadDetails {
    pub fn new(
        certificate: String,
        resource_id: i64,
        aad_authority: String,
        aad_tenant_id: String,
        service_principal_client_id: String,
        service_principal_object_id: String,
        azure_management_endpoint_audience: String,
        subject: String,
        valid_from: time::OffsetDateTime,
        valid_to: time::OffsetDateTime,
        thumbprint: String,
        friendly_name: String,
        issuer: String,
    ) -> Self {
        Self {
            auth_type: None,
            certificate,
            resource_id,
            aad_authority,
            aad_tenant_id,
            service_principal_client_id,
            service_principal_object_id,
            azure_management_endpoint_audience,
            subject,
            valid_from,
            valid_to,
            thumbprint,
            friendly_name,
            issuer,
        }
    }
}
pub mod resource_certificate_and_aad_details {
    use super::*;
    #[doc = "Specify the Authentication type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthType {
        Invalid,
        AccessControlService,
        AzureActiveDirectory,
    }
}
#[doc = "The SecuritySettings of a device"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of SecuritySettings"]
    pub properties: SecuritySettingsProperties,
}
impl SecuritySettings {
    pub fn new(properties: SecuritySettingsProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The properties of SecuritySettings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySettingsProperties {
    #[doc = "This class can be used as the Type for any secret entity represented as Password, CertThumbprint, Algorithm. This class is intended to be used when the secret is encrypted with an asymmetric key pair. The encryptionAlgorithm field is mainly for future usage to potentially allow different entities encrypted using different algorithms."]
    #[serde(rename = "deviceAdminPassword")]
    pub device_admin_password: AsymmetricEncryptedSecret,
}
impl SecuritySettingsProperties {
    pub fn new(device_admin_password: AsymmetricEncryptedSecret) -> Self {
        Self { device_admin_password }
    }
}
#[doc = "Request for sending test alert email"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendTestAlertEmailRequest {
    #[doc = "List of emails to send the test alerts"]
    #[serde(rename = "emailList")]
    pub email_list: Vec<String>,
}
impl SendTestAlertEmailRequest {
    pub fn new(email_list: Vec<String>) -> Self {
        Self { email_list }
    }
}
#[doc = "The storage account credential"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCredential {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Storage account properties"]
    pub properties: StorageAccountCredentialProperties,
}
impl StorageAccountCredential {
    pub fn new(properties: StorageAccountCredentialProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Collection of Storage account credential entities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCredentialList {
    #[doc = "The value."]
    pub value: Vec<StorageAccountCredential>,
}
impl azure_core::Continuable for StorageAccountCredentialList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl StorageAccountCredentialList {
    pub fn new(value: Vec<StorageAccountCredential>) -> Self {
        Self { value }
    }
}
#[doc = "Storage account properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCredentialProperties {
    #[doc = "The cloud service provider"]
    #[serde(rename = "cloudType")]
    pub cloud_type: storage_account_credential_properties::CloudType,
    #[doc = "The storage endpoint"]
    #[serde(rename = "endPoint")]
    pub end_point: String,
    #[doc = "The storage account login"]
    pub login: String,
    #[doc = "The storage account's geo location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "SSL needs to be enabled or not"]
    #[serde(rename = "enableSSL")]
    pub enable_ssl: storage_account_credential_properties::EnableSsl,
    #[doc = "This class can be used as the Type for any secret entity represented as Password, CertThumbprint, Algorithm. This class is intended to be used when the secret is encrypted with an asymmetric key pair. The encryptionAlgorithm field is mainly for future usage to potentially allow different entities encrypted using different algorithms."]
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<AsymmetricEncryptedSecret>,
}
impl StorageAccountCredentialProperties {
    pub fn new(
        cloud_type: storage_account_credential_properties::CloudType,
        end_point: String,
        login: String,
        enable_ssl: storage_account_credential_properties::EnableSsl,
    ) -> Self {
        Self {
            cloud_type,
            end_point,
            login,
            location: None,
            enable_ssl,
            access_key: None,
        }
    }
}
pub mod storage_account_credential_properties {
    use super::*;
    #[doc = "The cloud service provider"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CloudType {
        Azure,
        S3,
        #[serde(rename = "S3_RRS")]
        S3Rrs,
        OpenStack,
        #[serde(rename = "HP")]
        Hp,
    }
    #[doc = "SSL needs to be enabled or not"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EnableSsl {
        Enabled,
        Disabled,
    }
}
#[doc = "The storage domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageDomain {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The storage domain properties."]
    pub properties: StorageDomainProperties,
}
impl StorageDomain {
    pub fn new(properties: StorageDomainProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "Collection of storage domains"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageDomainList {
    #[doc = "The value."]
    pub value: Vec<StorageDomain>,
}
impl azure_core::Continuable for StorageDomainList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl StorageDomainList {
    pub fn new(value: Vec<StorageDomain>) -> Self {
        Self { value }
    }
}
#[doc = "The storage domain properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageDomainProperties {
    #[doc = "The storage account credentials."]
    #[serde(rename = "storageAccountCredentialIds")]
    pub storage_account_credential_ids: Vec<String>,
    #[doc = "This class can be used as the Type for any secret entity represented as Password, CertThumbprint, Algorithm. This class is intended to be used when the secret is encrypted with an asymmetric key pair. The encryptionAlgorithm field is mainly for future usage to potentially allow different entities encrypted using different algorithms."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<AsymmetricEncryptedSecret>,
    #[doc = "The encryption status \"Enabled | Disabled\"."]
    #[serde(rename = "encryptionStatus")]
    pub encryption_status: storage_domain_properties::EncryptionStatus,
}
impl StorageDomainProperties {
    pub fn new(storage_account_credential_ids: Vec<String>, encryption_status: storage_domain_properties::EncryptionStatus) -> Self {
        Self {
            storage_account_credential_ids,
            encryption_key: None,
            encryption_status,
        }
    }
}
pub mod storage_domain_properties {
    use super::*;
    #[doc = "The encryption status \"Enabled | Disabled\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionStatus {
        Enabled,
        Disabled,
    }
}
#[doc = "This class can be used as the Type for any secret entity represented as Value, ValueCertificateThumbprint, EncryptionAlgorithm. In this case, \"Value\" is a secret and the \"valueThumbprint\" represents the certificate thumbprint of the value. The algorithm field is mainly for future usage to potentially allow different entities encrypted using different algorithms."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymmetricEncryptedSecret {
    #[doc = "The value of the secret itself. If the secret is in plaintext or null then EncryptionAlgorithm will be none"]
    pub value: String,
    #[doc = "Thumbprint cert that was used to encrypt \"Value\""]
    #[serde(rename = "valueCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub value_certificate_thumbprint: Option<String>,
    #[doc = "Algorithm used to encrypt \"Value\""]
    #[serde(rename = "encryptionAlgorithm")]
    pub encryption_algorithm: symmetric_encrypted_secret::EncryptionAlgorithm,
}
impl SymmetricEncryptedSecret {
    pub fn new(value: String, encryption_algorithm: symmetric_encrypted_secret::EncryptionAlgorithm) -> Self {
        Self {
            value,
            value_certificate_thumbprint: None,
            encryption_algorithm,
        }
    }
}
pub mod symmetric_encrypted_secret {
    use super::*;
    #[doc = "Algorithm used to encrypt \"Value\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionAlgorithm {
        None,
        #[serde(rename = "AES256")]
        Aes256,
        #[serde(rename = "RSAES_PKCS1_v_1_5")]
        RsaesPkcs1V15,
    }
}
#[doc = "The Time."]
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
#[doc = "The TimeSettings of a device"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of TimeSettings"]
    pub properties: TimeSettingsProperties,
}
impl TimeSettings {
    pub fn new(properties: TimeSettingsProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The properties of TimeSettings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSettingsProperties {
    #[doc = "The timezone of device, like '(UTC -06:00) Central America'"]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[doc = "The primary Network Time Protocol (NTP) server name, like 'time.windows.com'."]
    #[serde(rename = "primaryTimeServer")]
    pub primary_time_server: String,
    #[doc = "The secondary Network Time Protocol (NTP) server name, like 'time.contoso.com'. It's optional."]
    #[serde(rename = "secondaryTimeServer", default, skip_serializing_if = "Option::is_none")]
    pub secondary_time_server: Option<String>,
}
impl TimeSettingsProperties {
    pub fn new(time_zone: String, primary_time_server: String) -> Self {
        Self {
            time_zone,
            primary_time_server,
            secondary_time_server: None,
        }
    }
}
#[doc = "details available during the download"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDownloadProgress {
    #[doc = "The download phase."]
    #[serde(rename = "downloadPhase", default, skip_serializing_if = "Option::is_none")]
    pub download_phase: Option<update_download_progress::DownloadPhase>,
    #[doc = "Percentage of completion."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "Total bytes to download."]
    #[serde(rename = "totalBytesToDownload", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes_to_download: Option<f64>,
    #[doc = "Total bytes downloaded."]
    #[serde(rename = "totalBytesDownloaded", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes_downloaded: Option<f64>,
    #[doc = "Number of updates to download."]
    #[serde(rename = "numberOfUpdatesToDownload", default, skip_serializing_if = "Option::is_none")]
    pub number_of_updates_to_download: Option<i32>,
    #[doc = "Number of updates downloaded."]
    #[serde(rename = "numberOfUpdatesDownloaded", default, skip_serializing_if = "Option::is_none")]
    pub number_of_updates_downloaded: Option<i32>,
}
impl UpdateDownloadProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_download_progress {
    use super::*;
    #[doc = "The download phase."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DownloadPhase {
        Unknown,
        Initializing,
        Downloading,
        Verifying,
    }
}
#[doc = "Class representing the progress during installation of updates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateInstallProgress {
    #[doc = "Percentage of completion."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "Number of updates to install."]
    #[serde(rename = "numberOfUpdatesToInstall", default, skip_serializing_if = "Option::is_none")]
    pub number_of_updates_to_install: Option<i32>,
    #[doc = "Number of updates installed."]
    #[serde(rename = "numberOfUpdatesInstalled", default, skip_serializing_if = "Option::is_none")]
    pub number_of_updates_installed: Option<i32>,
}
impl UpdateInstallProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updates profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Updates {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Properties of the update profile"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdatesProperties>,
}
impl Updates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the update profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdatesProperties {
    #[doc = "The current Device version."]
    #[serde(rename = "deviceVersion", default, skip_serializing_if = "Option::is_none")]
    pub device_version: Option<String>,
    #[doc = "The last time when the device did an update scan."]
    #[serde(rename = "deviceLastScannedTime", with = "azure_core::date::rfc3339::option")]
    pub device_last_scanned_time: Option<time::OffsetDateTime>,
    #[doc = "Set to true if regular updates were detected for the current version of the device."]
    #[serde(rename = "regularUpdatesAvailable", default, skip_serializing_if = "Option::is_none")]
    pub regular_updates_available: Option<bool>,
    #[doc = "Set to true if RegularUpdatesAvailable is true and if at least one of the updateItems detected has needs a reboot to install."]
    #[serde(rename = "rebootRequiredForInstall", default, skip_serializing_if = "Option::is_none")]
    pub reboot_required_for_install: Option<bool>,
    #[doc = "The total number of items pending for download."]
    #[serde(rename = "totalItemsPendingForDownload", default, skip_serializing_if = "Option::is_none")]
    pub total_items_pending_for_download: Option<i32>,
    #[doc = "The total number of items pending for install."]
    #[serde(rename = "totalItemsPendingForInstall", default, skip_serializing_if = "Option::is_none")]
    pub total_items_pending_for_install: Option<i32>,
    #[doc = "The current update operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<updates_properties::Status>,
    #[doc = "The time when the last scan job was completed (success|cancelled|failed) on the device."]
    #[serde(rename = "lastCompletedScanTime", with = "azure_core::date::rfc3339::option")]
    pub last_completed_scan_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the last Download job was completed (success|cancelled|failed) on the device."]
    #[serde(rename = "lastCompletedDownloadJobTime", with = "azure_core::date::rfc3339::option")]
    pub last_completed_download_job_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the last Install job was completed (success|cancelled|failed) on the device."]
    #[serde(rename = "lastCompletedInstallJobTime", with = "azure_core::date::rfc3339::option")]
    pub last_completed_install_job_time: Option<time::OffsetDateTime>,
    #[doc = "If a download is in progress, this field contains the JobId of that particular download job"]
    #[serde(rename = "inProgressDownloadJobId", default, skip_serializing_if = "Option::is_none")]
    pub in_progress_download_job_id: Option<String>,
    #[doc = "If an install is in progress, this field contains the JobId of that particular install job"]
    #[serde(rename = "inProgressInstallJobId", default, skip_serializing_if = "Option::is_none")]
    pub in_progress_install_job_id: Option<String>,
    #[doc = "The time when the currently running scan (if any) started"]
    #[serde(rename = "inProgressScanStartedTime", with = "azure_core::date::rfc3339::option")]
    pub in_progress_scan_started_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the currently running download (if any) started"]
    #[serde(rename = "inProgressDownloadJobStartedTime", with = "azure_core::date::rfc3339::option")]
    pub in_progress_download_job_started_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the currently running install (if any) started"]
    #[serde(rename = "inProgressInstallJobStartedTime", with = "azure_core::date::rfc3339::option")]
    pub in_progress_install_job_started_time: Option<time::OffsetDateTime>,
}
impl UpdatesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod updates_properties {
    use super::*;
    #[doc = "The current update operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Idle,
        Scanning,
        Downloading,
        Installing,
    }
}
#[doc = "Upload Certificate Request to IDM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadCertificateRequest {
    #[doc = "Raw Certificate Data From IDM"]
    pub properties: RawCertificateData,
    #[doc = "Gets ContractVersion"]
    #[serde(rename = "contractVersion", default, skip_serializing_if = "Option::is_none")]
    pub contract_version: Option<upload_certificate_request::ContractVersion>,
}
impl UploadCertificateRequest {
    pub fn new(properties: RawCertificateData) -> Self {
        Self {
            properties,
            contract_version: None,
        }
    }
}
pub mod upload_certificate_request {
    use super::*;
    #[doc = "Gets ContractVersion"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ContractVersion {
        InvalidVersion,
        #[serde(rename = "V2011_09")]
        V201109,
        #[serde(rename = "V2012_02")]
        V201202,
        #[serde(rename = "V2012_05")]
        V201205,
        #[serde(rename = "V2012_12")]
        V201212,
        #[serde(rename = "V2013_04")]
        V201304,
        #[serde(rename = "V2013_10")]
        V201310,
        #[serde(rename = "V2013_11")]
        V201311,
        #[serde(rename = "V2014_04")]
        V201404,
        #[serde(rename = "V2014_06")]
        V201406,
        #[serde(rename = "V2014_07")]
        V201407,
        #[serde(rename = "V2014_09")]
        V201409,
        #[serde(rename = "V2014_10")]
        V201410,
        #[serde(rename = "V2014_12")]
        V201412,
        #[serde(rename = "V2015_01")]
        V201501,
        #[serde(rename = "V2015_02")]
        V201502,
        #[serde(rename = "V2015_04")]
        V201504,
        #[serde(rename = "V2015_05")]
        V201505,
        #[serde(rename = "V2015_06")]
        V201506,
        #[serde(rename = "V2015_07")]
        V201507,
        #[serde(rename = "V2015_08")]
        V201508,
        #[serde(rename = "V2015_10")]
        V201510,
        #[serde(rename = "V2015_12")]
        V201512,
        #[serde(rename = "V2016_01")]
        V201601,
        #[serde(rename = "V2016_02")]
        V201602,
        #[serde(rename = "V2016_04")]
        V201604,
        #[serde(rename = "V2016_05")]
        V201605,
        #[serde(rename = "V2016_07")]
        V201607,
        #[serde(rename = "V2016_08")]
        V201608,
    }
}
#[doc = "Upload Certificate Response from IDM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadCertificateResponse {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "Resource Certificate And AAD Details from IDM"]
    pub properties: ResourceCertificateAndAadDetails,
}
impl UploadCertificateResponse {
    pub fn new(properties: ResourceCertificateAndAadDetails) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
