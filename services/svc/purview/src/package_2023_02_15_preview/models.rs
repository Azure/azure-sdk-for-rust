#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "AdlsGen2 Sink"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2AccountSink {
    #[serde(flatten)]
    pub sink: Sink,
    #[doc = "Properties for AdlsGen2 storage account"]
    pub properties: AdlsGen2AccountSinkProperties,
}
impl AdlsGen2AccountSink {
    pub fn new(sink: Sink, properties: AdlsGen2AccountSinkProperties) -> Self {
        Self { sink, properties }
    }
}
#[doc = "Properties for AdlsGen2 storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2AccountSinkProperties {
    #[doc = "Adls Gen 2 Container Name"]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Adls Gen 2 Folder"]
    pub folder: String,
    #[doc = "Adls Gen 2 Location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Adls Gen 2 Mount Path"]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
}
impl AdlsGen2AccountSinkProperties {
    pub fn new(container_name: String, folder: String) -> Self {
        Self {
            container_name,
            folder,
            location: None,
            mount_path: None,
        }
    }
}
#[doc = "An Adls Gen2 storage account artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2Artifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    #[doc = "Properties for Adls Gen2 storage account."]
    pub properties: AdlsGen2ArtifactProperties,
}
impl AdlsGen2Artifact {
    pub fn new(artifact: Artifact, properties: AdlsGen2ArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[doc = "Properties for Adls Gen2 storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2ArtifactProperties {
    #[doc = "Location of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "A list of Adls Gen2 storage account paths."]
    pub paths: Vec<StorageAccountPath>,
}
impl AdlsGen2ArtifactProperties {
    pub fn new(paths: Vec<StorageAccountPath>) -> Self {
        Self { location: None, paths }
    }
}
#[doc = "A class for sent share artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[doc = "A Store Reference for an artifact or sink."]
    #[serde(rename = "storeReference")]
    pub store_reference: StoreReference,
}
impl Artifact {
    pub fn new(store_reference: StoreReference) -> Self {
        Self { store_reference }
    }
}
#[doc = "The types of asset."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "storeKind")]
pub enum ArtifactUnion {
    AdlsGen2Account(AdlsGen2Artifact),
    BlobAccount(BlobStorageArtifact),
}
#[doc = "Blob Sink"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobAccountSink {
    #[serde(flatten)]
    pub sink: Sink,
    #[doc = "Properties for blob storage account"]
    pub properties: BlobAccountSinkProperties,
}
impl BlobAccountSink {
    pub fn new(sink: Sink, properties: BlobAccountSinkProperties) -> Self {
        Self { sink, properties }
    }
}
#[doc = "Properties for blob storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobAccountSinkProperties {
    #[doc = "Blob Container Name"]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Blob Folder"]
    pub folder: String,
    #[doc = "Blob Location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Blob Mount Path"]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
}
impl BlobAccountSinkProperties {
    pub fn new(container_name: String, folder: String) -> Self {
        Self {
            container_name,
            folder,
            location: None,
            mount_path: None,
        }
    }
}
#[doc = "Blob storage account artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStorageArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    #[doc = "Properties for blob storage account."]
    pub properties: BlobStorageArtifactProperties,
}
impl BlobStorageArtifact {
    pub fn new(artifact: Artifact, properties: BlobStorageArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[doc = "Properties for blob storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStorageArtifactProperties {
    #[doc = "Location of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "A list of blob storage account paths."]
    pub paths: Vec<StorageAccountPath>,
}
impl BlobStorageArtifactProperties {
    pub fn new(paths: Vec<StorageAccountPath>) -> Self {
        Self { location: None, paths }
    }
}
#[doc = "An InPlace received share kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InPlaceReceivedShare {
    #[serde(flatten)]
    pub received_share: ReceivedShare,
    #[doc = "Properties of in place received share."]
    pub properties: InPlaceReceivedShareProperties,
}
impl InPlaceReceivedShare {
    pub fn new(received_share: ReceivedShare, properties: InPlaceReceivedShareProperties) -> Self {
        Self {
            received_share,
            properties,
        }
    }
}
#[doc = "Properties of in place received share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InPlaceReceivedShareProperties {
    #[doc = "Location of the shared Asset."]
    #[serde(rename = "assetLocation", default, skip_serializing_if = "Option::is_none")]
    pub asset_location: Option<String>,
    #[doc = "The types of asset."]
    #[serde(rename = "assetStoreKind", default, skip_serializing_if = "Option::is_none")]
    pub asset_store_kind: Option<StoreKind>,
    #[doc = "Time at which the received share was created. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Received Share Name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The expiration date of the received share. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Email of the user/receiver who received the sent share invitation"]
    #[serde(rename = "receiverEmail", default, skip_serializing_if = "Option::is_none")]
    pub receiver_email: Option<String>,
    #[doc = "Name of the user/receiver who received the sent share invitation"]
    #[serde(rename = "receiverName", default, skip_serializing_if = "Option::is_none")]
    pub receiver_name: Option<String>,
    #[doc = "Tenant name of the user/receiver who received the sent share invitation"]
    #[serde(rename = "receiverTenantName", default, skip_serializing_if = "Option::is_none")]
    pub receiver_tenant_name: Option<String>,
    #[doc = "Email of the sender who created the sent share invitation"]
    #[serde(rename = "senderEmail", default, skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[doc = "Name of the sender who created the sent share invitation"]
    #[serde(rename = "senderName", default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Tenant name of the sender who created the sent share invitation"]
    #[serde(rename = "senderTenantName", default, skip_serializing_if = "Option::is_none")]
    pub sender_tenant_name: Option<String>,
    #[doc = "Share description."]
    #[serde(rename = "sentShareDescription", default, skip_serializing_if = "Option::is_none")]
    pub sent_share_description: Option<String>,
    #[doc = "Time at which the sent share was shared. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "sharedAt", default, with = "azure_core::date::rfc3339::option")]
    pub shared_at: Option<time::OffsetDateTime>,
    #[doc = "Share status."]
    #[serde(rename = "shareStatus", default, skip_serializing_if = "Option::is_none")]
    pub share_status: Option<ShareStatus>,
    #[doc = "Holds details on the destination of the mapped artifact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sink: Option<SinkUnion>,
    #[doc = "State of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}
impl InPlaceReceivedShareProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An InPlace share kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InPlaceSentShare {
    #[serde(flatten)]
    pub sent_share: SentShare,
    #[doc = "Properties for InPlace sent share."]
    pub properties: InPlaceSentShareProperties,
}
impl InPlaceSentShare {
    pub fn new(sent_share: SentShare, properties: InPlaceSentShareProperties) -> Self {
        Self { sent_share, properties }
    }
}
#[doc = "Properties for InPlace sent share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InPlaceSentShareProperties {
    #[doc = "A class for sent share artifact."]
    pub artifact: ArtifactUnion,
    #[doc = "Time at which the sent share was created. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "List of shares on which the sent share depends."]
    #[serde(
        rename = "dependsOn",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub depends_on: Vec<String>,
    #[doc = "Sent share description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The name of the sent share"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Email of the sender who created the sent share."]
    #[serde(rename = "senderEmail", default, skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[doc = "Name of the sender who created the sent share."]
    #[serde(rename = "senderName", default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Tenant name of the sender who created the sent share."]
    #[serde(rename = "senderTenantName", default, skip_serializing_if = "Option::is_none")]
    pub sender_tenant_name: Option<String>,
    #[doc = "State of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}
impl InPlaceSentShareProperties {
    pub fn new(artifact: ArtifactUnion, display_name: String) -> Self {
        Self {
            artifact,
            created_at: None,
            depends_on: Vec::new(),
            description: None,
            display_name,
            sender_email: None,
            sender_name: None,
            sender_tenant_name: None,
            state: None,
        }
    }
}
#[doc = "The types of invitations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InvitationKind")]
pub enum InvitationKind {
    User,
    Service,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InvitationKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InvitationKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InvitationKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("InvitationKind", 0u32, "User"),
            Self::Service => serializer.serialize_unit_variant("InvitationKind", 1u32, "Service"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Response for long running operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResponse {
    #[doc = "End time of the long running operation. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The purview share error body model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PurviewShareErrorInfo>,
    #[doc = "Job id of the long running operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Start time of the long running operation. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "States for long running operations."]
    pub status: OperationStatus,
}
impl OperationResponse {
    pub fn new(status: OperationStatus) -> Self {
        Self {
            end_time: None,
            error: None,
            id: None,
            start_time: None,
            status,
        }
    }
}
#[doc = "States for long running operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationStatus")]
pub enum OperationStatus {
    Running,
    TransientFailure,
    Succeeded,
    Failed,
    NotStarted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Running => serializer.serialize_unit_variant("OperationStatus", 0u32, "Running"),
            Self::TransientFailure => serializer.serialize_unit_variant("OperationStatus", 1u32, "TransientFailure"),
            Self::Succeeded => serializer.serialize_unit_variant("OperationStatus", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("OperationStatus", 3u32, "Failed"),
            Self::NotStarted => serializer.serialize_unit_variant("OperationStatus", 4u32, "NotStarted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Base data transfer object implementation for proxy resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "The unique id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The purview share error model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurviewShareError {
    #[doc = "The purview share error body model."]
    pub error: PurviewShareErrorInfo,
}
impl azure_core::Continuable for PurviewShareError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PurviewShareError {
    pub fn new(error: PurviewShareErrorInfo) -> Self {
        Self { error }
    }
}
#[doc = "The purview share error body model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurviewShareErrorInfo {
    #[doc = "Code of the error"]
    pub code: String,
    #[doc = "Nested details of the error model"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<PurviewShareErrorInfo>,
    #[doc = "Message of the error"]
    pub message: String,
    #[doc = "Target of the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl PurviewShareErrorInfo {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            details: Vec::new(),
            message,
            target: None,
        }
    }
}
#[doc = "A received share data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceivedShare {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl ReceivedShare {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
        }
    }
}
#[doc = "Defines the supported types for share."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "shareKind")]
pub enum ReceivedShareUnion {
    InPlace(InPlaceReceivedShare),
}
#[doc = "List of received shares."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceivedShareList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ReceivedShareUnion>,
}
impl azure_core::Continuable for ReceivedShareList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReceivedShareList {
    pub fn new(value: Vec<ReceivedShareUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Defines the type of resource being shared"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReferenceNameType")]
