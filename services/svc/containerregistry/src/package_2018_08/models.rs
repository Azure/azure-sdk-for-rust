#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Error information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrErrorInfo {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
impl AcrErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Acr error response describing why the operation failed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrErrors {
    #[doc = "Array of detailed error"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<AcrErrorInfo>,
}
impl AcrErrors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Manifest attributes details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrManifestAttributes {
    #[doc = "Registry name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "Image name"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Manifest details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<AcrManifestAttributesBase>,
}
impl AcrManifestAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Manifest details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrManifestAttributesBase {
    #[doc = "Manifest digest"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[doc = "Created time"]
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[doc = "Last update time"]
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[doc = "CPU architecture"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[doc = "Operating system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[doc = "Media type"]
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "List of tags"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<ChangeableAttributes>,
}
impl AcrManifestAttributesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Manifest attributes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrManifests {
    #[doc = "Registry name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "Image name"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "List of manifests"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<AcrManifestAttributesBase>,
}
impl AcrManifests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of tag details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrRepositoryTags {
    #[doc = "Registry name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "Image name"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "List of tag attribute details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<AcrTagAttributesBase>,
}
impl AcrRepositoryTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag attributes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrTagAttributes {
    #[doc = "Registry name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "Image name"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Tag attribute details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<AcrTagAttributesBase>,
}
impl AcrTagAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag attribute details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrTagAttributesBase {
    #[doc = "Tag name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Tag digest"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[doc = "Tag created time"]
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[doc = "Tag last update time"]
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[doc = "Is signed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed: Option<bool>,
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<ChangeableAttributes>,
}
impl AcrTagAttributesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeableAttributes {
    #[doc = "Delete enabled"]
    #[serde(rename = "deleteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub delete_enabled: Option<bool>,
    #[doc = "Write enabled"]
    #[serde(rename = "writeEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    #[doc = "List enabled"]
    #[serde(rename = "listEnabled", default, skip_serializing_if = "Option::is_none")]
    pub list_enabled: Option<bool>,
    #[doc = "Read enabled"]
    #[serde(rename = "readEnabled", default, skip_serializing_if = "Option::is_none")]
    pub read_enabled: Option<bool>,
}
impl ChangeableAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deleted repository"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedRepository {
    #[doc = "SHA of the deleted image"]
    #[serde(rename = "manifestsDeleted", default, skip_serializing_if = "Vec::is_empty")]
    pub manifests_deleted: Vec<String>,
    #[doc = "Tag of the deleted image"]
    #[serde(rename = "tagsDeleted", default, skip_serializing_if = "Vec::is_empty")]
    pub tags_deleted: Vec<String>,
}
impl DeletedRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of unstructured historical data for v1 compatibility"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageHistory {
    #[doc = "The raw v1 compatibility information"]
    #[serde(rename = "v1Compatibility", default, skip_serializing_if = "Option::is_none")]
    pub v1_compatibility: Option<String>,
}
impl ImageHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image layer information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageLayer {
    #[doc = "SHA of an image layer"]
    #[serde(rename = "blobSum", default, skip_serializing_if = "Option::is_none")]
    pub blob_sum: Option<String>,
}
impl ImageLayer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Signature of a signed manifest"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageSignature {
    #[doc = "A JSON web signature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Jwk>,
    #[doc = "A signature for the image manifest, signed by a libtrust private key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "The signed protected header"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected: Option<String>,
}
impl ImageSignature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A JSON web signature"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Jwk {
    #[doc = "JSON web key parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwk: Option<JwkHeader>,
    #[doc = "The algorithm used to sign or encrypt the JWT"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
}
impl Jwk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "JSON web key parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JwkHeader {
    #[doc = "crv value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[doc = "kid value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[doc = "kty value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,
    #[doc = "x value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[doc = "y value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}
impl JwkHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns the requested manifest file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Manifest {
    #[doc = "Schema version"]
    #[serde(rename = "schemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<f64>,
    #[doc = "CPU architecture"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[doc = "Image name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Image tag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "List of layer information"]
    #[serde(rename = "fsLayers", default, skip_serializing_if = "Vec::is_empty")]
    pub fs_layers: Vec<ImageLayer>,
    #[doc = "Image history"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<ImageHistory>,
    #[doc = "Image signature"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<ImageSignature>,
}
impl Manifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of manifest attributes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestAttributesManifest {
    #[doc = "List of manifest attributes details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<ManifestAttributesManifestReferences>,
    #[doc = "Quarantine tag name"]
    #[serde(rename = "quarantineTag", default, skip_serializing_if = "Option::is_none")]
    pub quarantine_tag: Option<String>,
}
impl ManifestAttributesManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Manifest attributes details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestAttributesManifestReferences {
    #[doc = "Manifest digest"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[doc = "CPU architecture"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[doc = "Operating system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}
impl ManifestAttributesManifestReferences {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Changeable attributes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestChangeableAttributes {
    #[doc = "Delete enabled"]
    #[serde(rename = "deleteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub delete_enabled: Option<bool>,
    #[doc = "Write enabled"]
    #[serde(rename = "writeEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    #[doc = "List enabled"]
    #[serde(rename = "listEnabled", default, skip_serializing_if = "Option::is_none")]
    pub list_enabled: Option<bool>,
    #[doc = "Read enabled"]
    #[serde(rename = "readEnabled", default, skip_serializing_if = "Option::is_none")]
    pub read_enabled: Option<bool>,
    #[doc = "Quarantine state"]
    #[serde(rename = "quarantineState", default, skip_serializing_if = "Option::is_none")]
    pub quarantine_state: Option<String>,
    #[doc = "Quarantine details"]
    #[serde(rename = "quarantineDetails", default, skip_serializing_if = "Option::is_none")]
    pub quarantine_details: Option<String>,
}
impl ManifestChangeableAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of repositories"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Repositories {
    #[doc = "Repository names"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
}
impl Repositories {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Repository attributes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryAttributes {
    #[doc = "Registry name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "Image name"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Image created time"]
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[doc = "Image last update time"]
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[doc = "Number of the manifests"]
    #[serde(rename = "manifestCount", default, skip_serializing_if = "Option::is_none")]
    pub manifest_count: Option<f64>,
    #[doc = "Number of the tags"]
    #[serde(rename = "tagCount", default, skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<f64>,
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<ChangeableAttributes>,
}
impl RepositoryAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list tags of the image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryTags {
    #[doc = "Name of the image"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of tags"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl RepositoryTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag attributes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagAttributes {
    #[doc = "Registry name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "Image name"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Tag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<TagAttributesTag>,
}
impl TagAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagAttributesTag {
    #[doc = "SignatureRecord value"]
    #[serde(rename = "signatureRecord", default, skip_serializing_if = "Option::is_none")]
    pub signature_record: Option<String>,
}
impl TagAttributesTag {
    pub fn new() -> Self {
        Self::default()
    }
}
