#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The access control record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlRecord {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of access control record."]
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
#[doc = "The collection of access control records."]
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
#[doc = "The properties of access control record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlRecordProperties {
    #[doc = "The iSCSI initiator name (IQN)."]
    #[serde(rename = "initiatorName")]
    pub initiator_name: String,
    #[doc = "The number of volumes using the access control record."]
    #[serde(rename = "volumeCount", default, skip_serializing_if = "Option::is_none")]
    pub volume_count: Option<i32>,
}
impl AccessControlRecordProperties {
    pub fn new(initiator_name: String) -> Self {
        Self {
            initiator_name,
            volume_count: None,
        }
    }
}
#[doc = "The ACS configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsConfiguration {
    #[doc = "The namespace."]
    pub namespace: String,
    #[doc = "The realm."]
    pub realm: String,
    #[doc = "The service URL."]
    #[serde(rename = "serviceUrl")]
    pub service_url: String,
}
impl AcsConfiguration {
    pub fn new(namespace: String, realm: String, service_url: String) -> Self {
        Self {
            namespace,
            realm,
            service_url,
        }
    }
}
#[doc = "The alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of alert"]
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
#[doc = "The details of the error for which the alert was raised"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertErrorDetails {
    #[doc = "The error code"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The number of occurrences"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurences: Option<i32>,
}
impl AlertErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The OData filters to be used for Alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertFilter {
    #[doc = "Specifies the status of the alerts to be filtered. Only 'Equality' operator is supported for this property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<alert_filter::Status>,
    #[doc = "Specifies the severity of the alerts to be filtered. Only 'Equality' operator is supported for this property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<alert_filter::Severity>,
    #[doc = "Specifies the source type of the alerts to be filtered. Only 'Equality' operator is supported for this property."]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<alert_filter::SourceType>,
    #[doc = "Specifies the source name of the alerts to be filtered. Only 'Equality' operator is supported for this property."]
    #[serde(rename = "sourceName", default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[doc = "Specifies the appeared time (in UTC) of the alerts to be filtered. Only 'Greater-Than' and 'Lesser-Than' operators are supported for this property."]
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
    #[doc = "Specifies the status of the alerts to be filtered. Only 'Equality' operator is supported for this property."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Cleared,
    }
    #[doc = "Specifies the severity of the alerts to be filtered. Only 'Equality' operator is supported for this property."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Informational,
        Warning,
        Critical,
    }
    #[doc = "Specifies the source type of the alerts to be filtered. Only 'Equality' operator is supported for this property."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SourceType {
        Resource,
        Device,
    }
}
#[doc = "The collection of alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertList {
    #[doc = "The value."]
    pub value: Vec<Alert>,
    #[doc = "The URI of the next page of alerts."]
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
#[doc = "The properties of the alert notification settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertNotificationProperties {
    #[doc = "Indicates whether email notification enabled or not."]
    #[serde(rename = "emailNotification")]
    pub email_notification: alert_notification_properties::EmailNotification,
    #[doc = "The alert notification culture."]
    #[serde(rename = "alertNotificationCulture", default, skip_serializing_if = "Option::is_none")]
    pub alert_notification_culture: Option<String>,
    #[doc = "The value indicating whether alert notification enabled for admin or not."]
    #[serde(rename = "notificationToServiceOwners", default, skip_serializing_if = "Option::is_none")]
    pub notification_to_service_owners: Option<alert_notification_properties::NotificationToServiceOwners>,
    #[doc = "The alert notification email list."]
    #[serde(rename = "additionalRecipientEmailList", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_recipient_email_list: Vec<String>,
}
impl AlertNotificationProperties {
    pub fn new(email_notification: alert_notification_properties::EmailNotification) -> Self {
        Self {
            email_notification,
            alert_notification_culture: None,
            notification_to_service_owners: None,
            additional_recipient_email_list: Vec::new(),
        }
    }
}
pub mod alert_notification_properties {
    use super::*;
    #[doc = "Indicates whether email notification enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EmailNotification {
        Enabled,
        Disabled,
    }
    #[doc = "The value indicating whether alert notification enabled for admin or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NotificationToServiceOwners {
        Enabled,
        Disabled,
    }
}
#[doc = "The properties of alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProperties {
    #[doc = "The title of the alert"]
    pub title: String,
    #[doc = "The scope of the alert"]
    pub scope: alert_properties::Scope,
    #[doc = "The type of the alert"]
    #[serde(rename = "alertType")]
    pub alert_type: String,
    #[doc = "The UTC time at which the alert was raised"]
    #[serde(rename = "appearedAtTime", with = "azure_core::date::rfc3339")]
    pub appeared_at_time: time::OffsetDateTime,
    #[doc = "The source time at which the alert was raised"]
    #[serde(rename = "appearedAtSourceTime", with = "azure_core::date::rfc3339")]
    pub appeared_at_source_time: time::OffsetDateTime,
    #[doc = "The UTC time at which the alert was cleared"]
    #[serde(rename = "clearedAtTime", with = "azure_core::date::rfc3339::option")]
    pub cleared_at_time: Option<time::OffsetDateTime>,
    #[doc = "The source time at which the alert was cleared"]
    #[serde(rename = "clearedAtSourceTime", with = "azure_core::date::rfc3339::option")]
    pub cleared_at_source_time: Option<time::OffsetDateTime>,
    #[doc = "The source details at which the alert was raised"]
    pub source: AlertSource,
    #[doc = "The recommended action for the issue raised in the alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[doc = "The reason for resolving the alert"]
    #[serde(rename = "resolutionReason", default, skip_serializing_if = "Option::is_none")]
    pub resolution_reason: Option<String>,
    #[doc = "The severity of the alert"]
    pub severity: alert_properties::Severity,
    #[doc = "The current status of the alert"]
    pub status: alert_properties::Status,
    #[doc = "The details of the error for which the alert was raised"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<AlertErrorDetails>,
    #[doc = "More details about the alert"]
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
    #[doc = "The scope of the alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        Resource,
        Device,
    }
    #[doc = "The severity of the alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Informational,
        Warning,
        Critical,
    }
    #[doc = "The current status of the alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Cleared,
    }
}
#[doc = "The alert settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertSettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the alert notification settings."]
    pub properties: AlertNotificationProperties,
}
impl AlertSettings {
    pub fn new(properties: AlertNotificationProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The source details at which the alert was raised"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertSource {
    #[doc = "The name of the source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The time zone of the source"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "The source type of the alert"]
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
    #[doc = "The source type of the alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AlertSourceType {
        Resource,
        Device,
    }
}
#[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsymmetricEncryptedSecret {
    #[doc = "The value of the secret."]
    pub value: String,
    #[doc = "Thumbprint certificate that was used to encrypt \"Value\". If the value in unencrypted, it will be null."]
    #[serde(rename = "encryptionCertThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub encryption_cert_thumbprint: Option<String>,
    #[doc = "The algorithm used to encrypt \"Value\"."]
    #[serde(rename = "encryptionAlgorithm")]
    pub encryption_algorithm: asymmetric_encrypted_secret::EncryptionAlgorithm,
}
impl AsymmetricEncryptedSecret {
    pub fn new(value: String, encryption_algorithm: asymmetric_encrypted_secret::EncryptionAlgorithm) -> Self {
        Self {
            value,
            encryption_cert_thumbprint: None,
            encryption_algorithm,
        }
    }
}
pub mod asymmetric_encrypted_secret {
    use super::*;
    #[doc = "The algorithm used to encrypt \"Value\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionAlgorithm {
        None,
        #[serde(rename = "AES256")]
        Aes256,
        #[serde(rename = "RSAES_PKCS1_v_1_5")]
        RsaesPkcs1V15,
    }
}
#[doc = "Represents available provider operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperation {
    #[doc = "The name of the operation being performed on a particular object. Name format: \"{resourceProviderNamespace}/{resourceType}/{read|write|delete|action}\". Eg. Microsoft.StorSimple/managers/devices/volumeContainers/read, Microsoft.StorSimple/managers/devices/alerts/clearAlerts/action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contains the localized display information for this particular operation/action. These value will be used by several clients for (a) custom role definitions for RBAC, (b) complex query filters for the event service and (c) audit history/records for management operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableProviderOperationDisplay>,
    #[doc = "The intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Represents properties of AvailableProviderOperation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableProviderOperationProperties>,
}
impl AvailableProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the localized display information for this particular operation/action. These value will be used by several clients for (a) custom role definitions for RBAC, (b) complex query filters for the event service and (c) audit history/records for management operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperationDisplay {
    #[doc = "The localized friendly form of the resource provider name - it is expected to also include the publisher/company responsible. It should use Title Casing and begin with 'Microsoft' for 1st party services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The localized friendly form of the resource type related to this action/operation - it should match the public documentation for the resource provider. It should use Title Casing - for examples, please refer to the 'name' section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The localized friendly name for the operation, as it should be shown to the user. It should be concise (to fit in drop downs) but clear (i.e. self-documenting). It should use Title Casing and include the entity/resource to which it applies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The localized friendly description for the operation, as it should be shown to the user. It should be thorough, yet concise - it will be used in tool tips and detailed views."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AvailableProviderOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of available provider operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableProviderOperationList {
    #[doc = "The value."]
    pub value: Vec<AvailableProviderOperation>,
    #[doc = "The NextLink."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailableProviderOperationList {
    pub fn new(value: Vec<AvailableProviderOperation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Represents properties of AvailableProviderOperation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperationProperties {}
impl AvailableProviderOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Backup {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the backup."]
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
#[doc = "The backup element."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupElement {
    #[doc = "The path ID that uniquely identifies the backup element."]
    #[serde(rename = "elementId")]
    pub element_id: String,
    #[doc = "The name of the backup element."]
    #[serde(rename = "elementName")]
    pub element_name: String,
    #[doc = "The hierarchical type of the backup element."]
    #[serde(rename = "elementType")]
    pub element_type: String,
    #[doc = "The size in bytes."]
    #[serde(rename = "sizeInBytes")]
    pub size_in_bytes: i64,
    #[doc = "The name of the volume."]
    #[serde(rename = "volumeName")]
    pub volume_name: String,
    #[doc = "The path ID of the volume container."]
    #[serde(rename = "volumeContainerId")]
    pub volume_container_id: String,
    #[doc = "The volume type."]
    #[serde(rename = "volumeType", default, skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<backup_element::VolumeType>,
}
impl BackupElement {
    pub fn new(
        element_id: String,
        element_name: String,
        element_type: String,
        size_in_bytes: i64,
        volume_name: String,
        volume_container_id: String,
    ) -> Self {
        Self {
            element_id,
            element_name,
            element_type,
            size_in_bytes,
            volume_name,
            volume_container_id,
            volume_type: None,
        }
    }
}
pub mod backup_element {
    use super::*;
    #[doc = "The volume type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VolumeType {
        Tiered,
        Archival,
        LocallyPinned,
    }
}
#[doc = "The OData filters to be used for backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupFilter {
    #[doc = "Specifies the backupPolicyId of the backups to be filtered. Only 'Equality' operator is supported for this property."]
    #[serde(rename = "backupPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy_id: Option<String>,
    #[doc = "Specifies the volumeId of the backups to be filtered. Only 'Equality' operator is supported for this property."]
    #[serde(rename = "volumeId", default, skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    #[doc = "Specifies the creation time of the backups to be filtered. Only 'Greater Than or Equal To' and 'Lesser Than or Equal To' operators are supported for this property."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
}
impl BackupFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The collection of backups."]
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
#[doc = "The backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicy {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the backup policy."]
    pub properties: BackupPolicyProperties,
}
impl BackupPolicy {
    pub fn new(properties: BackupPolicyProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The collection of backup policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicyList {
    #[doc = "The value."]
    pub value: Vec<BackupPolicy>,
}
impl azure_core::Continuable for BackupPolicyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BackupPolicyList {
    pub fn new(value: Vec<BackupPolicy>) -> Self {
        Self { value }
    }
}
#[doc = "The properties of the backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicyProperties {
    #[doc = "The path IDs of the volumes which are part of the backup policy."]
    #[serde(rename = "volumeIds")]
    pub volume_ids: Vec<String>,
    #[doc = "The time of the next backup for the backup policy."]
    #[serde(rename = "nextBackupTime", with = "azure_core::date::rfc3339::option")]
    pub next_backup_time: Option<time::OffsetDateTime>,
    #[doc = "The time of the last backup for the backup policy."]
    #[serde(rename = "lastBackupTime", with = "azure_core::date::rfc3339::option")]
    pub last_backup_time: Option<time::OffsetDateTime>,
    #[doc = "The count of schedules the backup policy contains."]
    #[serde(rename = "schedulesCount", default, skip_serializing_if = "Option::is_none")]
    pub schedules_count: Option<i64>,
    #[doc = "Indicates whether at least one of the schedules in the backup policy is active or not."]
    #[serde(rename = "scheduledBackupStatus", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_backup_status: Option<backup_policy_properties::ScheduledBackupStatus>,
    #[doc = "The backup policy creation type. Indicates whether this was created through SaaS or through StorSimple Snapshot Manager."]
    #[serde(rename = "backupPolicyCreationType", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy_creation_type: Option<backup_policy_properties::BackupPolicyCreationType>,
    #[doc = "If the backup policy was created by StorSimple Snapshot Manager, then this field indicates the hostname of the StorSimple Snapshot Manager."]
    #[serde(rename = "ssmHostName", default, skip_serializing_if = "Option::is_none")]
    pub ssm_host_name: Option<String>,
}
impl BackupPolicyProperties {
    pub fn new(volume_ids: Vec<String>) -> Self {
        Self {
            volume_ids,
            next_backup_time: None,
            last_backup_time: None,
            schedules_count: None,
            scheduled_backup_status: None,
            backup_policy_creation_type: None,
            ssm_host_name: None,
        }
    }
}
pub mod backup_policy_properties {
    use super::*;
    #[doc = "Indicates whether at least one of the schedules in the backup policy is active or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScheduledBackupStatus {
        Disabled,
        Enabled,
    }
    #[doc = "The backup policy creation type. Indicates whether this was created through SaaS or through StorSimple Snapshot Manager."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupPolicyCreationType {
        BySaaS,
        #[serde(rename = "BySSM")]
        BySsm,
    }
}
#[doc = "The properties of the backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupProperties {
    #[doc = "The time when the backup was created."]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339")]
    pub created_on: time::OffsetDateTime,
    #[doc = "The backup size in bytes."]
    #[serde(rename = "sizeInBytes")]
    pub size_in_bytes: i64,
    #[doc = "The type of the backup."]
    #[serde(rename = "backupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<backup_properties::BackupType>,
    #[doc = "The backup job creation type."]
    #[serde(rename = "backupJobCreationType", default, skip_serializing_if = "Option::is_none")]
    pub backup_job_creation_type: Option<backup_properties::BackupJobCreationType>,
    #[doc = "The path ID of the backup policy."]
    #[serde(rename = "backupPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy_id: Option<String>,
    #[doc = "The StorSimple Snapshot Manager host name."]
    #[serde(rename = "ssmHostName", default, skip_serializing_if = "Option::is_none")]
    pub ssm_host_name: Option<String>,
    #[doc = "The backup elements."]
    pub elements: Vec<BackupElement>,
}
impl BackupProperties {
    pub fn new(created_on: time::OffsetDateTime, size_in_bytes: i64, elements: Vec<BackupElement>) -> Self {
        Self {
            created_on,
            size_in_bytes,
            backup_type: None,
            backup_job_creation_type: None,
            backup_policy_id: None,
            ssm_host_name: None,
            elements,
        }
    }
}
pub mod backup_properties {
    use super::*;
    #[doc = "The type of the backup."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupType {
        LocalSnapshot,
        CloudSnapshot,
    }
    #[doc = "The backup job creation type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupJobCreationType {
        Adhoc,
        BySchedule,
        #[serde(rename = "BySSM")]
        BySsm,
    }
}
#[doc = "The backup schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupSchedule {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the backup schedule."]
    pub properties: BackupScheduleProperties,
}
impl BackupSchedule {
    pub fn new(properties: BackupScheduleProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The backup schedule list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupScheduleList {
    #[doc = "The value."]
    pub value: Vec<BackupSchedule>,
}
impl azure_core::Continuable for BackupScheduleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BackupScheduleList {
    pub fn new(value: Vec<BackupSchedule>) -> Self {
        Self { value }
    }
}
#[doc = "The properties of the backup schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupScheduleProperties {
    #[doc = "The schedule recurrence."]
    #[serde(rename = "scheduleRecurrence")]
    pub schedule_recurrence: ScheduleRecurrence,
    #[doc = "The type of backup which needs to be taken."]
    #[serde(rename = "backupType")]
    pub backup_type: backup_schedule_properties::BackupType,
    #[doc = "The number of backups to be retained."]
    #[serde(rename = "retentionCount")]
    pub retention_count: i64,
    #[doc = "The start time of the schedule."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "The schedule status."]
    #[serde(rename = "scheduleStatus")]
    pub schedule_status: backup_schedule_properties::ScheduleStatus,
    #[doc = "The last successful backup run which was triggered for the schedule."]
    #[serde(rename = "lastSuccessfulRun", with = "azure_core::date::rfc3339::option")]
    pub last_successful_run: Option<time::OffsetDateTime>,
}
impl BackupScheduleProperties {
    pub fn new(
        schedule_recurrence: ScheduleRecurrence,
        backup_type: backup_schedule_properties::BackupType,
        retention_count: i64,
        start_time: time::OffsetDateTime,
        schedule_status: backup_schedule_properties::ScheduleStatus,
    ) -> Self {
        Self {
            schedule_recurrence,
            backup_type,
            retention_count,
            start_time,
            schedule_status,
            last_successful_run: None,
        }
    }
}
pub mod backup_schedule_properties {
    use super::*;
    #[doc = "The type of backup which needs to be taken."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupType {
        LocalSnapshot,
        CloudSnapshot,
    }
    #[doc = "The schedule status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScheduleStatus {
        Enabled,
        Disabled,
    }
}
#[doc = "The properties of the bandwidth setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandwidthRateSettingProperties {
    #[doc = "The schedules."]
    pub schedules: Vec<BandwidthSchedule>,
    #[doc = "The number of volumes that uses the bandwidth setting."]
    #[serde(rename = "volumeCount", default, skip_serializing_if = "Option::is_none")]
    pub volume_count: Option<i32>,
}
impl BandwidthRateSettingProperties {
    pub fn new(schedules: Vec<BandwidthSchedule>) -> Self {
        Self {
            schedules,
            volume_count: None,
        }
    }
}
#[doc = "The schedule for bandwidth setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandwidthSchedule {
    #[doc = "The time."]
    pub start: Time,
    #[doc = "The time."]
    pub stop: Time,
    #[doc = "The rate in Mbps."]
    #[serde(rename = "rateInMbps")]
    pub rate_in_mbps: i32,
    #[doc = "The days of the week when this schedule is applicable."]
    pub days: Vec<String>,
}
impl BandwidthSchedule {
    pub fn new(start: Time, stop: Time, rate_in_mbps: i32, days: Vec<String>) -> Self {
        Self {
            start,
            stop,
            rate_in_mbps,
            days,
        }
    }
}
#[doc = "The bandwidth setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandwidthSetting {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the bandwidth setting."]
    pub properties: BandwidthRateSettingProperties,
}
impl BandwidthSetting {
    pub fn new(properties: BandwidthRateSettingProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The collection of bandwidth setting entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandwidthSettingList {
    #[doc = "The value."]
    pub value: Vec<BandwidthSetting>,
}
impl azure_core::Continuable for BandwidthSettingList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BandwidthSettingList {
    pub fn new(value: Vec<BandwidthSetting>) -> Self {
        Self { value }
    }
}
#[doc = "Represents the base class for all other ARM object models"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseModel {
    #[doc = "The path ID that uniquely identifies the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hierarchical type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The Kind of the object. Currently only Series8000 is supported"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<base_model::Kind>,
}
impl BaseModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod base_model {
    use super::*;
    #[doc = "The Kind of the object. Currently only Series8000 is supported"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Series8000,
    }
}
#[doc = "The Challenge-Handshake Authentication Protocol (CHAP) settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChapSettings {
    #[doc = "The CHAP initiator user."]
    #[serde(rename = "initiatorUser", default, skip_serializing_if = "Option::is_none")]
    pub initiator_user: Option<String>,
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "initiatorSecret", default, skip_serializing_if = "Option::is_none")]
    pub initiator_secret: Option<AsymmetricEncryptedSecret>,
    #[doc = "The CHAP target user."]
    #[serde(rename = "targetUser", default, skip_serializing_if = "Option::is_none")]
    pub target_user: Option<String>,
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "targetSecret", default, skip_serializing_if = "Option::is_none")]
    pub target_secret: Option<AsymmetricEncryptedSecret>,
}
impl ChapSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request for clearing the alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearAlertRequest {
    #[doc = "The resolution message while clearing the alert"]
    #[serde(rename = "resolutionMessage", default, skip_serializing_if = "Option::is_none")]
    pub resolution_message: Option<String>,
    #[doc = "The list of alert IDs to be cleared"]
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
#[doc = "The clone job request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloneRequest {
    #[doc = "The path ID of the device which will act as the clone target."]
    #[serde(rename = "targetDeviceId")]
    pub target_device_id: String,
    #[doc = "The name of the new volume which will be created and the backup will be cloned into."]
    #[serde(rename = "targetVolumeName")]
    pub target_volume_name: String,
    #[doc = "The list of path IDs of the access control records to be associated to the new cloned volume."]
    #[serde(rename = "targetAccessControlRecordIds")]
    pub target_access_control_record_ids: Vec<String>,
    #[doc = "The backup element."]
    #[serde(rename = "backupElement")]
    pub backup_element: BackupElement,
}
impl CloneRequest {
    pub fn new(
        target_device_id: String,
        target_volume_name: String,
        target_access_control_record_ids: Vec<String>,
        backup_element: BackupElement,
    ) -> Self {
        Self {
            target_device_id,
            target_volume_name,
            target_access_control_record_ids,
            backup_element,
        }
    }
}
#[doc = "The cloud appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudAppliance {
    #[doc = "The name."]
    pub name: String,
    #[doc = "The name of the virtual network."]
    #[serde(rename = "vnetName", default, skip_serializing_if = "Option::is_none")]
    pub vnet_name: Option<String>,
    #[doc = "The virtual network region."]
    #[serde(rename = "vnetRegion")]
    pub vnet_region: String,
    #[doc = "Indicates whether virtual network used is configured with DNS or not."]
    #[serde(rename = "isVnetDnsConfigured", default, skip_serializing_if = "Option::is_none")]
    pub is_vnet_dns_configured: Option<bool>,
    #[doc = "Indicates whether virtual network used is configured with express route or not."]
    #[serde(rename = "isVnetExpressConfigured", default, skip_serializing_if = "Option::is_none")]
    pub is_vnet_express_configured: Option<bool>,
    #[doc = "The name of the subnet."]
    #[serde(rename = "subnetName", default, skip_serializing_if = "Option::is_none")]
    pub subnet_name: Option<String>,
    #[doc = "The name of the storage account."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
    #[doc = "The type of the storage account."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<String>,
    #[doc = "The type of the virtual machine."]
    #[serde(rename = "vmType", default, skip_serializing_if = "Option::is_none")]
    pub vm_type: Option<String>,
    #[doc = "The name of the virtual machine image."]
    #[serde(rename = "vmImageName", default, skip_serializing_if = "Option::is_none")]
    pub vm_image_name: Option<String>,
    #[doc = "The model number."]
    #[serde(rename = "modelNumber", default, skip_serializing_if = "Option::is_none")]
    pub model_number: Option<String>,
}
impl CloudAppliance {
    pub fn new(name: String, vnet_region: String) -> Self {
        Self {
            name,
            vnet_name: None,
            vnet_region,
            is_vnet_dns_configured: None,
            is_vnet_express_configured: None,
            subnet_name: None,
            storage_account_name: None,
            storage_account_type: None,
            vm_type: None,
            vm_image_name: None,
            model_number: None,
        }
    }
}
#[doc = "The cloud appliance configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudApplianceConfiguration {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of cloud appliance configuration."]
    pub properties: CloudApplianceConfigurationProperties,
}
impl CloudApplianceConfiguration {
    pub fn new(properties: CloudApplianceConfigurationProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The cloud appliance configuration list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudApplianceConfigurationList {
    #[doc = "The value."]
    pub value: Vec<CloudApplianceConfiguration>,
}
impl azure_core::Continuable for CloudApplianceConfigurationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudApplianceConfigurationList {
    pub fn new(value: Vec<CloudApplianceConfiguration>) -> Self {
        Self { value }
    }
}
#[doc = "The properties of cloud appliance configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudApplianceConfigurationProperties {
    #[doc = "The model number."]
    #[serde(rename = "modelNumber")]
    pub model_number: String,
    #[doc = "The cloud platform."]
    #[serde(rename = "cloudPlatform")]
    pub cloud_platform: String,
    #[doc = "The ACS configuration."]
    #[serde(rename = "acsConfiguration")]
    pub acs_configuration: AcsConfiguration,
    #[doc = "The supported storage account types."]
    #[serde(rename = "supportedStorageAccountTypes")]
    pub supported_storage_account_types: Vec<String>,
    #[doc = "The supported regions."]
    #[serde(rename = "supportedRegions")]
    pub supported_regions: Vec<String>,
    #[doc = "The supported virtual machine types."]
    #[serde(rename = "supportedVmTypes")]
    pub supported_vm_types: Vec<String>,
    #[doc = "The supported virtual machine images."]
    #[serde(rename = "supportedVmImages")]
    pub supported_vm_images: Vec<VmImage>,
}
impl CloudApplianceConfigurationProperties {
    pub fn new(
        model_number: String,
        cloud_platform: String,
        acs_configuration: AcsConfiguration,
        supported_storage_account_types: Vec<String>,
        supported_regions: Vec<String>,
        supported_vm_types: Vec<String>,
        supported_vm_images: Vec<VmImage>,
    ) -> Self {
        Self {
            model_number,
            cloud_platform,
            acs_configuration,
            supported_storage_account_types,
            supported_regions,
            supported_vm_types,
            supported_vm_images,
        }
    }
}
#[doc = "The cloud appliance settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudApplianceSettings {
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "serviceDataEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub service_data_encryption_key: Option<AsymmetricEncryptedSecret>,
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "channelIntegrityKey", default, skip_serializing_if = "Option::is_none")]
    pub channel_integrity_key: Option<AsymmetricEncryptedSecret>,
}
impl CloudApplianceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The mandatory device configuration request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigureDeviceRequest {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the configure device request."]
    pub properties: ConfigureDeviceRequestProperties,
}
impl ConfigureDeviceRequest {
    pub fn new(properties: ConfigureDeviceRequestProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The properties of the configure device request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigureDeviceRequestProperties {
    #[doc = "The friendly name for the device."]
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[doc = "The current name of the device."]
    #[serde(rename = "currentDeviceName")]
    pub current_device_name: String,
    #[doc = "The device time zone. For eg: \"Pacific Standard Time\""]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[doc = "The secondary DNS settings."]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<SecondaryDnsSettings>,
    #[doc = "The 'Data 0' network interface card settings."]
    #[serde(rename = "networkInterfaceData0Settings", default, skip_serializing_if = "Option::is_none")]
    pub network_interface_data0_settings: Option<NetworkInterfaceData0Settings>,
}
impl ConfigureDeviceRequestProperties {
    pub fn new(friendly_name: String, current_device_name: String, time_zone: String) -> Self {
        Self {
            friendly_name,
            current_device_name,
            time_zone,
            dns_settings: None,
            network_interface_data0_settings: None,
        }
    }
}
#[doc = "The controller power state change request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControllerPowerStateChangeRequest {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the controller power state change request."]
    pub properties: ControllerPowerStateChangeRequestProperties,
}
impl ControllerPowerStateChangeRequest {
    pub fn new(properties: ControllerPowerStateChangeRequestProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The properties of the controller power state change request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControllerPowerStateChangeRequestProperties {
    #[doc = "The power state that the request is expecting for the controller of the device."]
    pub action: controller_power_state_change_request_properties::Action,
    #[doc = "The active controller that the request is expecting on the device."]
    #[serde(rename = "activeController")]
    pub active_controller: controller_power_state_change_request_properties::ActiveController,
    #[doc = "The controller 0's status that the request is expecting on the device."]
    #[serde(rename = "controller0State")]
    pub controller0_state: controller_power_state_change_request_properties::Controller0State,
    #[doc = "The controller 1's status that the request is expecting on the device."]
    #[serde(rename = "controller1State")]
    pub controller1_state: controller_power_state_change_request_properties::Controller1State,
}
impl ControllerPowerStateChangeRequestProperties {
    pub fn new(
        action: controller_power_state_change_request_properties::Action,
        active_controller: controller_power_state_change_request_properties::ActiveController,
        controller0_state: controller_power_state_change_request_properties::Controller0State,
        controller1_state: controller_power_state_change_request_properties::Controller1State,
    ) -> Self {
        Self {
            action,
            active_controller,
            controller0_state,
            controller1_state,
        }
    }
}
pub mod controller_power_state_change_request_properties {
    use super::*;
    #[doc = "The power state that the request is expecting for the controller of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Start,
        Restart,
        Shutdown,
    }
    #[doc = "The active controller that the request is expecting on the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActiveController {
        Unknown,
        None,
        Controller0,
        Controller1,
    }
    #[doc = "The controller 0's status that the request is expecting on the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Controller0State {
        NotPresent,
        PoweredOff,
        Ok,
        Recovering,
        Warning,
        Failure,
    }
    #[doc = "The controller 1's status that the request is expecting on the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Controller1State {
        NotPresent,
        PoweredOff,
        Ok,
        Recovering,
        Warning,
        Failure,
    }
}
#[doc = "The DNS(Domain Name Server) settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsSettings {
    #[doc = "The primary IPv4 DNS server for the device"]
    #[serde(rename = "primaryDnsServer", default, skip_serializing_if = "Option::is_none")]
    pub primary_dns_server: Option<String>,
    #[doc = "The primary IPv6 DNS server for the device"]
    #[serde(rename = "primaryIpv6DnsServer", default, skip_serializing_if = "Option::is_none")]
    pub primary_ipv6_dns_server: Option<String>,
    #[doc = "The secondary IPv4 DNS server for the device"]
    #[serde(rename = "secondaryDnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub secondary_dns_servers: Vec<String>,
    #[doc = "The secondary IPv6 DNS server for the device"]
    #[serde(rename = "secondaryIpv6DnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub secondary_ipv6_dns_servers: Vec<String>,
}
impl DnsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The additional details related to the data related statistics of a job. Currently applicable only for Backup, Clone and Restore jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataStatistics {
    #[doc = "The total bytes of data to be processed, as part of the job."]
    #[serde(rename = "totalData", default, skip_serializing_if = "Option::is_none")]
    pub total_data: Option<i64>,
    #[doc = "The number of bytes of data processed till now, as part of the job."]
    #[serde(rename = "processedData", default, skip_serializing_if = "Option::is_none")]
    pub processed_data: Option<i64>,
    #[doc = "The number of bytes of data written to cloud, as part of the job."]
    #[serde(rename = "cloudData", default, skip_serializing_if = "Option::is_none")]
    pub cloud_data: Option<i64>,
    #[doc = "The average throughput of data processed(bytes/sec), as part of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,
}
impl DataStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The StorSimple device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the StorSimple device."]
    pub properties: DeviceProperties,
}
impl Device {
    pub fn new(properties: DeviceProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The additional device details regarding the end point count and volume container count."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceDetails {
    #[doc = "The total number of endpoints that are currently on the device ( i.e. number of volumes)."]
    #[serde(rename = "endpointCount", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_count: Option<i32>,
    #[doc = "The total number of volume containers on the device."]
    #[serde(rename = "volumeContainerCount", default, skip_serializing_if = "Option::is_none")]
    pub volume_container_count: Option<i32>,
}
impl DeviceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The collection of devices."]
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
#[doc = "The device patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicePatch {
    #[doc = "The properties of the device patch."]
    pub properties: DevicePatchProperties,
}
impl DevicePatch {
    pub fn new(properties: DevicePatchProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The properties of the device patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevicePatchProperties {
    #[doc = "Short description given for the device"]
    #[serde(rename = "deviceDescription", default, skip_serializing_if = "Option::is_none")]
    pub device_description: Option<String>,
}
impl DevicePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the StorSimple device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceProperties {
    #[doc = "The friendly name of the device."]
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[doc = "The UTC time at which the device was activated"]
    #[serde(rename = "activationTime", with = "azure_core::date::rfc3339")]
    pub activation_time: time::OffsetDateTime,
    #[doc = "The language culture setting on the device. For eg: \"en-US\""]
    pub culture: String,
    #[doc = "The device description."]
    #[serde(rename = "deviceDescription")]
    pub device_description: String,
    #[doc = "The version number of the software running on the device."]
    #[serde(rename = "deviceSoftwareVersion")]
    pub device_software_version: String,
    #[doc = "The friendly name of the software running on the device."]
    #[serde(rename = "friendlySoftwareName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_software_name: Option<String>,
    #[doc = "The current configuration status of the device."]
    #[serde(rename = "deviceConfigurationStatus")]
    pub device_configuration_status: device_properties::DeviceConfigurationStatus,
    #[doc = "The target IQN."]
    #[serde(rename = "targetIqn")]
    pub target_iqn: String,
    #[doc = "The device model."]
    #[serde(rename = "modelDescription")]
    pub model_description: String,
    #[doc = "The current status of the device."]
    pub status: device_properties::Status,
    #[doc = "The serial number."]
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[doc = "The type of the device."]
    #[serde(rename = "deviceType")]
    pub device_type: device_properties::DeviceType,
    #[doc = "The identifier of the active controller of the device."]
    #[serde(rename = "activeController")]
    pub active_controller: device_properties::ActiveController,
    #[doc = "The device friendly software version."]
    #[serde(rename = "friendlySoftwareVersion")]
    pub friendly_software_version: String,
    #[doc = "The storage in bytes that is available locally on the device."]
    #[serde(rename = "availableLocalStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_local_storage_in_bytes: Option<i64>,
    #[doc = "The storage in bytes that is available on the device for tiered volumes."]
    #[serde(rename = "availableTieredStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_tiered_storage_in_bytes: Option<i64>,
    #[doc = "The storage in bytes that has been provisioned on the device for tiered volumes."]
    #[serde(rename = "provisionedTieredStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_tiered_storage_in_bytes: Option<i64>,
    #[doc = "The storage in bytes used for locally pinned volumes on the device (including additional local reservation)."]
    #[serde(rename = "provisionedLocalStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_local_storage_in_bytes: Option<i64>,
    #[doc = "Total capacity in bytes of tiered and locally pinned volumes on the device"]
    #[serde(rename = "provisionedVolumeSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_volume_size_in_bytes: Option<i64>,
    #[doc = "The storage in bytes that is currently being used on the device, including both local and cloud."]
    #[serde(rename = "usingStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub using_storage_in_bytes: Option<i64>,
    #[doc = "The total tiered storage available on the device in bytes."]
    #[serde(rename = "totalTieredStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_tiered_storage_in_bytes: Option<i64>,
    #[doc = "The device agent group version."]
    #[serde(rename = "agentGroupVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_group_version: Option<i32>,
    #[doc = "The number of network interface cards"]
    #[serde(rename = "networkInterfaceCardCount", default, skip_serializing_if = "Option::is_none")]
    pub network_interface_card_count: Option<i32>,
    #[doc = "The location of the virtual appliance."]
    #[serde(rename = "deviceLocation", default, skip_serializing_if = "Option::is_none")]
    pub device_location: Option<String>,
    #[doc = "The virtual machine API type."]
    #[serde(rename = "virtualMachineApiType", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_api_type: Option<device_properties::VirtualMachineApiType>,
    #[doc = "The additional device details regarding the end point count and volume container count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<DeviceDetails>,
    #[doc = "The additional device details for the service data encryption key rollover."]
    #[serde(rename = "rolloverDetails", default, skip_serializing_if = "Option::is_none")]
    pub rollover_details: Option<DeviceRolloverDetails>,
}
impl DeviceProperties {
    pub fn new(
        friendly_name: String,
        activation_time: time::OffsetDateTime,
        culture: String,
        device_description: String,
        device_software_version: String,
        device_configuration_status: device_properties::DeviceConfigurationStatus,
        target_iqn: String,
        model_description: String,
        status: device_properties::Status,
        serial_number: String,
        device_type: device_properties::DeviceType,
        active_controller: device_properties::ActiveController,
        friendly_software_version: String,
    ) -> Self {
        Self {
            friendly_name,
            activation_time,
            culture,
            device_description,
            device_software_version,
            friendly_software_name: None,
            device_configuration_status,
            target_iqn,
            model_description,
            status,
            serial_number,
            device_type,
            active_controller,
            friendly_software_version,
            available_local_storage_in_bytes: None,
            available_tiered_storage_in_bytes: None,
            provisioned_tiered_storage_in_bytes: None,
            provisioned_local_storage_in_bytes: None,
            provisioned_volume_size_in_bytes: None,
            using_storage_in_bytes: None,
            total_tiered_storage_in_bytes: None,
            agent_group_version: None,
            network_interface_card_count: None,
            device_location: None,
            virtual_machine_api_type: None,
            details: None,
            rollover_details: None,
        }
    }
}
pub mod device_properties {
    use super::*;
    #[doc = "The current configuration status of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceConfigurationStatus {
        Complete,
        Pending,
    }
    #[doc = "The current status of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unknown,
        Online,
        Offline,
        Deactivated,
        RequiresAttention,
        MaintenanceMode,
        Creating,
        Provisioning,
        Deactivating,
        Deleted,
        ReadyToSetup,
    }
    #[doc = "The type of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceType {
        Invalid,
        Series8000VirtualAppliance,
        Series8000PhysicalAppliance,
    }
    #[doc = "The identifier of the active controller of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActiveController {
        Unknown,
        None,
        Controller0,
        Controller1,
    }
    #[doc = "The virtual machine API type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VirtualMachineApiType {
        Classic,
        Arm,
    }
}
#[doc = "The additional device details for the service data encryption key rollover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceRolloverDetails {
    #[doc = "The eligibility status of device for service data encryption key rollover."]
    #[serde(rename = "authorizationEligibility", default, skip_serializing_if = "Option::is_none")]
    pub authorization_eligibility: Option<device_rollover_details::AuthorizationEligibility>,
    #[doc = "The authorization status of the device for service data encryption key rollover."]
    #[serde(rename = "authorizationStatus", default, skip_serializing_if = "Option::is_none")]
    pub authorization_status: Option<device_rollover_details::AuthorizationStatus>,
    #[doc = "The reason for inEligibility of device, in case it's not eligible for service data encryption key rollover."]
    #[serde(rename = "inEligibilityReason", default, skip_serializing_if = "Option::is_none")]
    pub in_eligibility_reason: Option<device_rollover_details::InEligibilityReason>,
}
impl DeviceRolloverDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod device_rollover_details {
    use super::*;
    #[doc = "The eligibility status of device for service data encryption key rollover."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthorizationEligibility {
        InEligible,
        Eligible,
    }
    #[doc = "The authorization status of the device for service data encryption key rollover."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthorizationStatus {
        Disabled,
        Enabled,
    }
    #[doc = "The reason for inEligibility of device, in case it's not eligible for service data encryption key rollover."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InEligibilityReason {
        DeviceNotOnline,
        NotSupportedAppliance,
        RolloverPending,
    }
}
#[doc = "The dimension filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionFilter {
    #[doc = "Specifies the dimension name. E.g., NetworkInterface. Valid values are the ones specified in the field \"dimensions\" in the ListMetricDefinitions call. Only 'Equality' operator is supported for this property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the dimension value. E.g., Data0. Valid values are the ones returned in the field \"dimensions\" in the ListMetricDefinitions call. Only 'Equality' operator is supported for this property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
}
impl DimensionFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The encryption settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of encryption settings."]
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
#[doc = "The properties of encryption settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettingsProperties {
    #[doc = "The encryption status to indicates if encryption is enabled or not."]
    #[serde(rename = "encryptionStatus")]
    pub encryption_status: encryption_settings_properties::EncryptionStatus,
    #[doc = "The key rollover status to indicates if key rollover is required or not. If secret's encryption has been upgraded, then it requires key rollover."]
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
    #[doc = "The encryption status to indicates if encryption is enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionStatus {
        Enabled,
        Disabled,
    }
    #[doc = "The key rollover status to indicates if key rollover is required or not. If secret's encryption has been upgraded, then it requires key rollover."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyRolloverStatus {
        Required,
        NotRequired,
    }
}
#[doc = "The request object for triggering a failover of volume containers, from a source device to a target device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverRequest {
    #[doc = "The ARM path ID of the device which will act as the failover target."]
    #[serde(rename = "targetDeviceId", default, skip_serializing_if = "Option::is_none")]
    pub target_device_id: Option<String>,
    #[doc = "The list of path IDs of the volume containers which needs to be failed-over to the target device."]
    #[serde(rename = "volumeContainers", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_containers: Vec<String>,
}
impl FailoverRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The failover set on a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverSet {
    #[doc = "The list of meta data of volume containers, which are part of the failover set."]
    #[serde(rename = "volumeContainers", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_containers: Vec<VolumeContainerFailoverMetadata>,
    #[doc = "The eligibility result of failover set, for failover."]
    #[serde(rename = "eligibilityResult", default, skip_serializing_if = "Option::is_none")]
    pub eligibility_result: Option<FailoverSetEligibilityResult>,
}
impl FailoverSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The eligibility result of failover set, for failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverSetEligibilityResult {
    #[doc = "Represents if this failover set is eligible for failover or not."]
    #[serde(rename = "isEligibleForFailover", default, skip_serializing_if = "Option::is_none")]
    pub is_eligible_for_failover: Option<bool>,
    #[doc = "The error message, if the failover set is not eligible for failover."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl FailoverSetEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of failover sets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverSetsList {
    #[doc = "The list of failover sets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FailoverSet>,
}
impl azure_core::Continuable for FailoverSetsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl FailoverSetsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the eligibility of a device as a failover target device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverTarget {
    #[doc = "The path ID of the device."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "The status of the device."]
    #[serde(rename = "deviceStatus", default, skip_serializing_if = "Option::is_none")]
    pub device_status: Option<failover_target::DeviceStatus>,
    #[doc = "The model number of the device."]
    #[serde(rename = "modelDescription", default, skip_serializing_if = "Option::is_none")]
    pub model_description: Option<String>,
    #[doc = "The software version of the device."]
    #[serde(rename = "deviceSoftwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub device_software_version: Option<String>,
    #[doc = "The count of data containers on the device."]
    #[serde(rename = "dataContainersCount", default, skip_serializing_if = "Option::is_none")]
    pub data_containers_count: Option<i32>,
    #[doc = "The count of volumes on the device."]
    #[serde(rename = "volumesCount", default, skip_serializing_if = "Option::is_none")]
    pub volumes_count: Option<i32>,
    #[doc = "The amount of free local storage available on the device in bytes."]
    #[serde(rename = "availableLocalStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_local_storage_in_bytes: Option<i64>,
    #[doc = "The amount of free tiered storage available for the device in bytes."]
    #[serde(rename = "availableTieredStorageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_tiered_storage_in_bytes: Option<i64>,
    #[doc = "The geo location (applicable only for cloud appliances) of the device."]
    #[serde(rename = "deviceLocation", default, skip_serializing_if = "Option::is_none")]
    pub device_location: Option<String>,
    #[doc = "The friendly name for the current version of software on the device."]
    #[serde(rename = "friendlyDeviceSoftwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub friendly_device_software_version: Option<String>,
    #[doc = "The eligibility result of device, as a failover target device."]
    #[serde(rename = "eligibilityResult", default, skip_serializing_if = "Option::is_none")]
    pub eligibility_result: Option<TargetEligibilityResult>,
}
impl FailoverTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod failover_target {
    use super::*;
    #[doc = "The status of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceStatus {
        Unknown,
        Online,
        Offline,
        Deactivated,
        RequiresAttention,
        MaintenanceMode,
        Creating,
        Provisioning,
        Deactivating,
        Deleted,
        ReadyToSetup,
    }
}
#[doc = "The list of all devices in a resource and their eligibility status as a failover target device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverTargetsList {
    #[doc = "The list of all the failover targets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FailoverTarget>,
}
impl azure_core::Continuable for FailoverTargetsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl FailoverTargetsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The feature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    #[doc = "The name of the feature."]
    pub name: String,
    #[doc = "The feature support status."]
    pub status: feature::Status,
}
impl Feature {
    pub fn new(name: String, status: feature::Status) -> Self {
        Self { name, status }
    }
}
pub mod feature {
    use super::*;
    #[doc = "The feature support status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotAvailable,
        UnsupportedDeviceVersion,
        Supported,
    }
}
#[doc = "The OData filter to be used for features."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeatureFilter {
    #[doc = "Specifies the device ID for which the features are required. Only 'Equality' operator is supported for this property."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}