pub enum ReferenceNameType {
    ArmResourceReference,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReferenceNameType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReferenceNameType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReferenceNameType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ArmResourceReference => serializer.serialize_unit_variant("ReferenceNameType", 0u32, "ArmResourceReference"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A sent share data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentShare {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl SentShare {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
        }
    }
}
#[doc = "Defines the supported types for share."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "shareKind")]
pub enum SentShareUnion {
    InPlace(InPlaceSentShare),
}
#[doc = "A sent share invitation data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentShareInvitation {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl SentShareInvitation {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
        }
    }
}
#[doc = "The types of invitations."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "invitationKind")]
pub enum SentShareInvitationUnion {
    Service(ServiceInvitation),
    User(UserInvitation),
}
#[doc = "List of the sent share invitations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentShareInvitationList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<SentShareInvitationUnion>,
}
impl azure_core::Continuable for SentShareInvitationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SentShareInvitationList {
    pub fn new(value: Vec<SentShareInvitationUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "List of sent shares."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentShareList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<SentShareUnion>,
}
impl azure_core::Continuable for SentShareList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SentShareList {
    pub fn new(value: Vec<SentShareUnion>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "An service invitation kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceInvitation {
    #[serde(flatten)]
    pub sent_share_invitation: SentShareInvitation,
    #[doc = "Properties of the service invitation type."]
    pub properties: ServiceInvitationProperties,
}
impl ServiceInvitation {
    pub fn new(sent_share_invitation: SentShareInvitation, properties: ServiceInvitationProperties) -> Self {
        Self {
            sent_share_invitation,
            properties,
        }
    }
}
#[doc = "Properties of the service invitation type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceInvitationProperties {
    #[doc = "The time at which the invitation will expire. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Email address of the sender."]
    #[serde(rename = "senderEmail", default, skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[doc = "Name of the sender"]
    #[serde(rename = "senderName", default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Tenant name of the sender"]
    #[serde(rename = "senderTenantName", default, skip_serializing_if = "Option::is_none")]
    pub sender_tenant_name: Option<String>,
    #[doc = "Gets the time at which the invitation was sent. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "sentAt", default, with = "azure_core::date::rfc3339::option")]
    pub sent_at: Option<time::OffsetDateTime>,
    #[doc = "Share status."]
    #[serde(rename = "shareStatus", default, skip_serializing_if = "Option::is_none")]
    pub share_status: Option<ShareStatus>,
    #[doc = "State of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[doc = "The target azure active directory id the invitation is sent to."]
    #[serde(rename = "targetActiveDirectoryId")]
    pub target_active_directory_id: String,
    #[doc = "The target object id in the azure active directory the invitation is sent to."]
    #[serde(rename = "targetObjectId")]
    pub target_object_id: String,
}
impl ServiceInvitationProperties {
    pub fn new(target_active_directory_id: String, target_object_id: String) -> Self {
        Self {
            expiration_date: None,
            sender_email: None,
            sender_name: None,
            sender_tenant_name: None,
            sent_at: None,
            share_status: None,
            state: None,
            target_active_directory_id,
            target_object_id,
        }
    }
}
#[doc = "Defines the supported types for share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ShareKind")]
pub enum ShareKind {
    InPlace,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ShareKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ShareKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ShareKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InPlace => serializer.serialize_unit_variant("ShareKind", 0u32, "InPlace"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Share status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ShareStatus")]
pub enum ShareStatus {
    Detached,
    Attached,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ShareStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ShareStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ShareStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Detached => serializer.serialize_unit_variant("ShareStatus", 0u32, "Detached"),
            Self::Attached => serializer.serialize_unit_variant("ShareStatus", 1u32, "Attached"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Holds details on the destination of the mapped artifact"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sink {
    #[doc = "A Store Reference for an artifact or sink."]
    #[serde(rename = "storeReference")]
    pub store_reference: StoreReference,
}
impl Sink {
    pub fn new(store_reference: StoreReference) -> Self {
        Self { store_reference }
    }
}
#[doc = "The types of asset."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "storeKind")]
pub enum SinkUnion {
    AdlsGen2Account(AdlsGen2AccountSink),
    BlobAccount(BlobAccountSink),
}
#[doc = "State of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "State")]
pub enum State {
    Unknown,
    Succeeded,
    Creating,
    Deleting,
    Moving,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for State {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("State", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("State", 1u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("State", 2u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("State", 3u32, "Deleting"),
            Self::Moving => serializer.serialize_unit_variant("State", 4u32, "Moving"),
            Self::Failed => serializer.serialize_unit_variant("State", 5u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines a single StorageAccountPath path"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountPath {
    #[doc = "Gets or sets the container name"]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Gets or sets the path on the receiver side where the artifact is to be mapped"]
    #[serde(rename = "receiverPath", default, skip_serializing_if = "Option::is_none")]
    pub receiver_path: Option<String>,
    #[doc = "Gets or sets the path to file/folder within the container to be shared"]
    #[serde(rename = "senderPath", default, skip_serializing_if = "Option::is_none")]
    pub sender_path: Option<String>,
}
impl StorageAccountPath {
    pub fn new(container_name: String) -> Self {
        Self {
            container_name,
            receiver_path: None,
            sender_path: None,
        }
    }
}
#[doc = "The types of asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StoreKind")]
pub enum StoreKind {
    AdlsGen2Account,
    BlobAccount,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StoreKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StoreKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StoreKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AdlsGen2Account => serializer.serialize_unit_variant("StoreKind", 0u32, "AdlsGen2Account"),
            Self::BlobAccount => serializer.serialize_unit_variant("StoreKind", 1u32, "BlobAccount"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A Store Reference for an artifact or sink."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StoreReference {
    #[doc = "Reference name for resource associated with the sink or artifact."]
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[doc = "Defines the type of resource being shared"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ReferenceNameType>,
}
impl StoreReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A tenant email registration data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantEmailRegistration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Tenant email registration property bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TenantEmailRegistrationProperties>,
}
impl TenantEmailRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant email registration property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantEmailRegistrationProperties {
    #[doc = "Activation code for the registration."]
    #[serde(rename = "activationCode")]
    pub activation_code: String,
    #[doc = "Date of the activation expiration. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "activationExpiration", default, with = "azure_core::date::rfc3339::option")]
    pub activation_expiration: Option<time::OffsetDateTime>,
    #[doc = "The email to register."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Defines the supported types for registration."]
    #[serde(rename = "registrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub registration_status: Option<TenantEmailRegistrationStatus>,
    #[doc = "State of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[doc = "The tenant id to register."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl TenantEmailRegistrationProperties {
    pub fn new(activation_code: String) -> Self {
        Self {
            activation_code,
            activation_expiration: None,
            email: None,
            registration_status: None,
            state: None,
            tenant_id: None,
        }
    }
}
#[doc = "Defines the supported types for registration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TenantEmailRegistrationStatus")]
pub enum TenantEmailRegistrationStatus {
    ActivationPending,
    Activated,
    ActivationAttemptsExhausted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TenantEmailRegistrationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TenantEmailRegistrationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TenantEmailRegistrationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ActivationPending => serializer.serialize_unit_variant("TenantEmailRegistrationStatus", 0u32, "ActivationPending"),
            Self::Activated => serializer.serialize_unit_variant("TenantEmailRegistrationStatus", 1u32, "Activated"),
            Self::ActivationAttemptsExhausted => {
                serializer.serialize_unit_variant("TenantEmailRegistrationStatus", 2u32, "ActivationAttemptsExhausted")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A user invitation kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitation {
    #[serde(flatten)]
    pub sent_share_invitation: SentShareInvitation,
    #[doc = "Properties of the user invitation type."]
    pub properties: UserInvitationProperties,
}
impl UserInvitation {
    pub fn new(sent_share_invitation: SentShareInvitation, properties: UserInvitationProperties) -> Self {
        Self {
            sent_share_invitation,
            properties,
        }
    }
}
#[doc = "Properties of the user invitation type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationProperties {
    #[doc = "The time at which the invitation will expire. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Whether or not the recipient was notified via email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
    #[doc = "Email address of the sender."]
    #[serde(rename = "senderEmail", default, skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[doc = "Name of the sender"]
    #[serde(rename = "senderName", default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Tenant name of the sender"]
    #[serde(rename = "senderTenantName", default, skip_serializing_if = "Option::is_none")]
    pub sender_tenant_name: Option<String>,
    #[doc = "Gets the time at which the invitation was sent. Represented in the standard date-time format as defined by [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)"]
    #[serde(rename = "sentAt", default, with = "azure_core::date::rfc3339::option")]
    pub sent_at: Option<time::OffsetDateTime>,
    #[doc = "Share status."]
    #[serde(rename = "shareStatus", default, skip_serializing_if = "Option::is_none")]
    pub share_status: Option<ShareStatus>,
    #[doc = "State of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[doc = "The receiver email for the invitation is being sent."]
    #[serde(rename = "targetEmail")]
    pub target_email: String,
}
impl UserInvitationProperties {
    pub fn new(target_email: String) -> Self {
        Self {
            expiration_date: None,
            notify: None,
            sender_email: None,
            sender_name: None,
            sender_tenant_name: None,
            sent_at: None,
            share_status: None,
            state: None,
            target_email,
        }
    }
}
