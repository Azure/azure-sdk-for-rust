#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The asset conversion error code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConversionErrorCode")]
pub enum ConversionErrorCode {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "NO_ERROR")]
    NoError,
    #[serde(rename = "SERVICE_ERROR")]
    ServiceError,
    #[serde(rename = "INVALID_ASSET_URI")]
    InvalidAssetUri,
    #[serde(rename = "INVALID_JOB_ID")]
    InvalidJobId,
    #[serde(rename = "INVALID_GRAVITY")]
    InvalidGravity,
    #[serde(rename = "INVALID_SCALE")]
    InvalidScale,
    #[serde(rename = "ASSET_SIZE_TOO_LARGE")]
    AssetSizeTooLarge,
    #[serde(rename = "ASSET_DIMENSIONS_OUT_OF_BOUNDS")]
    AssetDimensionsOutOfBounds,
    #[serde(rename = "ZERO_FACES")]
    ZeroFaces,
    #[serde(rename = "INVALID_FACE_VERTICES")]
    InvalidFaceVertices,
    #[serde(rename = "ZERO_TRAJECTORIES_GENERATED")]
    ZeroTrajectoriesGenerated,
    #[serde(rename = "TOO_MANY_RIG_POSES")]
    TooManyRigPoses,
    #[serde(rename = "ASSET_CANNOT_BE_CONVERTED")]
    AssetCannotBeConverted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConversionErrorCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConversionErrorCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConversionErrorCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ConversionErrorCode", 0u32, "UNKNOWN"),
            Self::NoError => serializer.serialize_unit_variant("ConversionErrorCode", 1u32, "NO_ERROR"),
            Self::ServiceError => serializer.serialize_unit_variant("ConversionErrorCode", 2u32, "SERVICE_ERROR"),
            Self::InvalidAssetUri => serializer.serialize_unit_variant("ConversionErrorCode", 3u32, "INVALID_ASSET_URI"),
            Self::InvalidJobId => serializer.serialize_unit_variant("ConversionErrorCode", 4u32, "INVALID_JOB_ID"),
            Self::InvalidGravity => serializer.serialize_unit_variant("ConversionErrorCode", 5u32, "INVALID_GRAVITY"),
            Self::InvalidScale => serializer.serialize_unit_variant("ConversionErrorCode", 6u32, "INVALID_SCALE"),
            Self::AssetSizeTooLarge => serializer.serialize_unit_variant("ConversionErrorCode", 7u32, "ASSET_SIZE_TOO_LARGE"),
            Self::AssetDimensionsOutOfBounds => {
                serializer.serialize_unit_variant("ConversionErrorCode", 8u32, "ASSET_DIMENSIONS_OUT_OF_BOUNDS")
            }
            Self::ZeroFaces => serializer.serialize_unit_variant("ConversionErrorCode", 9u32, "ZERO_FACES"),
            Self::InvalidFaceVertices => serializer.serialize_unit_variant("ConversionErrorCode", 10u32, "INVALID_FACE_VERTICES"),
            Self::ZeroTrajectoriesGenerated => {
                serializer.serialize_unit_variant("ConversionErrorCode", 11u32, "ZERO_TRAJECTORIES_GENERATED")
            }
            Self::TooManyRigPoses => serializer.serialize_unit_variant("ConversionErrorCode", 12u32, "TOO_MANY_RIG_POSES"),
            Self::AssetCannotBeConverted => serializer.serialize_unit_variant("ConversionErrorCode", 13u32, "ASSET_CANNOT_BE_CONVERTED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    pub code: String,
    #[doc = "The error message."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "Inner error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "Represents an error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "Represents an error."]
    pub error: ErrorDetail,
}
impl ErrorResponse {
    pub fn new(error: ErrorDetail) -> Self {
        Self { error }
    }
}
#[doc = "Represents an ingestion configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IngestionConfiguration {
    #[doc = "Represents a vector with three single-precision floating-point values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vector3>,
    #[doc = "Represents a vector with three single-precision floating-point values."]
    #[serde(rename = "boundingBoxCenter", default, skip_serializing_if = "Option::is_none")]
    pub bounding_box_center: Option<Vector3>,
    #[doc = "Represents a vector with three single-precision floating-point values."]
    pub gravity: Vector3,
    #[doc = "Indices of Key Frames."]
    #[serde(
        rename = "keyFrameIndexes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub key_frame_indexes: Vec<i32>,
    #[doc = "Ground truth trajectory."]
    #[serde(
        rename = "gtTrajectory",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub gt_trajectory: Vec<Pose>,
    #[doc = "Represents a vector that is used to encode three-dimensional physical rotations."]
    #[serde(rename = "principalAxis", default, skip_serializing_if = "Option::is_none")]
    pub principal_axis: Option<Quaternion>,
    #[doc = "Scale of transformation of asset units into meter space."]
    pub scale: f32,
    #[doc = "Whether or not disable the scale units in the model metadata"]
    #[serde(rename = "disableDetectScaleUnits", default, skip_serializing_if = "Option::is_none")]
    pub disable_detect_scale_units: Option<bool>,
    #[doc = "Represents a vector with four single-precision floating-point values."]
    #[serde(rename = "supportingPlane", default, skip_serializing_if = "Option::is_none")]
    pub supporting_plane: Option<Vector4>,
    #[doc = "Test Trajectory."]
    #[serde(
        rename = "testTrajectory",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_trajectory: Vec<Pose>,
}
impl IngestionConfiguration {
    pub fn new(gravity: Vector3, scale: f32) -> Self {
        Self {
            dimensions: None,
            bounding_box_center: None,
            gravity,
            key_frame_indexes: Vec::new(),
            gt_trajectory: Vec::new(),
            principal_axis: None,
            scale,
            disable_detect_scale_units: None,
            supporting_plane: None,
            test_trajectory: Vec::new(),
        }
    }
}
#[doc = "Represents the status of an AOA asset conversion job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngestionProperties {
    #[doc = "Information about the cause of a ClientError JobStatus."]
    #[serde(rename = "clientErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub client_error_details: Option<String>,
    #[doc = "Information about the cause of a ServerError JobStatus."]
    #[serde(rename = "serverErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub server_error_details: Option<String>,
    #[doc = "The asset conversion error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ConversionErrorCode>,
    #[doc = "Identifier for the AOA asset conversion job."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The URI for downloading the generated AOA Model"]
    #[serde(rename = "outputModelUri", default, skip_serializing_if = "Option::is_none")]
    pub output_model_uri: Option<String>,
    #[doc = "Represents job status state."]
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<JobStatus>,
    #[doc = "The file type of the original 3D asset. Examples include: \"ply\", \"obj\", \"fbx\", \"glb\", \"gltf\", etc."]
    #[serde(rename = "assetFileType", default, skip_serializing_if = "Option::is_none")]
    pub asset_file_type: Option<String>,
    #[doc = "The Uri to the Asset to be ingested by the AOA asset conversion service. This asset needs to have been uploaded to the service using an endpoint provided from a call to the GetUploadUri API."]
    #[serde(rename = "inputAssetUri", default, skip_serializing_if = "Option::is_none")]
    pub input_asset_uri: Option<String>,
    #[doc = "Identifier for the Account owning the asset conversion job."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Represents an ingestion configuration."]
    #[serde(rename = "ingestionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_configuration: Option<IngestionConfiguration>,
    #[doc = "Represents a vector with three single-precision floating-point values."]
    #[serde(rename = "scaledAssetDimensions", default, skip_serializing_if = "Option::is_none")]
    pub scaled_asset_dimensions: Option<Vector3>,
}
impl IngestionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Inner error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "A more specific error code than was provided by the containing error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Inner error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<InnerError>>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents job status state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobStatus {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}
#[doc = "Represents a pose."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    #[doc = "Represents a vector that is used to encode three-dimensional physical rotations."]
    pub rotation: Quaternion,
    #[doc = "Represents a vector with three single-precision floating-point values."]
    pub translation: Vector3,
}
impl Pose {
    pub fn new(rotation: Quaternion, translation: Vector3) -> Self {
        Self { rotation, translation }
    }
}
#[doc = "Represents a vector that is used to encode three-dimensional physical rotations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    #[doc = "The x value of the vector component of the quaternion."]
    pub x: f32,
    #[doc = "The y value of the vector component of the quaternion."]
    pub y: f32,
    #[doc = "The z value of the vector component of the quaternion."]
    pub z: f32,
    #[doc = "The rotation component of the quaternion."]
    pub w: f32,
    #[doc = "Gets a value that indicates whether the current instance is the identity quaternion"]
    #[serde(rename = "isIdentity", default, skip_serializing_if = "Option::is_none")]
    pub is_identity: Option<bool>,
}
impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w,
            is_identity: None,
        }
    }
}
#[doc = "Represents an upload location for model ingestion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadLocation {
    #[doc = "The blob upload URI where a model should be uploaded to the service for ingestion."]
    #[serde(rename = "inputAssetUri")]
    pub input_asset_uri: String,
}
impl UploadLocation {
    pub fn new(input_asset_uri: String) -> Self {
        Self { input_asset_uri }
    }
}
#[doc = "Represents a vector with three single-precision floating-point values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    #[doc = "The x component of the vector."]
    pub x: f32,
    #[doc = "The y component of the vector."]
    pub y: f32,
    #[doc = "The z component of the vector."]
    pub z: f32,
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
#[doc = "Represents a vector with four single-precision floating-point values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector4 {
    #[doc = "The x component of the vector."]
    pub x: f32,
    #[doc = "The y component of the vector."]
    pub y: f32,
    #[doc = "The z component of the vector."]
    pub z: f32,
    #[doc = "The w component of the vector."]
    pub w: f32,
}
impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}