impl FeatureFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The collections of features."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureList {
    #[doc = "The value."]
    pub value: Vec<Feature>,
}
impl azure_core::Continuable for FeatureList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl FeatureList {
    pub fn new(value: Vec<Feature>) -> Self {
        Self { value }
    }
}
#[doc = "The hardware component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareComponent {
    #[doc = "The component ID."]
    #[serde(rename = "componentId")]
    pub component_id: String,
    #[doc = "The display name of the hardware component."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The status of the hardware component."]
    pub status: hardware_component::Status,
    #[doc = "The display name of the status of hardware component."]
    #[serde(rename = "statusDisplayName")]
    pub status_display_name: String,
}
impl HardwareComponent {
    pub fn new(component_id: String, display_name: String, status: hardware_component::Status, status_display_name: String) -> Self {
        Self {
            component_id,
            display_name,
            status,
            status_display_name,
        }
    }
}
pub mod hardware_component {
    use super::*;
    #[doc = "The status of the hardware component."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unknown,
        NotPresent,
        PoweredOff,
        Ok,
        Recovering,
        Warning,
        Failure,
    }
}
#[doc = "The hardware component group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareComponentGroup {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of hardware component group."]
    pub properties: HardwareComponentGroupProperties,
}
impl HardwareComponentGroup {
    pub fn new(properties: HardwareComponentGroupProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The collection of hardware component groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareComponentGroupList {
    #[doc = "The value."]
    pub value: Vec<HardwareComponentGroup>,
}
impl azure_core::Continuable for HardwareComponentGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl HardwareComponentGroupList {
    pub fn new(value: Vec<HardwareComponentGroup>) -> Self {
        Self { value }
    }
}
#[doc = "The properties of hardware component group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareComponentGroupProperties {
    #[doc = "The display name the hardware component group."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The last updated time."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339")]
    pub last_updated_time: time::OffsetDateTime,
    #[doc = "The list of hardware components."]
    pub components: Vec<HardwareComponent>,
}
impl HardwareComponentGroupProperties {
    pub fn new(display_name: String, last_updated_time: time::OffsetDateTime, components: Vec<HardwareComponent>) -> Self {
        Self {
            display_name,
            last_updated_time,
            components,
        }
    }
}
#[doc = "The job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The current status of the job."]
    pub status: job::Status,
    #[doc = "The UTC time at which the job was started."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC time at which the job completed."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The percentage of the job that is already complete."]
    #[serde(rename = "percentComplete")]
    pub percent_complete: i32,
    #[doc = "The job error details. Contains list of job error items."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JobErrorDetails>,
    #[doc = "The properties of the job."]
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
    #[doc = "The current status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Running,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[doc = "The job error details. Contains list of job error items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobErrorDetails {
    #[doc = "The error details."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<JobErrorItem>,
    #[doc = "The error code intended for programmatic access."]
    pub code: String,
    #[doc = "The error message intended to describe the error in detail."]
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
    #[doc = "The error code intended for programmatic access."]
    pub code: String,
    #[doc = "The error message intended to describe the error in detail."]
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
#[doc = "The OData filter to be used for jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobFilter {
    #[doc = "Specifies the status of the jobs to be filtered. For e.g., \"Running\", \"Succeeded\", \"Failed\" or \"Canceled\". Only 'Equality' operator is supported for this property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Specifies the type of the jobs to be filtered. For e.g., \"ScheduledBackup\", \"ManualBackup\", \"RestoreBackup\", \"CloneVolume\", \"FailoverVolumeContainers\", \"CreateLocallyPinnedVolume\", \"ModifyVolume\", \"InstallUpdates\", \"SupportPackageLogs\", or \"CreateCloudAppliance\". Only 'Equality' operator can be used for this property."]
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[doc = "Specifies the start time of the jobs to be filtered.  Only 'Greater Than or Equal To' and 'Lesser Than or Equal To' operators are supported for this property."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl JobFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The collection of jobs."]
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
#[doc = "The properties of the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[doc = "The type of the job."]
    #[serde(rename = "jobType")]
    pub job_type: job_properties::JobType,
    #[doc = "The additional details related to the data related statistics of a job. Currently applicable only for Backup, Clone and Restore jobs."]
    #[serde(rename = "dataStats", default, skip_serializing_if = "Option::is_none")]
    pub data_stats: Option<DataStatistics>,
    #[doc = "The entity identifier for which the job ran."]
    #[serde(rename = "entityLabel", default, skip_serializing_if = "Option::is_none")]
    pub entity_label: Option<String>,
    #[doc = "The entity type for which the job ran."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The job stages."]
    #[serde(rename = "jobStages", default, skip_serializing_if = "Vec::is_empty")]
    pub job_stages: Vec<JobStage>,
    #[doc = "The device ID in which the job ran."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Represents whether the job is cancellable or not."]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
    #[doc = "The backup type (CloudSnapshot | LocalSnapshot). Applicable only for backup jobs."]
    #[serde(rename = "backupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<job_properties::BackupType>,
    #[doc = "The source device ID of the failover job."]
    #[serde(rename = "sourceDeviceId", default, skip_serializing_if = "Option::is_none")]
    pub source_device_id: Option<String>,
    #[doc = "The time of the backup used for the failover."]
    #[serde(rename = "backupPointInTime", with = "azure_core::date::rfc3339::option")]
    pub backup_point_in_time: Option<time::OffsetDateTime>,
}
impl JobProperties {
    pub fn new(job_type: job_properties::JobType) -> Self {
        Self {
            job_type,
            data_stats: None,
            entity_label: None,
            entity_type: None,
            job_stages: Vec::new(),
            device_id: None,
            is_cancellable: None,
            backup_type: None,
            source_device_id: None,
            backup_point_in_time: None,
        }
    }
}
pub mod job_properties {
    use super::*;
    #[doc = "The type of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobType {
        ScheduledBackup,
        ManualBackup,
        RestoreBackup,
        CloneVolume,
        FailoverVolumeContainers,
        CreateLocallyPinnedVolume,
        ModifyVolume,
        InstallUpdates,
        SupportPackageLogs,
        CreateCloudAppliance,
    }
    #[doc = "The backup type (CloudSnapshot | LocalSnapshot). Applicable only for backup jobs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupType {
        LocalSnapshot,
        CloudSnapshot,
    }
}
#[doc = "The details about the specific stage of a job."]
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
        Running,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[doc = "The key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Key {
    #[doc = "The activation key for the device."]
    #[serde(rename = "activationKey")]
    pub activation_key: String,
}
impl Key {
    pub fn new(activation_key: String) -> Self {
        Self { activation_key }
    }
}
#[doc = "The request object for fetching the list of failover targets (eligible devices for failover)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListFailoverTargetsRequest {
    #[doc = "The list of path IDs of the volume containers that needs to be failed-over, for which we want to fetch the eligible targets."]
    #[serde(rename = "volumeContainers", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_containers: Vec<String>,
}
impl ListFailoverTargetsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The StorSimple Manager."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manager {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the StorSimple Manager."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagerProperties>,
    #[doc = "The etag of the manager."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagerExtendedInfo {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the manager extended info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagerExtendedInfoProperties>,
    #[doc = "The etag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ManagerExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the manager extended info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagerExtendedInfoProperties {
    #[doc = "The version of the extended info being persisted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Represents the CIK of the resource."]
    #[serde(rename = "integrityKey")]
    pub integrity_key: String,
    #[doc = "Represents the CEK of the resource."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[doc = "Represents the Cert thumbprint that was used to encrypt the CEK."]
    #[serde(rename = "encryptionKeyThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_thumbprint: Option<String>,
    #[doc = "Represents the portal thumbprint which can be used optionally to encrypt the entire data before storing it."]
    #[serde(rename = "portalCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub portal_certificate_thumbprint: Option<String>,
    #[doc = "Represents the encryption algorithm used to encrypt the keys. None - if Key is saved in plain text format. Algorithm name - if key is encrypted"]
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
#[doc = "Intrinsic settings which refers to the type of the StorSimple Manager."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagerIntrinsicSettings {
    #[doc = "The type of StorSimple Manager."]
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
    #[doc = "The type of StorSimple Manager."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        GardaV1,
        HelsinkiV1,
    }
}
#[doc = "The list of StorSimple Managers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagerList {
    #[doc = "The list of StorSimple managers."]
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
    #[doc = "The tags attached to the Manager."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagerPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the StorSimple Manager."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagerProperties {
    #[doc = "Intrinsic settings which refers to the type of the StorSimple Manager."]
    #[serde(rename = "cisIntrinsicSettings", default, skip_serializing_if = "Option::is_none")]
    pub cis_intrinsic_settings: Option<ManagerIntrinsicSettings>,
    #[doc = "The Sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ManagerSku>,
    #[doc = "Specifies the state of the resource as it is getting provisioned. Value of \"Succeeded\" means the Manager was successfully created."]
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
#[doc = "The metric availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailablity {
    #[doc = "The aggregation interval for the metric."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "The retention period for the metric at the specified timegrain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
}
impl MetricAvailablity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricData {
    #[doc = "The time stamp of the metric data."]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339::option")]
    pub time_stamp: Option<time::OffsetDateTime>,
    #[doc = "The sum of all samples at the time stamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[doc = "The count of all samples at the time stamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "The average of all samples at the time stamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[doc = "The minimum of all samples at the time stamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[doc = "The maximum of all samples at the time stamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
}
impl MetricData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The monitoring metric definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDefinition {
    #[doc = "The metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The metric unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<metric_definition::Unit>,
    #[doc = "The metric aggregation type."]
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<metric_definition::PrimaryAggregationType>,
    #[doc = "The metric source ID."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The available metric granularities."]
    #[serde(rename = "metricAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_availabilities: Vec<MetricAvailablity>,
    #[doc = "The available metric dimensions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "The category of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The metric definition type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl MetricDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metric_definition {
    use super::*;
    #[doc = "The metric unit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Bytes,
        BytesPerSecond,
        Count,
        CountPerSecond,
        Percent,
        Seconds,
    }
    #[doc = "The metric aggregation type."]
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
#[doc = "The list of metric definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDefinitionList {
    #[doc = "The list of metric definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetricDefinition>,
}
impl azure_core::Continuable for MetricDefinitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric dimension. It indicates the source of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDimension {
    #[doc = "The metric dimension name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The metric dimension values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl MetricDimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The OData filters to be used for metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricFilter {
    #[doc = "The metric name filter, specifying the name of the metric to be filtered on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricNameFilter>,
    #[doc = "Specifies the start time of the time range to be queried. Only 'Greater Than Or Equal To' operator is supported for this property."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Specifies the end time of the time range to be queried. Only 'Less Than Or Equal To' operator is supported for this property."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Specifies the time granularity of the metrics to be returned. E.g., \"P1D\". Valid values are the ones returned as the field \"timeGrain\" in the ListMetricDefinitions call. Only 'Equality' operator is supported for this property."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "Specifies the category of the metrics to be filtered. E.g., \"CapacityUtilization\". Valid values are the ones returned as the field \"category\" in the ListMetricDefinitions call. Only 'Equality' operator is supported for this property."]
    pub category: String,
    #[doc = "The dimension filter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionFilter>,
}
impl MetricFilter {
    pub fn new(category: String) -> Self {
        Self {
            name: None,
            start_time: None,
            end_time: None,
            time_grain: None,
            category,
            dimensions: None,
        }
    }
}
#[doc = "The metric list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricList {
    #[doc = "The value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Metrics>,
}
impl azure_core::Continuable for MetricList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricName {
    #[doc = "The metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized metric name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl MetricName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric name filter, specifying the name of the metric to be filtered on."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricNameFilter {
    #[doc = "Specifies the metric name to be filtered on. E.g., CloudStorageUsed. Valid values are the ones returned in the field \"name\" in the ListMetricDefinitions call. Only 'Equality' operator is supported for this property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl MetricNameFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The monitoring metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metrics {
    #[doc = "The ID of metric source."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The start time of the metric data."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the metric data."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The time granularity of the metric data."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "The metric aggregation type."]
    #[serde(rename = "primaryAggregation", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation: Option<metrics::PrimaryAggregation>,
    #[doc = "The metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The metric dimensions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "The unit of the metric data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<metrics::Unit>,
    #[doc = "The type of the metric data."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The list of the metric data."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<MetricData>,
}
impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metrics {
    use super::*;
    #[doc = "The metric aggregation type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrimaryAggregation {
        Average,
        Last,
        Maximum,
        Minimum,
        None,
        Total,
    }
    #[doc = "The unit of the metric data."]
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
#[doc = "The collection of network adapters on the device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkAdapterList {
    #[doc = "The value."]
    pub value: Vec<NetworkAdapters>,
}
impl NetworkAdapterList {
    pub fn new(value: Vec<NetworkAdapters>) -> Self {
        Self { value }
    }
}
#[doc = "Represents the network adapter on device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkAdapters {
    #[doc = "The ID of the network adapter."]
    #[serde(rename = "interfaceId")]
    pub interface_id: network_adapters::InterfaceId,
    #[doc = "Value indicating status of network adapter."]
    #[serde(rename = "netInterfaceStatus")]
    pub net_interface_status: network_adapters::NetInterfaceStatus,
    #[doc = "Value indicating whether this instance is default."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Value indicating cloud and ISCSI status of network adapter."]
    #[serde(rename = "iscsiAndCloudStatus")]
    pub iscsi_and_cloud_status: network_adapters::IscsiAndCloudStatus,
    #[doc = "The speed of the network adapter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[doc = "The mode of network adapter, either IPv4, IPv6 or both."]
    pub mode: network_adapters::Mode,
    #[doc = "Details related to the IPv4 address configuration."]
    #[serde(rename = "nicIpv4Settings", default, skip_serializing_if = "Option::is_none")]
    pub nic_ipv4_settings: Option<NicIPv4>,
    #[doc = "Details related to the IPv6 address configuration."]
    #[serde(rename = "nicIpv6Settings", default, skip_serializing_if = "Option::is_none")]
    pub nic_ipv6_settings: Option<NicIPv6>,
}
impl NetworkAdapters {
    pub fn new(
        interface_id: network_adapters::InterfaceId,
        net_interface_status: network_adapters::NetInterfaceStatus,
        iscsi_and_cloud_status: network_adapters::IscsiAndCloudStatus,
        mode: network_adapters::Mode,
    ) -> Self {
        Self {
            interface_id,
            net_interface_status,
            is_default: None,
            iscsi_and_cloud_status,
            speed: None,
            mode,
            nic_ipv4_settings: None,
            nic_ipv6_settings: None,
        }
    }
}
pub mod network_adapters {
    use super::*;
    #[doc = "The ID of the network adapter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InterfaceId {
        Invalid,
        Data0,
        Data1,
        Data2,
        Data3,
        Data4,
        Data5,
    }
    #[doc = "Value indicating status of network adapter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NetInterfaceStatus {
        Enabled,
        Disabled,
    }
    #[doc = "Value indicating cloud and ISCSI status of network adapter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IscsiAndCloudStatus {
        Disabled,
        IscsiEnabled,
        CloudEnabled,
        IscsiAndCloudEnabled,
    }
    #[doc = "The mode of network adapter, either IPv4, IPv6 or both."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Invalid,
        #[serde(rename = "IPV4")]
        Ipv4,
        #[serde(rename = "IPV6")]
        Ipv6,
        #[serde(rename = "BOTH")]
        Both,
    }
}
#[doc = "The 'Data 0' network interface card settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceData0Settings {
    #[doc = "The controller 0's IPv4 address."]
    #[serde(rename = "controllerZeroIp", default, skip_serializing_if = "Option::is_none")]
    pub controller_zero_ip: Option<String>,
    #[doc = "The controller 1's IPv4 address."]
    #[serde(rename = "controllerOneIp", default, skip_serializing_if = "Option::is_none")]
    pub controller_one_ip: Option<String>,
}
impl NetworkInterfaceData0Settings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the network settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the network settings of device."]
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
#[doc = "Represents the patch request for the network settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSettingsPatch {
    #[doc = "The properties of the network settings patch."]
    pub properties: NetworkSettingsPatchProperties,
}
impl NetworkSettingsPatch {
    pub fn new(properties: NetworkSettingsPatchProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The properties of the network settings patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSettingsPatchProperties {
    #[doc = "The DNS(Domain Name Server) settings of a device."]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<DnsSettings>,
    #[doc = "The collection of network adapters on the device."]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Option::is_none")]
    pub network_adapters: Option<NetworkAdapterList>,
}
impl NetworkSettingsPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the network settings of device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSettingsProperties {
    #[doc = "The DNS(Domain Name Server) settings of a device."]
    #[serde(rename = "dnsSettings")]
    pub dns_settings: DnsSettings,
    #[doc = "The collection of network adapters on the device."]
    #[serde(rename = "networkAdapters")]
    pub network_adapters: NetworkAdapterList,
    #[doc = "The web proxy settings on the device."]
    #[serde(rename = "webproxySettings")]
    pub webproxy_settings: WebproxySettings,
}
impl NetworkSettingsProperties {
    pub fn new(dns_settings: DnsSettings, network_adapters: NetworkAdapterList, webproxy_settings: WebproxySettings) -> Self {
        Self {
            dns_settings,
            network_adapters,
            webproxy_settings,
        }
    }
}
#[doc = "Details related to the IPv4 address configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NicIPv4 {
    #[doc = "The IPv4 address of the network adapter."]
    #[serde(rename = "ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[doc = "The IPv4 netmask of the network adapter."]
    #[serde(rename = "ipv4Netmask", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_netmask: Option<String>,
    #[doc = "The IPv4 gateway of the network adapter."]
    #[serde(rename = "ipv4Gateway", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_gateway: Option<String>,
    #[doc = "The IPv4 address of Controller0."]
    #[serde(rename = "controller0Ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub controller0_ipv4_address: Option<String>,
    #[doc = "The IPv4 address of Controller1."]
    #[serde(rename = "controller1Ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub controller1_ipv4_address: Option<String>,
}
impl NicIPv4 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details related to the IPv6 address configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NicIPv6 {
    #[doc = "The IPv6 address of the network adapter."]
    #[serde(rename = "ipv6Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[doc = "The IPv6 prefix of the network adapter."]
    #[serde(rename = "ipv6Prefix", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_prefix: Option<String>,
    #[doc = "The IPv6 gateway of the network adapter."]
    #[serde(rename = "ipv6Gateway", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_gateway: Option<String>,
    #[doc = "The IPv6 address of Controller0."]
    #[serde(rename = "controller0Ipv6Address", default, skip_serializing_if = "Option::is_none")]
    pub controller0_ipv6_address: Option<String>,
    #[doc = "The IPv6 address of Controller1."]
    #[serde(rename = "controller1Ipv6Address", default, skip_serializing_if = "Option::is_none")]
    pub controller1_ipv6_address: Option<String>,
}
impl NicIPv6 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The public key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKey {
    #[doc = "The key."]
    pub key: String,
}
impl PublicKey {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
#[doc = "The settings for remote management of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteManagementSettings {
    #[doc = "The remote management mode."]
    #[serde(rename = "remoteManagementMode")]
    pub remote_management_mode: remote_management_settings::RemoteManagementMode,
    #[doc = "The remote management certificates."]
    #[serde(rename = "remoteManagementCertificate", default, skip_serializing_if = "Option::is_none")]
    pub remote_management_certificate: Option<String>,
}
impl RemoteManagementSettings {
    pub fn new(remote_management_mode: remote_management_settings::RemoteManagementMode) -> Self {
        Self {
            remote_management_mode,
            remote_management_certificate: None,
        }
    }
}
pub mod remote_management_settings {
    use super::*;
    #[doc = "The remote management mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RemoteManagementMode {
        Unknown,
        Disabled,
        HttpsEnabled,
        HttpsAndHttpEnabled,
    }
}
#[doc = "The settings for updating remote management mode of the device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteManagementSettingsPatch {
    #[doc = "The remote management mode."]
    #[serde(rename = "remoteManagementMode")]
    pub remote_management_mode: remote_management_settings_patch::RemoteManagementMode,
}
impl RemoteManagementSettingsPatch {
    pub fn new(remote_management_mode: remote_management_settings_patch::RemoteManagementMode) -> Self {
        Self { remote_management_mode }
    }
}
pub mod remote_management_settings_patch {
    use super::*;
    #[doc = "The remote management mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RemoteManagementMode {
        Unknown,
        Disabled,
        HttpsEnabled,
        HttpsAndHttpEnabled,
    }
}
#[doc = "The Azure Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The geo location of the resource."]
    pub location: String,
    #[doc = "The tags attached to the resource."]
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
#[doc = "The schedule recurrence."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleRecurrence {
    #[doc = "The recurrence type."]
    #[serde(rename = "recurrenceType")]
    pub recurrence_type: schedule_recurrence::RecurrenceType,
    #[doc = "The recurrence value."]
    #[serde(rename = "recurrenceValue")]
    pub recurrence_value: i32,
    #[doc = "The week days list. Applicable only for schedules of recurrence type 'weekly'."]
    #[serde(rename = "weeklyDaysList", default, skip_serializing_if = "Vec::is_empty")]
    pub weekly_days_list: Vec<String>,
}
impl ScheduleRecurrence {
    pub fn new(recurrence_type: schedule_recurrence::RecurrenceType, recurrence_value: i32) -> Self {
        Self {
            recurrence_type,
            recurrence_value,
            weekly_days_list: Vec::new(),
        }
    }
}
pub mod schedule_recurrence {
    use super::*;
    #[doc = "The recurrence type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RecurrenceType {
        Minutes,
        Hourly,
        Daily,
        Weekly,
    }
}
#[doc = "The secondary DNS settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecondaryDnsSettings {
    #[doc = "The list of secondary DNS Server IP addresses."]
    #[serde(rename = "secondaryDnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub secondary_dns_servers: Vec<String>,
}
impl SecondaryDnsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The security settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of security settings of a device."]
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
#[doc = "Represents the patch request for the security settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySettingsPatch {
    #[doc = "The properties of the security settings patch."]
    pub properties: SecuritySettingsPatchProperties,
}
impl SecuritySettingsPatch {
    pub fn new(properties: SecuritySettingsPatchProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The properties of the security settings patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecuritySettingsPatchProperties {
    #[doc = "The settings for updating remote management mode of the device."]
    #[serde(rename = "remoteManagementSettings", default, skip_serializing_if = "Option::is_none")]
    pub remote_management_settings: Option<RemoteManagementSettingsPatch>,
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "deviceAdminPassword", default, skip_serializing_if = "Option::is_none")]
    pub device_admin_password: Option<AsymmetricEncryptedSecret>,
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "snapshotPassword", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_password: Option<AsymmetricEncryptedSecret>,
    #[doc = "The Challenge-Handshake Authentication Protocol (CHAP) settings."]
    #[serde(rename = "chapSettings", default, skip_serializing_if = "Option::is_none")]
    pub chap_settings: Option<ChapSettings>,
    #[doc = "The cloud appliance settings."]
    #[serde(rename = "cloudApplianceSettings", default, skip_serializing_if = "Option::is_none")]
    pub cloud_appliance_settings: Option<CloudApplianceSettings>,
}
impl SecuritySettingsPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of security settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySettingsProperties {
    #[doc = "The settings for remote management of a device."]
    #[serde(rename = "remoteManagementSettings")]
    pub remote_management_settings: RemoteManagementSettings,
    #[doc = "The Challenge-Handshake Authentication Protocol (CHAP) settings."]
    #[serde(rename = "chapSettings")]
    pub chap_settings: ChapSettings,
}
impl SecuritySettingsProperties {
    pub fn new(remote_management_settings: RemoteManagementSettings, chap_settings: ChapSettings) -> Self {
        Self {
            remote_management_settings,
            chap_settings,
        }
    }
}
#[doc = "The request for sending test alert email"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendTestAlertEmailRequest {
    #[doc = "The list of email IDs to send the test alert email"]
    #[serde(rename = "emailList")]
    pub email_list: Vec<String>,
}
impl SendTestAlertEmailRequest {
    pub fn new(email_list: Vec<String>) -> Self {
        Self { email_list }
    }
}
#[doc = "The storage account credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCredential {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The storage account credential properties."]
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
#[doc = "The collection of storage account credential entities."]
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
#[doc = "The storage account credential properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCredentialProperties {
    #[doc = "The storage endpoint"]
    #[serde(rename = "endPoint")]
    pub end_point: String,
    #[doc = "Signifies whether SSL needs to be enabled or not."]
    #[serde(rename = "sslStatus")]
    pub ssl_status: storage_account_credential_properties::SslStatus,
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<AsymmetricEncryptedSecret>,
    #[doc = "The count of volumes using this storage account credential."]
    #[serde(rename = "volumesCount", default, skip_serializing_if = "Option::is_none")]
    pub volumes_count: Option<i32>,
}
impl StorageAccountCredentialProperties {
    pub fn new(end_point: String, ssl_status: storage_account_credential_properties::SslStatus) -> Self {
        Self {
            end_point,
            ssl_status,
            access_key: None,
            volumes_count: None,
        }
    }
}
pub mod storage_account_credential_properties {
    use super::*;
    #[doc = "Signifies whether SSL needs to be enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SslStatus {
        Enabled,
        Disabled,
    }
}
#[doc = "Represents the secrets encrypted using Symmetric Encryption Key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymmetricEncryptedSecret {
    #[doc = "The value of the secret itself. If the secret is in plaintext or null then EncryptionAlgorithm will be none."]
    pub value: String,
    #[doc = "The thumbprint of the cert that was used to encrypt \"Value\"."]
    #[serde(rename = "valueCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub value_certificate_thumbprint: Option<String>,
    #[doc = "The algorithm used to encrypt the \"Value\"."]
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
    #[doc = "The algorithm used to encrypt the \"Value\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionAlgorithm {
        None,
        #[serde(rename = "AES256")]
        Aes256,
        #[serde(rename = "RSAES_PKCS1_v_1_5")]
        RsaesPkcs1V15,
    }
}
#[doc = "The error/warning message due to which the device is ineligible as a failover target device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetEligibilityErrorMessage {
    #[doc = "The localized error message stating the reason why the device is not eligible as a target device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The localized resolution message for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[doc = "The result code for the error, due to which the device does not qualify as a failover target device."]
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<target_eligibility_error_message::ResultCode>,
}
impl TargetEligibilityErrorMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod target_eligibility_error_message {
    use super::*;
    #[doc = "The result code for the error, due to which the device does not qualify as a failover target device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultCode {
        TargetAndSourceCannotBeSameError,
        TargetIsNotOnlineError,
        TargetSourceIncompatibleVersionError,
        LocalToTieredVolumesConversionWarning,
        TargetInsufficientCapacityError,
        TargetInsufficientLocalVolumeMemoryError,
        TargetInsufficientTieredVolumeMemoryError,
    }
}
#[doc = "The eligibility result of device, as a failover target device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetEligibilityResult {
    #[doc = "The eligibility status of device, as a failover target device."]
    #[serde(rename = "eligibilityStatus", default, skip_serializing_if = "Option::is_none")]
    pub eligibility_status: Option<target_eligibility_result::EligibilityStatus>,
    #[doc = "The list of error messages, if a device does not qualify as a failover target device."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<TargetEligibilityErrorMessage>,
}
impl TargetEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod target_eligibility_result {
    use super::*;
    #[doc = "The eligibility status of device, as a failover target device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EligibilityStatus {
        NotEligible,
        Eligible,
    }
}
#[doc = "The time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Time {
    #[doc = "The hour."]
    pub hours: i32,
    #[doc = "The minute."]
    pub minutes: i32,
    #[doc = "The second."]
    pub seconds: i32,
}
impl Time {
    pub fn new(hours: i32, minutes: i32, seconds: i32) -> Self {
        Self { hours, minutes, seconds }
    }
}
#[doc = "The time settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSettings {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of time settings of a device."]
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
#[doc = "The properties of time settings of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSettingsProperties {
    #[doc = "The timezone of device, like '(UTC -06:00) Central America'"]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[doc = "The primary Network Time Protocol (NTP) server name, like 'time.windows.com'."]
    #[serde(rename = "primaryTimeServer", default, skip_serializing_if = "Option::is_none")]
    pub primary_time_server: Option<String>,
    #[doc = "The secondary Network Time Protocol (NTP) server name, like 'time.contoso.com'. It's optional."]
    #[serde(rename = "secondaryTimeServer", default, skip_serializing_if = "Vec::is_empty")]
    pub secondary_time_server: Vec<String>,
}
impl TimeSettingsProperties {
    pub fn new(time_zone: String) -> Self {
        Self {
            time_zone,
            primary_time_server: None,
            secondary_time_server: Vec::new(),
        }
    }
}
#[doc = "The updates profile of a device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Updates {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of the updates profile."]
    pub properties: UpdatesProperties,
}
impl Updates {
    pub fn new(properties: UpdatesProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The properties of the updates profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdatesProperties {
    #[doc = "Set to 'true' if regular updates are available for the device."]
    #[serde(rename = "regularUpdatesAvailable", default, skip_serializing_if = "Option::is_none")]
    pub regular_updates_available: Option<bool>,
    #[doc = "Set to 'true' if maintenance mode update available."]
    #[serde(rename = "maintenanceModeUpdatesAvailable", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_mode_updates_available: Option<bool>,
    #[doc = "Indicates whether an update is in progress or not."]
    #[serde(rename = "isUpdateInProgress", default, skip_serializing_if = "Option::is_none")]
    pub is_update_in_progress: Option<bool>,
    #[doc = "The time when the last update was completed."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
}
impl UpdatesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The virtual machine image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmImage {
    #[doc = "The name."]
    pub name: String,
    #[doc = "The version."]
    pub version: String,
    #[doc = "The offer."]
    pub offer: String,
    #[doc = "The publisher."]
    pub publisher: String,
    #[doc = "The SKU."]
    pub sku: String,
}
impl VmImage {
    pub fn new(name: String, version: String, offer: String, publisher: String, sku: String) -> Self {
        Self {
            name,
            version,
            offer,
            publisher,
            sku,
        }
    }
}
#[doc = "The volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of volume."]
    pub properties: VolumeProperties,
}
impl Volume {
    pub fn new(properties: VolumeProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The volume container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeContainer {
    #[serde(flatten)]
    pub base_model: BaseModel,
    #[doc = "The properties of volume container."]
    pub properties: VolumeContainerProperties,
}
impl VolumeContainer {
    pub fn new(properties: VolumeContainerProperties) -> Self {
        Self {
            base_model: BaseModel::default(),
            properties,
        }
    }
}
#[doc = "The metadata of the volume container, that is being considered as part of a failover set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeContainerFailoverMetadata {
    #[doc = "The path ID of the volume container."]
    #[serde(rename = "volumeContainerId", default, skip_serializing_if = "Option::is_none")]
    pub volume_container_id: Option<String>,
    #[doc = "The list of metadata of volumes inside the volume container, which contains valid cloud snapshots."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<VolumeFailoverMetadata>,
}
impl VolumeContainerFailoverMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The collection of volume container entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeContainerList {
    #[doc = "The value."]
    pub value: Vec<VolumeContainer>,
}
impl azure_core::Continuable for VolumeContainerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl VolumeContainerList {
    pub fn new(value: Vec<VolumeContainer>) -> Self {
        Self { value }
    }
}
#[doc = "The properties of volume container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeContainerProperties {
    #[doc = "Represent the secrets intended for encryption with asymmetric key pair."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<AsymmetricEncryptedSecret>,
    #[doc = "The flag to denote whether encryption is enabled or not."]
    #[serde(rename = "encryptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub encryption_status: Option<volume_container_properties::EncryptionStatus>,
    #[doc = "The number of volumes in the volume Container."]
    #[serde(rename = "volumeCount", default, skip_serializing_if = "Option::is_none")]
    pub volume_count: Option<i32>,
    #[doc = "The path ID of storage account associated with the volume container."]
    #[serde(rename = "storageAccountCredentialId")]
    pub storage_account_credential_id: String,
    #[doc = "The owner ship status of the volume container. Only when the status is \"NotOwned\", the delete operation on the volume container is permitted."]
    #[serde(rename = "ownerShipStatus", default, skip_serializing_if = "Option::is_none")]
    pub owner_ship_status: Option<volume_container_properties::OwnerShipStatus>,
    #[doc = "The bandwidth-rate set on the volume container."]
    #[serde(rename = "bandWidthRateInMbps", default, skip_serializing_if = "Option::is_none")]
    pub band_width_rate_in_mbps: Option<i32>,
    #[doc = "The ID of the bandwidth setting associated with the volume container."]
    #[serde(rename = "bandwidthSettingId", default, skip_serializing_if = "Option::is_none")]
    pub bandwidth_setting_id: Option<String>,
    #[doc = "The total cloud storage for the volume container."]
    #[serde(rename = "totalCloudStorageUsageInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_cloud_storage_usage_in_bytes: Option<i64>,
}
impl VolumeContainerProperties {
    pub fn new(storage_account_credential_id: String) -> Self {
        Self {
            encryption_key: None,
            encryption_status: None,
            volume_count: None,
            storage_account_credential_id,
            owner_ship_status: None,
            band_width_rate_in_mbps: None,
            bandwidth_setting_id: None,
            total_cloud_storage_usage_in_bytes: None,
        }
    }
}
pub mod volume_container_properties {
    use super::*;
    #[doc = "The flag to denote whether encryption is enabled or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionStatus {
        Enabled,
        Disabled,
    }
    #[doc = "The owner ship status of the volume container. Only when the status is \"NotOwned\", the delete operation on the volume container is permitted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OwnerShipStatus {
        Owned,
        NotOwned,
    }
}
#[doc = "The metadata of a volume that has valid cloud snapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeFailoverMetadata {
    #[doc = "The path ID of the volume."]
    #[serde(rename = "volumeId", default, skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    #[doc = "The type of the volume."]
    #[serde(rename = "volumeType", default, skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<volume_failover_metadata::VolumeType>,
    #[doc = "The size of the volume in bytes at the time the snapshot was taken."]
    #[serde(rename = "sizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[doc = "The date at which the snapshot was taken."]
    #[serde(rename = "backupCreatedDate", with = "azure_core::date::rfc3339::option")]
    pub backup_created_date: Option<time::OffsetDateTime>,
    #[doc = "The path ID of the backup-element for this volume, inside the backup set."]
    #[serde(rename = "backupElementId", default, skip_serializing_if = "Option::is_none")]
    pub backup_element_id: Option<String>,
    #[doc = "The path ID of the backup set."]
    #[serde(rename = "backupId", default, skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[doc = "The path ID of the backup policy using which the snapshot was taken."]
    #[serde(rename = "backupPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy_id: Option<String>,
}
impl VolumeFailoverMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod volume_failover_metadata {
    use super::*;
    #[doc = "The type of the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VolumeType {
        Tiered,
        Archival,
        LocallyPinned,
    }
}
#[doc = "The collection of volumes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeList {
    #[doc = "The value."]
    pub value: Vec<Volume>,
}
impl azure_core::Continuable for VolumeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl VolumeList {
    pub fn new(value: Vec<Volume>) -> Self {
        Self { value }
    }
}
#[doc = "The properties of volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[doc = "The size of the volume in bytes."]
    #[serde(rename = "sizeInBytes")]
    pub size_in_bytes: i64,
    #[doc = "The type of the volume."]
    #[serde(rename = "volumeType")]
    pub volume_type: volume_properties::VolumeType,
    #[doc = "The ID of the volume container, in which this volume is created."]
    #[serde(rename = "volumeContainerId", default, skip_serializing_if = "Option::is_none")]
    pub volume_container_id: Option<String>,
    #[doc = "The IDs of the access control records, associated with the volume."]
    #[serde(rename = "accessControlRecordIds")]
    pub access_control_record_ids: Vec<String>,
    #[doc = "The volume status."]
    #[serde(rename = "volumeStatus")]
    pub volume_status: volume_properties::VolumeStatus,
    #[doc = "The operation status on the volume."]
    #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<volume_properties::OperationStatus>,
    #[doc = "The backup status of the volume."]
    #[serde(rename = "backupStatus", default, skip_serializing_if = "Option::is_none")]
    pub backup_status: Option<volume_properties::BackupStatus>,
    #[doc = "The monitoring status of the volume."]
    #[serde(rename = "monitoringStatus")]
    pub monitoring_status: volume_properties::MonitoringStatus,
    #[doc = "The IDs of the backup policies, in which this volume is part of."]
    #[serde(rename = "backupPolicyIds", default, skip_serializing_if = "Vec::is_empty")]
    pub backup_policy_ids: Vec<String>,
}
impl VolumeProperties {
    pub fn new(
        size_in_bytes: i64,
        volume_type: volume_properties::VolumeType,
        access_control_record_ids: Vec<String>,
        volume_status: volume_properties::VolumeStatus,
        monitoring_status: volume_properties::MonitoringStatus,
    ) -> Self {
        Self {
            size_in_bytes,
            volume_type,
            volume_container_id: None,
            access_control_record_ids,
            volume_status,
            operation_status: None,
            backup_status: None,
            monitoring_status,
            backup_policy_ids: Vec::new(),
        }
    }
}
pub mod volume_properties {
    use super::*;
    #[doc = "The type of the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VolumeType {
        Tiered,
        Archival,
        LocallyPinned,
    }
    #[doc = "The volume status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VolumeStatus {
        Online,
        Offline,
    }
    #[doc = "The operation status on the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationStatus {
        None,
        Updating,
        Deleting,
        Restoring,
    }
    #[doc = "The backup status of the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupStatus {
        Enabled,
        Disabled,
    }
    #[doc = "The monitoring status of the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitoringStatus {
        Enabled,
        Disabled,
    }
}
#[doc = "The web proxy settings on the device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebproxySettings {
    #[doc = "The connection URI."]
    #[serde(rename = "connectionUri", default, skip_serializing_if = "Option::is_none")]
    pub connection_uri: Option<String>,
    #[doc = "The authentication type."]
    pub authentication: webproxy_settings::Authentication,
    #[doc = "The webproxy username."]
    pub username: String,
}
impl WebproxySettings {
    pub fn new(authentication: webproxy_settings::Authentication, username: String) -> Self {
        Self {
            connection_uri: None,
            authentication,
            username,
        }
    }
}
pub mod webproxy_settings {
    use super::*;
    #[doc = "The authentication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Authentication {
        Invalid,
        None,
        Basic,
        #[serde(rename = "NTLM")]
        Ntlm,
    }
}
