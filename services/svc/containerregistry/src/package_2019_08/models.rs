#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessToken {
    #[doc = "The access token for performing authenticated requests"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}
impl AccessToken {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    pub detail: Option<serde_json::Value>,
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
    pub manifests: Vec<ManifestAttributesBase>,
}
impl AcrManifests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional information provided through arbitrary metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Annotations {
    #[doc = "Date and time on which the image was built (string, date-time as defined by https://tools.ietf.org/html/rfc3339#section-5.6)"]
    #[serde(rename = "org.opencontainers.image.created", default, with = "azure_core::date::rfc3339::option")]
    pub org_opencontainers_image_created: Option<time::OffsetDateTime>,
    #[doc = "Contact details of the people or organization responsible for the image."]
    #[serde(rename = "org.opencontainers.image.authors", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_authors: Option<String>,
    #[doc = "URL to find more information on the image."]
    #[serde(rename = "org.opencontainers.image.url", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_url: Option<String>,
    #[doc = "URL to get documentation on the image."]
    #[serde(
        rename = "org.opencontainers.image.documentation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub org_opencontainers_image_documentation: Option<String>,
    #[doc = "URL to get source code for building the image."]
    #[serde(rename = "org.opencontainers.image.source", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_source: Option<String>,
    #[doc = "Version of the packaged software. The version MAY match a label or tag in the source code repository, may also be Semantic versioning-compatible"]
    #[serde(rename = "org.opencontainers.image.version", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_version: Option<String>,
    #[doc = "Source control revision identifier for the packaged software."]
    #[serde(rename = "org.opencontainers.image.revision", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_revision: Option<String>,
    #[doc = "Name of the distributing entity, organization or individual."]
    #[serde(rename = "org.opencontainers.image.vendor", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_vendor: Option<String>,
    #[doc = "License(s) under which contained software is distributed as an SPDX License Expression."]
    #[serde(rename = "org.opencontainers.image.licenses", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_licenses: Option<String>,
    #[doc = "Name of the reference for a target."]
    #[serde(rename = "org.opencontainers.image.ref.name", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_ref_name: Option<String>,
    #[doc = "Human-readable title of the image"]
    #[serde(rename = "org.opencontainers.image.title", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_title: Option<String>,
    #[doc = "Human-readable description of the software packaged in the image"]
    #[serde(rename = "org.opencontainers.image.description", default, skip_serializing_if = "Option::is_none")]
    pub org_opencontainers_image_description: Option<String>,
}
impl Annotations {
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
#[doc = "Docker V2 image layer descriptor including config and layers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Descriptor {
    #[doc = "Layer media type"]
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "Layer size"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "Layer digest"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[doc = "Specifies a list of URIs from which this object may be downloaded."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<String>,
    #[doc = "Additional information provided through arbitrary metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}
impl Descriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image layer information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FsLayer {
    #[doc = "SHA of an image layer"]
    #[serde(rename = "blobSum", default, skip_serializing_if = "Option::is_none")]
    pub blob_sum: Option<String>,
}
impl FsLayer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of unstructured historical data for v1 compatibility"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct History {
    #[doc = "The raw v1 compatibility information"]
    #[serde(rename = "v1Compatibility", default, skip_serializing_if = "Option::is_none")]
    pub v1_compatibility: Option<String>,
}
impl History {
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
    pub schema_version: Option<i64>,
}
impl Manifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Manifest attributes details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestAttributes {
    #[doc = "Registry name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "Image name"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Manifest details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<ManifestAttributesBase>,
}
impl ManifestAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Manifest details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestAttributesBase {
    #[doc = "Manifest"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[doc = "Image size"]
    #[serde(rename = "imageSize", default, skip_serializing_if = "Option::is_none")]
    pub image_size: Option<i64>,
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
    #[doc = "Config blob media type"]
    #[serde(rename = "configMediaType", default, skip_serializing_if = "Option::is_none")]
    pub config_media_type: Option<String>,
    #[doc = "List of tags"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[doc = "Changeable attributes"]
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<ManifestChangeableAttributes>,
}
impl ManifestAttributesBase {
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
#[doc = "Returns the requested Docker multi-arch-manifest file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestList {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[doc = "Media type for this Manifest"]
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "List of V2 image layer information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<ManifestListAttributes>,
}
impl ManifestList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestListAttributes {
    #[doc = "The MIME type of the referenced object. This will generally be application/vnd.docker.image.manifest.v2+json, but it could also be application/vnd.docker.image.manifest.v1+json"]
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "The size in bytes of the object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "The digest of the content, as defined by the Registry V2 HTTP API Specification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[doc = "The platform object describes the platform which the image in the manifest runs on. A full list of valid operating system and architecture values are listed in the Go language documentation for $GOOS and $GOARCH"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
}
impl ManifestListAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns the requested manifest file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestWrapper {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[doc = "Media type for this Manifest"]
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "(ManifestList, OCIIndex) List of V2 image layer information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<ManifestListAttributes>,
    #[doc = "Docker V2 image layer descriptor including config and layers"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Descriptor>,
    #[doc = "(V2, OCI) List of V2 image layer information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<Descriptor>,
    #[doc = "Additional information provided through arbitrary metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    #[doc = "(V1) CPU architecture"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[doc = "(V1) Image name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "(V1) Image tag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "(V1) List of layer information"]
    #[serde(rename = "fsLayers", default, skip_serializing_if = "Vec::is_empty")]
    pub fs_layers: Vec<FsLayer>,
    #[doc = "(V1) Image history"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<History>,
    #[doc = "(V1) Image signature"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<ImageSignature>,
}
impl ManifestWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns the requested OCI index file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OciIndex {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[doc = "List of OCI image layer information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<ManifestListAttributes>,
    #[doc = "Additional information provided through arbitrary metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}
impl OciIndex {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns the requested OCI Manifest file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OciManifest {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[doc = "Docker V2 image layer descriptor including config and layers"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Descriptor>,
    #[doc = "List of V2 image layer information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<Descriptor>,
    #[doc = "Additional information provided through arbitrary metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}
impl OciManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The platform object describes the platform which the image in the manifest runs on. A full list of valid operating system and architecture values are listed in the Go language documentation for $GOOS and $GOARCH"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Platform {
    #[doc = "Specifies the CPU architecture, for example amd64 or ppc64le."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[doc = "The os field specifies the operating system, for example linux or windows."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[doc = "The optional os.version field specifies the operating system version, for example 10.0.10586."]
    #[serde(rename = "os.version", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "The optional os.features field specifies an array of strings, each listing a required OS feature (for example on Windows win32k"]
    #[serde(rename = "os.features", default, skip_serializing_if = "Vec::is_empty")]
    pub os_features: Vec<String>,
    #[doc = "The optional variant field specifies a variant of the CPU, for example armv6l to specify a particular CPU variant of the ARM CPU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[doc = "The optional features field specifies an array of strings, each listing a required CPU feature (for example sse4 or aes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<String>,
}
impl Platform {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshToken {
    #[doc = "The refresh token to be used for generating access tokens"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}
impl RefreshToken {
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
    pub manifest_count: Option<i64>,
    #[doc = "Number of the tags"]
    #[serde(rename = "tagCount", default, skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<i64>,
    #[doc = "Changeable attributes for Repository"]
    #[serde(rename = "changeableAttributes", default, skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<RepositoryChangeableAttributes>,
}
impl RepositoryAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Changeable attributes for Repository"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryChangeableAttributes {
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
    #[doc = "Enables Teleport functionality on new images in the repository improving Container startup performance"]
    #[serde(rename = "teleportEnabled", default, skip_serializing_if = "Option::is_none")]
    pub teleport_enabled: Option<bool>,
}
impl RepositoryChangeableAttributes {
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
    #[doc = "Tag attribute details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<TagAttributesBase>,
}
impl TagAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag attribute details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagAttributesBase {
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
    pub changeable_attributes: Option<TagChangeableAttributes>,
}
impl TagAttributesBase {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagChangeableAttributes {
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
impl TagChangeableAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of tag details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagList {
    #[doc = "Registry name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[doc = "Image name"]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "List of tag attribute details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagAttributesBase>,
}
impl TagList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns the requested V1 manifest file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct V1Manifest {
    #[serde(flatten)]
    pub manifest: Manifest,
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
    pub fs_layers: Vec<FsLayer>,
    #[doc = "Image history"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<History>,
    #[doc = "Image signature"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<ImageSignature>,
}
impl V1Manifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns the requested Docker V2 Manifest file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct V2Manifest {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[doc = "Media type for this Manifest"]
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "Docker V2 image layer descriptor including config and layers"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Descriptor>,
    #[doc = "List of V2 image layer information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<Descriptor>,
}
impl V2Manifest {
    pub fn new() -> Self {
        Self::default()
    }
}
