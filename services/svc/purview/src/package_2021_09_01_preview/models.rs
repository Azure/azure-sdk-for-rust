#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An accepted sent share data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcceptedSentShare {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Defines the supported types for share."]
    #[serde(rename = "shareKind")]
    pub share_kind: ShareKind,
}
impl AcceptedSentShare {
    pub fn new(share_kind: ShareKind) -> Self {
        Self {
            resource: Resource::default(),
            share_kind,
        }
    }
}
#[doc = "List of accepted sent shares."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcceptedSentShareList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<AcceptedSentShare>,
}
impl azure_core::Continuable for AcceptedSentShareList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AcceptedSentShareList {
    pub fn new(value: Vec<AcceptedSentShare>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "An adls gen2 storage account asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2AccountAsset {
    #[serde(flatten)]
    pub asset: Asset,
    #[doc = "Adls gen 2 account asset properties"]
    pub properties: AdlsGen2AccountAssetProperties,
}
impl AdlsGen2AccountAsset {
    pub fn new(asset: Asset, properties: AdlsGen2AccountAssetProperties) -> Self {
        Self { asset, properties }
    }
}
#[doc = "An blob storage account asset mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2AccountAssetMapping {
    #[serde(flatten)]
    pub asset_mapping: AssetMapping,
    #[doc = "Properties of the adls gen2 storage account asset mapping."]
    pub properties: AdlsGen2AccountAssetMappingProperties,
}
impl AdlsGen2AccountAssetMapping {
    pub fn new(asset_mapping: AssetMapping, properties: AdlsGen2AccountAssetMappingProperties) -> Self {
        Self { asset_mapping, properties }
    }
}
#[doc = "Properties of the adls gen2 storage account asset mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2AccountAssetMappingProperties {
    #[doc = "The id of the sender asset."]
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[doc = "State of asset mapping"]
    #[serde(rename = "assetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub asset_mapping_status: Option<AssetMappingStatus>,
    #[doc = "Name of the container to received the shared paths."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Folder under which the shared paths will be reflected."]
    pub folder: String,
    #[doc = "Location of the receiver storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Optional mount path for the shared paths."]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    #[doc = "Provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Resource id of the receiver storage account."]
    #[serde(rename = "storageAccountResourceId")]
    pub storage_account_resource_id: String,
}
impl AdlsGen2AccountAssetMappingProperties {
    pub fn new(asset_id: String, container_name: String, folder: String, storage_account_resource_id: String) -> Self {
        Self {
            asset_id,
            asset_mapping_status: None,
            container_name,
            folder,
            location: None,
            mount_path: None,
            provisioning_state: None,
            storage_account_resource_id,
        }
    }
}
#[doc = "Adls gen 2 account asset properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2AccountAssetProperties {
    #[doc = "Location of the adls gen2 storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "A list of adls gen2 storage account paths to be shared."]
    pub paths: Vec<StorageAccountPath>,
    #[doc = "Provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Name of the asset for the receiver."]
    #[serde(rename = "receiverAssetName")]
    pub receiver_asset_name: String,
    #[doc = "Resource id of the adls gen2 storage account."]
    #[serde(rename = "storageAccountResourceId")]
    pub storage_account_resource_id: String,
}
impl AdlsGen2AccountAssetProperties {
    pub fn new(paths: Vec<StorageAccountPath>, receiver_asset_name: String, storage_account_resource_id: String) -> Self {
        Self {
            location: None,
            paths,
            provisioning_state: None,
            receiver_asset_name,
            storage_account_resource_id,
        }
    }
}
#[doc = "Adls gen2 storage account received asset"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsGen2AccountReceivedAsset {
    #[serde(flatten)]
    pub received_asset: ReceivedAsset,
    #[doc = "Properties of adls gen2 account received asset."]
    pub properties: AdlsGen2AccountReceivedAssetProperties,
}
impl AdlsGen2AccountReceivedAsset {
    pub fn new(received_asset: ReceivedAsset, properties: AdlsGen2AccountReceivedAssetProperties) -> Self {
        Self {
            received_asset,
            properties,
        }
    }
}
#[doc = "Properties of adls gen2 account received asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdlsGen2AccountReceivedAssetProperties {
    #[doc = "Location of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Friendly name for the received asset."]
    #[serde(rename = "receiverAssetName", default, skip_serializing_if = "Option::is_none")]
    pub receiver_asset_name: Option<String>,
    #[doc = "Paths from adls gen2 account made available for the share."]
    #[serde(rename = "receiverPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub receiver_paths: Vec<String>,
}
impl AdlsGen2AccountReceivedAssetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An application invitation kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInvitation {
    #[serde(flatten)]
    pub sent_share_invitation: SentShareInvitation,
    #[doc = "Properties of the application invitation type."]
    pub properties: ApplicationInvitationProperties,
}
impl ApplicationInvitation {
    pub fn new(sent_share_invitation: SentShareInvitation, properties: ApplicationInvitationProperties) -> Self {
        Self {
            sent_share_invitation,
            properties,
        }
    }
}
#[doc = "Properties of the application invitation type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInvitationProperties {
    #[doc = "The expiration date for the invitation"]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Id of the invitation"]
    #[serde(rename = "invitationId", default, skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[doc = "Status of the invitation."]
    #[serde(rename = "invitationStatus", default, skip_serializing_if = "Option::is_none")]
    pub invitation_status: Option<InvitationStatus>,
    #[doc = "Provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The time the recipient responded to the invitation."]
    #[serde(rename = "respondedAt", default, with = "azure_core::date::rfc3339::option")]
    pub responded_at: Option<time::OffsetDateTime>,
    #[doc = "Email address of the sender."]
    #[serde(rename = "senderEmail", default, skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[doc = "Name of the sender"]
    #[serde(rename = "senderName", default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Tenant name of the sender"]
    #[serde(rename = "senderTenantName", default, skip_serializing_if = "Option::is_none")]
    pub sender_tenant_name: Option<String>,
    #[doc = "Gets the time at which the invitation was sent."]
    #[serde(rename = "sentAt", default, with = "azure_core::date::rfc3339::option")]
    pub sent_at: Option<time::OffsetDateTime>,
    #[doc = "Defines the supported types for share."]
    #[serde(rename = "shareKind", default, skip_serializing_if = "Option::is_none")]
    pub share_kind: Option<ShareKind>,
    #[doc = "The target azure active directory id the invitation is sent to."]
    #[serde(rename = "targetActiveDirectoryId")]
    pub target_active_directory_id: String,
    #[doc = "The target object id in the azure active directory the invitation is sent to."]
    #[serde(rename = "targetObjectId")]
    pub target_object_id: String,
}
impl ApplicationInvitationProperties {
    pub fn new(target_active_directory_id: String, target_object_id: String) -> Self {
        Self {
            expiration_date: None,
            invitation_id: None,
            invitation_status: None,
            provisioning_state: None,
            responded_at: None,
            sender_email: None,
            sender_name: None,
            sender_tenant_name: None,
            sent_at: None,
            share_kind: None,
            target_active_directory_id,
            target_object_id,
        }
    }
}
#[doc = "An application received invitation kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationReceivedInvitation {
    #[serde(flatten)]
    pub received_invitation: ReceivedInvitation,
    #[doc = "Properties for a received invitation of kind application."]
    pub properties: ApplicationReceivedInvitationProperties,
}
impl ApplicationReceivedInvitation {
    pub fn new(received_invitation: ReceivedInvitation, properties: ApplicationReceivedInvitationProperties) -> Self {
        Self {
            received_invitation,
            properties,
        }
    }
}
#[doc = "Properties for a received invitation of kind application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationReceivedInvitationProperties {
    #[doc = "Description shared when the invitation was created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The expiration date for the received share created by accepting the invitation."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Status of the invitation."]
    #[serde(rename = "invitationStatus", default, skip_serializing_if = "Option::is_none")]
    pub invitation_status: Option<InvitationStatus>,
    #[doc = "Location of the invitation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Email of the user/receiver who received the sent share invitation"]
    #[serde(rename = "receiverEmail", default, skip_serializing_if = "Option::is_none")]
    pub receiver_email: Option<String>,
    #[doc = "Name of the user/receiver who received the sent share invitation"]
    #[serde(rename = "receiverName", default, skip_serializing_if = "Option::is_none")]
    pub receiver_name: Option<String>,
    #[doc = "Tenant name of the user/receiver who received the sent share invitation"]
    #[serde(rename = "receiverTenantName", default, skip_serializing_if = "Option::is_none")]
    pub receiver_tenant_name: Option<String>,
    #[doc = "The time the recipient responded to the invitation."]
    #[serde(rename = "respondedAt", default, with = "azure_core::date::rfc3339::option")]
    pub responded_at: Option<time::OffsetDateTime>,
    #[doc = "Email of the sender who created the sent share invitation"]
    #[serde(rename = "senderEmail", default, skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[doc = "Name of the sender who created the sent share invitation"]
    #[serde(rename = "senderName", default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Tenant name of the sender who created the sent share invitation"]
    #[serde(rename = "senderTenantName", default, skip_serializing_if = "Option::is_none")]
    pub sender_tenant_name: Option<String>,
    #[doc = "Gets the time at which the invitation was sent."]
    #[serde(rename = "sentAt", default, with = "azure_core::date::rfc3339::option")]
    pub sent_at: Option<time::OffsetDateTime>,
    #[doc = "Gets the source share Name."]
    #[serde(rename = "sentShareName", default, skip_serializing_if = "Option::is_none")]
    pub sent_share_name: Option<String>,
    #[doc = "Defines the supported types for share."]
    #[serde(rename = "shareKind", default, skip_serializing_if = "Option::is_none")]
    pub share_kind: Option<ShareKind>,
    #[doc = "The target azure active directory id the invitation is sent to."]
    #[serde(rename = "targetActiveDirectoryId")]
    pub target_active_directory_id: String,
    #[doc = "The target object id in the azure active directory the invitation is sent to."]
    #[serde(rename = "targetObjectId")]
    pub target_object_id: String,
}
impl ApplicationReceivedInvitationProperties {
    pub fn new(target_active_directory_id: String, target_object_id: String) -> Self {
        Self {
            description: None,
            expiration_date: None,
            invitation_status: None,
            location: None,
            receiver_email: None,
            receiver_name: None,
            receiver_tenant_name: None,
            responded_at: None,
            sender_email: None,
            sender_name: None,
            sender_tenant_name: None,
            sent_at: None,
            sent_share_name: None,
            share_kind: None,
            target_active_directory_id,
            target_object_id,
        }
    }
}
#[doc = "A asset data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The types of asset."]
    pub kind: AssetType,
}
impl Asset {
    pub fn new(kind: AssetType) -> Self {
        Self {
            resource: Resource::default(),
            kind,
        }
    }
}
#[doc = "List of assets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<Asset>,
}
impl azure_core::Continuable for AssetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AssetList {
    pub fn new(value: Vec<Asset>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "An asset mapping data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetMapping {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Types of asset mapping."]
    pub kind: AssetMappingType,
}
impl AssetMapping {
    pub fn new(kind: AssetMappingType) -> Self {
        Self {
            resource: Resource::default(),
            kind,
        }
    }
}
#[doc = "List of asset mappings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetMappingList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<AssetMapping>,
}
impl azure_core::Continuable for AssetMappingList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AssetMappingList {
    pub fn new(value: Vec<AssetMapping>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "State of asset mapping"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssetMappingStatus")]
pub enum AssetMappingStatus {
    Ok,
    Broken,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssetMappingStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssetMappingStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssetMappingStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ok => serializer.serialize_unit_variant("AssetMappingStatus", 0u32, "Ok"),
            Self::Broken => serializer.serialize_unit_variant("AssetMappingStatus", 1u32, "Broken"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Types of asset mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssetMappingType")]
pub enum AssetMappingType {
    AdlsGen2Account,
    BlobAccount,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssetMappingType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssetMappingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssetMappingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AdlsGen2Account => serializer.serialize_unit_variant("AssetMappingType", 0u32, "AdlsGen2Account"),
            Self::BlobAccount => serializer.serialize_unit_variant("AssetMappingType", 1u32, "BlobAccount"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The types of asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssetType")]
pub enum AssetType {
    AdlsGen2Account,
    BlobAccount,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssetType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssetType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssetType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AdlsGen2Account => serializer.serialize_unit_variant("AssetType", 0u32, "AdlsGen2Account"),
            Self::BlobAccount => serializer.serialize_unit_variant("AssetType", 1u32, "BlobAccount"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A blob storage account asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobAccountAsset {
    #[serde(flatten)]
    pub asset: Asset,
    #[doc = "Blob storage account asset properties"]
    pub properties: BlobAccountAssetProperties,
}
impl BlobAccountAsset {
    pub fn new(asset: Asset, properties: BlobAccountAssetProperties) -> Self {
        Self { asset, properties }
    }
}
#[doc = "An adls gen 2 storage account asset mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobAccountAssetMapping {
    #[serde(flatten)]
    pub asset_mapping: AssetMapping,
    #[doc = "Properties of the blob storage account asset mapping."]
    pub properties: BlobAccountAssetMappingProperties,
}
impl BlobAccountAssetMapping {
    pub fn new(asset_mapping: AssetMapping, properties: BlobAccountAssetMappingProperties) -> Self {
        Self { asset_mapping, properties }
    }
}
#[doc = "Properties of the blob storage account asset mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobAccountAssetMappingProperties {
    #[doc = "The id of the sender asset."]
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[doc = "State of asset mapping"]
    #[serde(rename = "assetMappingStatus", default, skip_serializing_if = "Option::is_none")]
    pub asset_mapping_status: Option<AssetMappingStatus>,
    #[doc = "Name of the container to received the shared paths."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Folder under which the shared paths will be reflected."]
    pub folder: String,
    #[doc = "Location of the receiver storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Optional mount path for the shared paths."]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    #[doc = "Provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Resource id of the receiver storage account."]
    #[serde(rename = "storageAccountResourceId")]
    pub storage_account_resource_id: String,
}
impl BlobAccountAssetMappingProperties {
    pub fn new(asset_id: String, container_name: String, folder: String, storage_account_resource_id: String) -> Self {
        Self {
            asset_id,
            asset_mapping_status: None,
            container_name,
            folder,
            location: None,
            mount_path: None,
            provisioning_state: None,
            storage_account_resource_id,
        }
    }
}
#[doc = "Blob storage account asset properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobAccountAssetProperties {
    #[doc = "Location of the blob storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "A list of blob storage account paths to be shared."]
    pub paths: Vec<StorageAccountPath>,
    #[doc = "Provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Name of the asset for the receiver."]
    #[serde(rename = "receiverAssetName")]
    pub receiver_asset_name: String,
    #[doc = "Resource id of the blob storage account."]
    #[serde(rename = "storageAccountResourceId")]
    pub storage_account_resource_id: String,
}
impl BlobAccountAssetProperties {
    pub fn new(paths: Vec<StorageAccountPath>, receiver_asset_name: String, storage_account_resource_id: String) -> Self {
        Self {
            location: None,
            paths,
            provisioning_state: None,
            receiver_asset_name,
            storage_account_resource_id,
        }
    }
}
#[doc = "Blob storage account received asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobAccountReceivedAsset {
    #[serde(flatten)]
    pub received_asset: ReceivedAsset,
    #[doc = "Properties of blob account received asset."]
    pub properties: BlobAccountReceivedAssetProperties,
}
impl BlobAccountReceivedAsset {
    pub fn new(received_asset: ReceivedAsset, properties: BlobAccountReceivedAssetProperties) -> Self {
        Self {
            received_asset,
            properties,
        }
    }
}
#[doc = "Properties of blob account received asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobAccountReceivedAssetProperties {
    #[doc = "Location of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Friendly name for the received asset."]
    #[serde(rename = "receiverAssetName", default, skip_serializing_if = "Option::is_none")]
    pub receiver_asset_name: Option<String>,
    #[doc = "Paths from blob account made available for the share."]
    #[serde(rename = "receiverPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub receiver_paths: Vec<String>,
}
impl BlobAccountReceivedAssetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    #[doc = "Gets or sets the reference name."]
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[doc = "Gets or sets the reference type property."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl Collection {
    pub fn new(reference_name: String, type_: String) -> Self {
        Self { reference_name, type_ }
    }
}
#[doc = "An in place accepted sent share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InPlaceAcceptedSentShare {
    #[serde(flatten)]
    pub accepted_sent_share: AcceptedSentShare,
    #[doc = "Properties of in place accepted sent share."]
    pub properties: InPlaceAcceptedSentShareProperties,
}
impl InPlaceAcceptedSentShare {
    pub fn new(accepted_sent_share: AcceptedSentShare, properties: InPlaceAcceptedSentShareProperties) -> Self {
        Self {
            accepted_sent_share,
            properties,
        }
    }
}
#[doc = "Properties of in place accepted sent share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InPlaceAcceptedSentShareProperties {
    #[doc = "created at"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Expiration date of the received share in UTC format"]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "received share status"]
    #[serde(rename = "receivedShareStatus", default, skip_serializing_if = "Option::is_none")]
    pub received_share_status: Option<ReceivedShareStatus>,
    #[doc = "Email of the user/receiver who received the sent share invitation and created the received share"]
    #[serde(rename = "receiverEmail", default, skip_serializing_if = "Option::is_none")]
    pub receiver_email: Option<String>,
    #[doc = "Name of the user/receiver who received the sent share invitation and created the received share"]
    #[serde(rename = "receiverName", default, skip_serializing_if = "Option::is_none")]
    pub receiver_name: Option<String>,
    #[doc = "Receiver's target object id"]
    #[serde(rename = "receiverTargetObjectId", default, skip_serializing_if = "Option::is_none")]
    pub receiver_target_object_id: Option<String>,
    #[doc = "Tenant name of the user/receiver who received the sent share invitation and created the received share"]
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
    #[doc = "Shared at"]
    #[serde(rename = "sharedAt", default, with = "azure_core::date::rfc3339::option")]
    pub shared_at: Option<time::OffsetDateTime>,
}
impl InPlaceAcceptedSentShareProperties {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InPlaceReceivedShareProperties {
    #[doc = "Reference to a Collection."]
    pub collection: Collection,
    #[doc = "Time at which the received share was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The expiration date of the received share."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "The invitation id."]
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[doc = "Provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "received share status"]
    #[serde(rename = "receivedShareStatus", default, skip_serializing_if = "Option::is_none")]
    pub received_share_status: Option<ReceivedShareStatus>,
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
    #[doc = "Sent share location."]
    #[serde(rename = "sentShareLocation")]
    pub sent_share_location: String,
    #[doc = "Time at which the sent share was shared."]
    #[serde(rename = "sharedAt", default, with = "azure_core::date::rfc3339::option")]
    pub shared_at: Option<time::OffsetDateTime>,
    #[doc = "Name of the share"]
    #[serde(rename = "shareName", default, skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
}
impl InPlaceReceivedShareProperties {
    pub fn new(collection: Collection, invitation_id: String, sent_share_location: String) -> Self {
        Self {
            collection,
            created_at: None,
            expiration_date: None,
            invitation_id,
            provisioning_state: None,
            received_share_status: None,
            receiver_email: None,
            receiver_name: None,
            receiver_tenant_name: None,
            sender_email: None,
            sender_name: None,
            sender_tenant_name: None,
            sent_share_description: None,
            sent_share_location,
            shared_at: None,
            share_name: None,
        }
    }
}
#[doc = "An InPlace share kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InPlaceSentShare {
    #[serde(flatten)]
    pub sent_share: SentShare,
    #[doc = "Properties of in place sent share."]
    pub properties: InPlaceSentShareProperties,
}
impl InPlaceSentShare {
    pub fn new(sent_share: SentShare, properties: InPlaceSentShareProperties) -> Self {
        Self { sent_share, properties }
    }
}
#[doc = "Properties of in place sent share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InPlaceSentShareProperties {
    #[doc = "Reference to a Collection."]
    pub collection: Collection,
    #[doc = "Time at which the share was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Share description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Email of the sender who created the sent share."]
    #[serde(rename = "senderEmail", default, skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[doc = "Name of the sender who created the sent share."]
    #[serde(rename = "senderName", default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Tenant name of the sender who created the sent share."]
    #[serde(rename = "senderTenantName", default, skip_serializing_if = "Option::is_none")]
    pub sender_tenant_name: Option<String>,
}
impl InPlaceSentShareProperties {
    pub fn new(collection: Collection) -> Self {
        Self {
            collection,
            created_at: None,
            description: None,
            provisioning_state: None,
            sender_email: None,
            sender_name: None,
            sender_tenant_name: None,
        }
    }
}
#[doc = "The types of invitations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InvitationKind")]
pub enum InvitationKind {
    User,
    Application,
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
            Self::Application => serializer.serialize_unit_variant("InvitationKind", 1u32, "Application"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Status of the invitation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InvitationStatus")]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InvitationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InvitationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InvitationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("InvitationStatus", 0u32, "Pending"),
            Self::Accepted => serializer.serialize_unit_variant("InvitationStatus", 1u32, "Accepted"),
            Self::Rejected => serializer.serialize_unit_variant("InvitationStatus", 2u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provisioning status of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Unknown,
    Succeeded,
    Creating,
    Deleting,
    Moving,
    Failed,
    SoftDeleting,
    SoftDeleted,
    SourceMoved,
    SourceDeleted,
    TargetMoved,
    TargetDeleted,
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
            Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Moving"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
            Self::SoftDeleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "SoftDeleting"),
            Self::SoftDeleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "SoftDeleted"),
            Self::SourceMoved => serializer.serialize_unit_variant("ProvisioningState", 8u32, "SourceMoved"),
            Self::SourceDeleted => serializer.serialize_unit_variant("ProvisioningState", 9u32, "SourceDeleted"),
            Self::TargetMoved => serializer.serialize_unit_variant("ProvisioningState", 10u32, "TargetMoved"),
            Self::TargetDeleted => serializer.serialize_unit_variant("ProvisioningState", 11u32, "TargetDeleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "A receiver asset data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceivedAsset {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The types of asset."]
    pub kind: AssetType,
}
impl ReceivedAsset {
    pub fn new(kind: AssetType) -> Self {
        Self {
            resource: Resource::default(),
            kind,
        }
    }
}
#[doc = "List of received assets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceivedAssetList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ReceivedAsset>,
}
impl azure_core::Continuable for ReceivedAssetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReceivedAssetList {
    pub fn new(value: Vec<ReceivedAsset>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "A received share invitation data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceivedInvitation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The types of invitations."]
    #[serde(rename = "invitationKind")]
    pub invitation_kind: InvitationKind,
}
impl ReceivedInvitation {
    pub fn new(invitation_kind: InvitationKind) -> Self {
        Self {
            resource: Resource::default(),
            invitation_kind,
        }
    }
}
#[doc = "List of received invitations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceivedInvitationList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ReceivedInvitation>,
}
impl azure_core::Continuable for ReceivedInvitationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReceivedInvitationList {
    pub fn new(value: Vec<ReceivedInvitation>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "A received share data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceivedShare {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Defines the supported types for share."]
    #[serde(rename = "shareKind")]
    pub share_kind: ShareKind,
}
impl ReceivedShare {
    pub fn new(share_kind: ShareKind) -> Self {
        Self {
            resource: Resource::default(),
            share_kind,
        }
    }
}
#[doc = "List of received shares."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceivedShareList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<ReceivedShare>,
}
impl azure_core::Continuable for ReceivedShareList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReceivedShareList {
    pub fn new(value: Vec<ReceivedShare>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "received share status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReceivedShareStatus")]
pub enum ReceivedShareStatus {
    Active,
    Reinstating,
    Revoked,
    Revoking,
    RevokeFailed,
    ReinstateFailed,
    SourceDeleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReceivedShareStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReceivedShareStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReceivedShareStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("ReceivedShareStatus", 0u32, "Active"),
            Self::Reinstating => serializer.serialize_unit_variant("ReceivedShareStatus", 1u32, "Reinstating"),
            Self::Revoked => serializer.serialize_unit_variant("ReceivedShareStatus", 2u32, "Revoked"),
            Self::Revoking => serializer.serialize_unit_variant("ReceivedShareStatus", 3u32, "Revoking"),
            Self::RevokeFailed => serializer.serialize_unit_variant("ReceivedShareStatus", 4u32, "RevokeFailed"),
            Self::ReinstateFailed => serializer.serialize_unit_variant("ReceivedShareStatus", 5u32, "ReinstateFailed"),
            Self::SourceDeleted => serializer.serialize_unit_variant("ReceivedShareStatus", 6u32, "SourceDeleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the supported types for registration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RegistrationStatus")]
pub enum RegistrationStatus {
    ActivationPending,
    Activated,
    ActivationAttemptsExhausted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RegistrationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RegistrationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RegistrationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ActivationPending => serializer.serialize_unit_variant("RegistrationStatus", 0u32, "ActivationPending"),
            Self::Activated => serializer.serialize_unit_variant("RegistrationStatus", 1u32, "Activated"),
            Self::ActivationAttemptsExhausted => {
                serializer.serialize_unit_variant("RegistrationStatus", 2u32, "ActivationAttemptsExhausted")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Base data transfer object implementation for proxy resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A sent share data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentShare {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Defines the supported types for share."]
    #[serde(rename = "shareKind")]
    pub share_kind: ShareKind,
}
impl SentShare {
    pub fn new(share_kind: ShareKind) -> Self {
        Self {
            resource: Resource::default(),
            share_kind,
        }
    }
}
#[doc = "A sent share invitation data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentShareInvitation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The types of invitations."]
    #[serde(rename = "invitationKind")]
    pub invitation_kind: InvitationKind,
}
impl SentShareInvitation {
    pub fn new(invitation_kind: InvitationKind) -> Self {
        Self {
            resource: Resource::default(),
            invitation_kind,
        }
    }
}
#[doc = "List of the sent share invitations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentShareInvitationList {
    #[doc = "The Url of next result page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of items of type DataTransferObjects."]
    pub value: Vec<SentShareInvitation>,
}
impl azure_core::Continuable for SentShareInvitationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SentShareInvitationList {
    pub fn new(value: Vec<SentShareInvitation>) -> Self {
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
    pub value: Vec<SentShare>,
}
impl azure_core::Continuable for SentShareList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SentShareList {
    pub fn new(value: Vec<SentShare>) -> Self {
        Self { next_link: None, value }
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
#[doc = "Defines a single StorageAccountPath path."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountPath {
    #[doc = "Gets or sets the container name."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "Gets or sets the path on the receiver side where the asset is to be mapped."]
    #[serde(rename = "receiverPath", default, skip_serializing_if = "Option::is_none")]
    pub receiver_path: Option<String>,
    #[doc = "Gets or sets the path to file/folder within the container to be shared."]
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
#[doc = "A tenant email registration data transfer object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantEmailRegistration {
    #[serde(flatten)]
    pub resource: Resource,
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
    #[doc = "Date of the activation expiration."]
    #[serde(rename = "activationExpiration", default, with = "azure_core::date::rfc3339::option")]
    pub activation_expiration: Option<time::OffsetDateTime>,
    #[doc = "The email to register."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Defines the supported types for registration."]
    #[serde(rename = "registrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub registration_status: Option<RegistrationStatus>,
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
            tenant_id: None,
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
    #[doc = "The expiration date for the invitation"]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Id of the invitation"]
    #[serde(rename = "invitationId", default, skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[doc = "Status of the invitation."]
    #[serde(rename = "invitationStatus", default, skip_serializing_if = "Option::is_none")]
    pub invitation_status: Option<InvitationStatus>,
    #[doc = "Provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The time the recipient responded to the invitation."]
    #[serde(rename = "respondedAt", default, with = "azure_core::date::rfc3339::option")]
    pub responded_at: Option<time::OffsetDateTime>,
    #[doc = "Email address of the sender."]
    #[serde(rename = "senderEmail", default, skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[doc = "Name of the sender"]
    #[serde(rename = "senderName", default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Tenant name of the sender"]
    #[serde(rename = "senderTenantName", default, skip_serializing_if = "Option::is_none")]
    pub sender_tenant_name: Option<String>,
    #[doc = "Gets the time at which the invitation was sent."]
    #[serde(rename = "sentAt", default, with = "azure_core::date::rfc3339::option")]
    pub sent_at: Option<time::OffsetDateTime>,
    #[doc = "Defines the supported types for share."]
    #[serde(rename = "shareKind", default, skip_serializing_if = "Option::is_none")]
    pub share_kind: Option<ShareKind>,
    #[doc = "The receiver email for the invitation is being sent."]
    #[serde(rename = "targetEmail")]
    pub target_email: String,
}
impl UserInvitationProperties {
    pub fn new(target_email: String) -> Self {
        Self {
            expiration_date: None,
            invitation_id: None,
            invitation_status: None,
            provisioning_state: None,
            responded_at: None,
            sender_email: None,
            sender_name: None,
            sender_tenant_name: None,
            sent_at: None,
            share_kind: None,
            target_email,
        }
    }
}
#[doc = "A user received invitation kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserReceivedInvitation {
    #[serde(flatten)]
    pub received_invitation: ReceivedInvitation,
    #[doc = "Properties for a received invitation of kind user."]
    pub properties: UserReceivedInvitationProperties,
}
impl UserReceivedInvitation {
    pub fn new(received_invitation: ReceivedInvitation, properties: UserReceivedInvitationProperties) -> Self {
        Self {
            received_invitation,
            properties,
        }
    }
}
#[doc = "Properties for a received invitation of kind user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserReceivedInvitationProperties {
    #[doc = "Description shared when the invitation was created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The expiration date for the received share created by accepting the invitation."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Status of the invitation."]
    #[serde(rename = "invitationStatus", default, skip_serializing_if = "Option::is_none")]
    pub invitation_status: Option<InvitationStatus>,
    #[doc = "Location of the invitation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Email of the user/receiver who received the sent share invitation"]
    #[serde(rename = "receiverEmail", default, skip_serializing_if = "Option::is_none")]
    pub receiver_email: Option<String>,
    #[doc = "Name of the user/receiver who received the sent share invitation"]
    #[serde(rename = "receiverName", default, skip_serializing_if = "Option::is_none")]
    pub receiver_name: Option<String>,
    #[doc = "Tenant name of the user/receiver who received the sent share invitation"]
    #[serde(rename = "receiverTenantName", default, skip_serializing_if = "Option::is_none")]
    pub receiver_tenant_name: Option<String>,
    #[doc = "The time the recipient responded to the invitation."]
    #[serde(rename = "respondedAt", default, with = "azure_core::date::rfc3339::option")]
    pub responded_at: Option<time::OffsetDateTime>,
    #[doc = "Email of the sender who created the sent share invitation"]
    #[serde(rename = "senderEmail", default, skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[doc = "Name of the sender who created the sent share invitation"]
    #[serde(rename = "senderName", default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Tenant name of the sender who created the sent share invitation"]
    #[serde(rename = "senderTenantName", default, skip_serializing_if = "Option::is_none")]
    pub sender_tenant_name: Option<String>,
    #[doc = "Gets the time at which the invitation was sent."]
    #[serde(rename = "sentAt", default, with = "azure_core::date::rfc3339::option")]
    pub sent_at: Option<time::OffsetDateTime>,
    #[doc = "Gets the source share Name."]
    #[serde(rename = "sentShareName", default, skip_serializing_if = "Option::is_none")]
    pub sent_share_name: Option<String>,
    #[doc = "Defines the supported types for share."]
    #[serde(rename = "shareKind", default, skip_serializing_if = "Option::is_none")]
    pub share_kind: Option<ShareKind>,
    #[doc = "The receiver email for the invitation is being sent."]
    #[serde(rename = "targetEmail")]
    pub target_email: String,
}
impl UserReceivedInvitationProperties {
    pub fn new(target_email: String) -> Self {
        Self {
            description: None,
            expiration_date: None,
            invitation_status: None,
            location: None,
            receiver_email: None,
            receiver_name: None,
            receiver_tenant_name: None,
            responded_at: None,
            sender_email: None,
            sender_name: None,
            sender_tenant_name: None,
            sent_at: None,
            sent_share_name: None,
            share_kind: None,
            target_email,
        }
    }
}
