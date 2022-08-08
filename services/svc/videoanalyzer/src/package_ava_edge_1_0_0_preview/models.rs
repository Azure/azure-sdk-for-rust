#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Base class for certificate sources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateSource {
    #[doc = "Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl CertificateSource {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "A processor that allows the pipeline topology to send video frames to a Cognitive Services Vision extension. Inference results are relayed to downstream nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesVisionProcessor {
    #[serde(flatten)]
    pub processor_node_base: ProcessorNodeBase,
    #[doc = "Base class for endpoints."]
    pub endpoint: EndpointBase,
    #[doc = "Image transformations and formatting options to be applied to the video frame(s)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageProperties>,
    #[doc = "Defines how often media is submitted to the extension plugin."]
    #[serde(rename = "samplingOptions", default, skip_serializing_if = "Option::is_none")]
    pub sampling_options: Option<SamplingOptions>,
    #[doc = "Base class for Azure Cognitive Services Spatial Analysis operations."]
    pub operation: SpatialAnalysisOperationBase,
}
impl CognitiveServicesVisionProcessor {
    pub fn new(processor_node_base: ProcessorNodeBase, endpoint: EndpointBase, operation: SpatialAnalysisOperationBase) -> Self {
        Self {
            processor_node_base,
            endpoint,
            image: None,
            sampling_options: None,
            operation,
        }
    }
}
#[doc = "Base class for credential objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialsBase {
    #[doc = "Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl CredentialsBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Base class for endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointBase {
    #[doc = "Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "Base class for credential objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<CredentialsBase>,
    #[doc = "The endpoint URL for Video Analyzer to connect to."]
    pub url: String,
}
impl EndpointBase {
    pub fn new(type_: String, url: String) -> Self {
        Self {
            type_,
            credentials: None,
            url,
        }
    }
}
#[doc = "Base class for pipeline extension processors. Pipeline extensions allow for custom media analysis and processing to be plugged into the Video Analyzer pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionProcessorBase {
    #[serde(flatten)]
    pub processor_node_base: ProcessorNodeBase,
    #[doc = "Base class for endpoints."]
    pub endpoint: EndpointBase,
    #[doc = "Image transformations and formatting options to be applied to the video frame(s)."]
    pub image: ImageProperties,
    #[doc = "Defines how often media is submitted to the extension plugin."]
    #[serde(rename = "samplingOptions", default, skip_serializing_if = "Option::is_none")]
    pub sampling_options: Option<SamplingOptions>,
}
impl ExtensionProcessorBase {
    pub fn new(processor_node_base: ProcessorNodeBase, endpoint: EndpointBase, image: ImageProperties) -> Self {
        Self {
            processor_node_base,
            endpoint,
            image,
            sampling_options: None,
        }
    }
}
#[doc = "File sink allows for video and audio content to be recorded on the file system on the edge device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSink {
    #[serde(flatten)]
    pub sink_node_base: SinkNodeBase,
    #[doc = "Absolute directory path where media files will be stored."]
    #[serde(rename = "baseDirectoryPath")]
    pub base_directory_path: String,
    #[doc = "File name pattern for creating new files when performing event based recording. The pattern must include at least one system variable."]
    #[serde(rename = "fileNamePattern")]
    pub file_name_pattern: String,
    #[doc = "Maximum amount of disk space that can be used for storing files from this sink. Once this limit is reached, the oldest files from this sink will be automatically deleted."]
    #[serde(rename = "maximumSizeMiB")]
    pub maximum_size_mi_b: String,
}
impl FileSink {
    pub fn new(sink_node_base: SinkNodeBase, base_directory_path: String, file_name_pattern: String, maximum_size_mi_b: String) -> Self {
        Self {
            sink_node_base,
            base_directory_path,
            file_name_pattern,
            maximum_size_mi_b,
        }
    }
}
#[doc = "GRPC extension processor allows pipeline extension plugins to be connected to the pipeline through over a gRPC channel. Extension plugins must act as an gRPC server. Please see https://aka.ms/ava-extension-grpc for details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrpcExtension {
    #[serde(flatten)]
    pub extension_processor_base: ExtensionProcessorBase,
    #[doc = "Describes how media is transferred to the extension plugin."]
    #[serde(rename = "dataTransfer")]
    pub data_transfer: GrpcExtensionDataTransfer,
    #[doc = "An optional configuration string that is sent to the extension plugin. The configuration string is specific to each custom extension and it not understood neither validated by Video Analyzer. Please see https://aka.ms/ava-extension-grpc for details."]
    #[serde(rename = "extensionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub extension_configuration: Option<String>,
}
impl GrpcExtension {
    pub fn new(extension_processor_base: ExtensionProcessorBase, data_transfer: GrpcExtensionDataTransfer) -> Self {
        Self {
            extension_processor_base,
            data_transfer,
            extension_configuration: None,
        }
    }
}
#[doc = "Describes how media is transferred to the extension plugin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrpcExtensionDataTransfer {
    #[doc = "The share memory buffer for sample transfers, in mebibytes. It can only be used with the 'SharedMemory' transfer mode."]
    #[serde(rename = "sharedMemorySizeMiB", default, skip_serializing_if = "Option::is_none")]
    pub shared_memory_size_mi_b: Option<String>,
    #[doc = "Data transfer mode: embedded or sharedMemory."]
    pub mode: grpc_extension_data_transfer::Mode,
}
impl GrpcExtensionDataTransfer {
    pub fn new(mode: grpc_extension_data_transfer::Mode) -> Self {
        Self {
            shared_memory_size_mi_b: None,
            mode,
        }
    }
}
pub mod grpc_extension_data_transfer {
    use super::*;
    #[doc = "Data transfer mode: embedded or sharedMemory."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        #[serde(rename = "embedded")]
        Embedded,
        #[serde(rename = "sharedMemory")]
        SharedMemory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Embedded => serializer.serialize_unit_variant("Mode", 0u32, "embedded"),
                Self::SharedMemory => serializer.serialize_unit_variant("Mode", 1u32, "sharedMemory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "HTTP extension processor allows pipeline extension plugins to be connected to the pipeline through over the HTTP protocol. Extension plugins must act as an HTTP server. Please see https://aka.ms/ava-extension-http for details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpExtension {
    #[serde(flatten)]
    pub extension_processor_base: ExtensionProcessorBase,
}
impl HttpExtension {
    pub fn new(extension_processor_base: ExtensionProcessorBase) -> Self {
        Self { extension_processor_base }
    }
}
#[doc = "HTTP header credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpHeaderCredentials {
    #[serde(flatten)]
    pub credentials_base: CredentialsBase,
    #[doc = "HTTP header name."]
    #[serde(rename = "headerName")]
    pub header_name: String,
    #[doc = "HTTP header value. It is recommended that this value is parameterized as a secret string in order to prevent this value to be returned as part of the resource on API requests."]
    #[serde(rename = "headerValue")]
    pub header_value: String,
}
impl HttpHeaderCredentials {
    pub fn new(credentials_base: CredentialsBase, header_name: String, header_value: String) -> Self {
        Self {
            credentials_base,
            header_name,
            header_value,
        }
    }
}
#[doc = "BMP image encoding."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFormatBmp {
    #[serde(flatten)]
    pub image_format_properties: ImageFormatProperties,
}
impl ImageFormatBmp {
    pub fn new(image_format_properties: ImageFormatProperties) -> Self {
        Self { image_format_properties }
    }
}
#[doc = "JPEG image encoding."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFormatJpeg {
    #[serde(flatten)]
    pub image_format_properties: ImageFormatProperties,
    #[doc = "Image quality value between 0 to 100 (best quality)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
}
impl ImageFormatJpeg {
    pub fn new(image_format_properties: ImageFormatProperties) -> Self {
        Self {
            image_format_properties,
            quality: None,
        }
    }
}
#[doc = "PNG image encoding."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFormatPng {
    #[serde(flatten)]
    pub image_format_properties: ImageFormatProperties,
}
impl ImageFormatPng {
    pub fn new(image_format_properties: ImageFormatProperties) -> Self {
        Self { image_format_properties }
    }
}
#[doc = "Base class for image formatting properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFormatProperties {
    #[doc = "Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl ImageFormatProperties {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Raw image formatting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFormatRaw {
    #[serde(flatten)]
    pub image_format_properties: ImageFormatProperties,
    #[doc = "Pixel format to be applied to the raw image."]
    #[serde(rename = "pixelFormat")]
    pub pixel_format: image_format_raw::PixelFormat,
}
impl ImageFormatRaw {
    pub fn new(image_format_properties: ImageFormatProperties, pixel_format: image_format_raw::PixelFormat) -> Self {
        Self {
            image_format_properties,
            pixel_format,
        }
    }
}
pub mod image_format_raw {
    use super::*;
    #[doc = "Pixel format to be applied to the raw image."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PixelFormat")]
    pub enum PixelFormat {
        #[serde(rename = "yuv420p")]
        Yuv420p,
        #[serde(rename = "rgb565be")]
        Rgb565be,
        #[serde(rename = "rgb565le")]
        Rgb565le,
        #[serde(rename = "rgb555be")]
        Rgb555be,
        #[serde(rename = "rgb555le")]
        Rgb555le,
        #[serde(rename = "rgb24")]
        Rgb24,
        #[serde(rename = "bgr24")]
        Bgr24,
        #[serde(rename = "argb")]
        Argb,
        #[serde(rename = "rgba")]
        Rgba,
        #[serde(rename = "abgr")]
        Abgr,
        #[serde(rename = "bgra")]
        Bgra,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PixelFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PixelFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PixelFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Yuv420p => serializer.serialize_unit_variant("PixelFormat", 0u32, "yuv420p"),
                Self::Rgb565be => serializer.serialize_unit_variant("PixelFormat", 1u32, "rgb565be"),
                Self::Rgb565le => serializer.serialize_unit_variant("PixelFormat", 2u32, "rgb565le"),
                Self::Rgb555be => serializer.serialize_unit_variant("PixelFormat", 3u32, "rgb555be"),
                Self::Rgb555le => serializer.serialize_unit_variant("PixelFormat", 4u32, "rgb555le"),
                Self::Rgb24 => serializer.serialize_unit_variant("PixelFormat", 5u32, "rgb24"),
                Self::Bgr24 => serializer.serialize_unit_variant("PixelFormat", 6u32, "bgr24"),
                Self::Argb => serializer.serialize_unit_variant("PixelFormat", 7u32, "argb"),
                Self::Rgba => serializer.serialize_unit_variant("PixelFormat", 8u32, "rgba"),
                Self::Abgr => serializer.serialize_unit_variant("PixelFormat", 9u32, "abgr"),
                Self::Bgra => serializer.serialize_unit_variant("PixelFormat", 10u32, "bgra"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Image transformations and formatting options to be applied to the video frame(s)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageProperties {
    #[doc = "Image scaling mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<ImageScale>,
    #[doc = "Base class for image formatting properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ImageFormatProperties>,
}
impl ImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image scaling mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageScale {
    #[doc = "Describes the image scaling mode to be applied. Default mode is 'pad'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<image_scale::Mode>,
    #[doc = "The desired output image width."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[doc = "The desired output image height."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
}
impl ImageScale {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod image_scale {
    use super::*;
    #[doc = "Describes the image scaling mode to be applied. Default mode is 'pad'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        #[serde(rename = "preserveAspectRatio")]
        PreserveAspectRatio,
        #[serde(rename = "pad")]
        Pad,
        #[serde(rename = "stretch")]
        Stretch,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PreserveAspectRatio => serializer.serialize_unit_variant("Mode", 0u32, "preserveAspectRatio"),
                Self::Pad => serializer.serialize_unit_variant("Mode", 1u32, "pad"),
                Self::Stretch => serializer.serialize_unit_variant("Mode", 2u32, "stretch"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "IoT Hub Message sink allows for pipeline messages to published into the IoT Edge Hub. Published messages can then be delivered to the cloud and other modules via routes declared in the IoT Edge deployment manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubMessageSink {
    #[serde(flatten)]
    pub sink_node_base: SinkNodeBase,
    #[doc = "Name of the Iot Edge Hub output to which the messages will be published."]
    #[serde(rename = "hubOutputName")]
    pub hub_output_name: String,
}
impl IotHubMessageSink {
    pub fn new(sink_node_base: SinkNodeBase, hub_output_name: String) -> Self {
        Self {
            sink_node_base,
            hub_output_name,
        }
    }
}
#[doc = "IoT Hub Message source allows for the pipeline to consume messages from the IoT Edge Hub. Messages can be routed from other IoT modules via routes declared in the IoT Edge deployment manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubMessageSource {
    #[serde(flatten)]
    pub source_node_base: SourceNodeBase,
    #[doc = "Name of the IoT Edge Hub input from which messages will be consumed."]
    #[serde(rename = "hubInputName", default, skip_serializing_if = "Option::is_none")]
    pub hub_input_name: Option<String>,
}
impl IotHubMessageSource {
    pub fn new(source_node_base: SourceNodeBase) -> Self {
        Self {
            source_node_base,
            hub_input_name: None,
        }
    }
}
#[doc = "Line crossing processor allows for the detection of tracked objects moving across one or more predefined lines. It must be downstream of an object tracker of downstream on an AI extension node that generates sequenceId for objects which are tracked across different frames of the video. Inference events are generated every time objects crosses from one side of the line to another."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineCrossingProcessor {
    #[serde(flatten)]
    pub processor_node_base: ProcessorNodeBase,
    #[doc = "An array of lines used to compute line crossing events."]
    pub lines: Vec<NamedLineBase>,
}
impl LineCrossingProcessor {
    pub fn new(processor_node_base: ProcessorNodeBase, lines: Vec<NamedLineBase>) -> Self {
        Self {
            processor_node_base,
            lines,
        }
    }
}
#[doc = "Live Pipeline represents an unique instance of a pipeline topology which is used for real-time content ingestion and analysis."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LivePipeline {
    #[doc = "Live pipeline unique identifier."]
    pub name: String,
    #[doc = "Read-only system metadata associated with a resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Live pipeline properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LivePipelineProperties>,
}
impl LivePipeline {
    pub fn new(name: String) -> Self {
        Self {
            name,
            system_data: None,
            properties: None,
        }
    }
}
#[doc = "Activates an existing live pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LivePipelineActivateRequest {
    #[serde(flatten)]
    pub method_request_empty_body_base: MethodRequestEmptyBodyBase,
}
impl LivePipelineActivateRequest {
    pub fn new(method_request_empty_body_base: MethodRequestEmptyBodyBase) -> Self {
        Self {
            method_request_empty_body_base,
        }
    }
}
#[doc = "A collection of live pipelines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LivePipelineCollection {
    #[doc = "List of live pipelines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LivePipeline>,
    #[doc = "A continuation token to be used in subsequent calls when enumerating through the collection. This is returned when the collection results won't fit in a single response."]
    #[serde(rename = "@continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl LivePipelineCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deactivates an existing live pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LivePipelineDeactivateRequest {
    #[serde(flatten)]
    pub method_request_empty_body_base: MethodRequestEmptyBodyBase,
}
impl LivePipelineDeactivateRequest {
    pub fn new(method_request_empty_body_base: MethodRequestEmptyBodyBase) -> Self {
        Self {
            method_request_empty_body_base,
        }
    }
}
#[doc = "Deletes an existing live pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LivePipelineDeleteRequest {
    #[serde(flatten)]
    pub method_request_empty_body_base: MethodRequestEmptyBodyBase,
}
impl LivePipelineDeleteRequest {
    pub fn new(method_request_empty_body_base: MethodRequestEmptyBodyBase) -> Self {
        Self {
            method_request_empty_body_base,
        }
    }
}
#[doc = "Retrieves an existing live pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LivePipelineGetRequest {
    #[serde(flatten)]
    pub method_request_empty_body_base: MethodRequestEmptyBodyBase,
}
impl LivePipelineGetRequest {
    pub fn new(method_request_empty_body_base: MethodRequestEmptyBodyBase) -> Self {
        Self {
            method_request_empty_body_base,
        }
    }
}
#[doc = "List all existing live pipelines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LivePipelineListRequest {
    #[serde(flatten)]
    pub method_request: MethodRequest,
}
impl LivePipelineListRequest {
    pub fn new(method_request: MethodRequest) -> Self {
        Self { method_request }
    }
}
#[doc = "Live pipeline properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LivePipelineProperties {
    #[doc = "An optional description of the live pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The reference to an existing pipeline topology defined for real-time content processing. When activated, this live pipeline will process content according to the pipeline topology definition."]
    #[serde(rename = "topologyName", default, skip_serializing_if = "Option::is_none")]
    pub topology_name: Option<String>,
    #[doc = "List of the instance level parameter values for the user-defined topology parameters. A pipeline can only define or override parameters values for parameters which have been declared in the referenced topology. Topology parameters without a default value must be defined. Topology parameters with a default value can be optionally be overridden."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterDefinition>,
    #[doc = "Current pipeline state (read-only)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<live_pipeline_properties::State>,
}
impl LivePipelineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod live_pipeline_properties {
    use super::*;
    #[doc = "Current pipeline state (read-only)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        #[serde(rename = "inactive")]
        Inactive,
        #[serde(rename = "activating")]
        Activating,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "deactivating")]
        Deactivating,
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
                Self::Inactive => serializer.serialize_unit_variant("State", 0u32, "inactive"),
                Self::Activating => serializer.serialize_unit_variant("State", 1u32, "activating"),
                Self::Active => serializer.serialize_unit_variant("State", 2u32, "active"),
                Self::Deactivating => serializer.serialize_unit_variant("State", 3u32, "deactivating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Creates a new live pipeline or updates an existing one."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LivePipelineSetRequest {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    #[doc = "Live Pipeline represents an unique instance of a pipeline topology which is used for real-time content ingestion and analysis."]
    #[serde(rename = "livePipeline")]
    pub live_pipeline: LivePipeline,
}
impl LivePipelineSetRequest {
    pub fn new(method_request: MethodRequest, live_pipeline: LivePipeline) -> Self {
        Self {
            method_request,
            live_pipeline,
        }
    }
}
#[doc = "Live pipeline resource representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LivePipelineSetRequestBody {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    #[serde(flatten)]
    pub live_pipeline: LivePipeline,
}
impl LivePipelineSetRequestBody {
    pub fn new(method_request: MethodRequest, live_pipeline: LivePipeline) -> Self {
        Self {
            method_request,
            live_pipeline,
        }
    }
}
#[doc = "Base class for direct method calls."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MethodRequest {
    #[doc = "Direct method method name."]
    #[serde(rename = "methodName")]
    pub method_name: String,
    #[doc = "Video Analyzer API version."]
    #[serde(rename = "@apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<method_request::ApiVersion>,
}
impl MethodRequest {
    pub fn new(method_name: String) -> Self {
        Self {
            method_name,
            api_version: None,
        }
    }
}
pub mod method_request {
    use super::*;
    #[doc = "Video Analyzer API version."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApiVersion {
        #[serde(rename = "1.0")]
        N1_0,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MethodRequestEmptyBodyBase {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    #[doc = "Resource name."]
    pub name: String,
}
impl MethodRequestEmptyBodyBase {
    pub fn new(method_request: MethodRequest, name: String) -> Self {
        Self { method_request, name }
    }
}
#[doc = "Motion detection processor allows for motion detection on the video stream. It generates motion events whenever motion is present on the video."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionDetectionProcessor {
    #[serde(flatten)]
    pub processor_node_base: ProcessorNodeBase,
    #[doc = "Motion detection sensitivity: low, medium, high."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<motion_detection_processor::Sensitivity>,
    #[doc = "Indicates whether the processor should detect and output the regions within the video frame where motion was detected. Default is true."]
    #[serde(rename = "outputMotionRegion", default, skip_serializing_if = "Option::is_none")]
    pub output_motion_region: Option<bool>,
    #[doc = "Time window duration on which events are aggregated before being emitted. Value must be specified in ISO8601 duration format (i.e. \"PT2S\" equals 2 seconds). Use 0 seconds for no aggregation. Default is 1 second."]
    #[serde(rename = "eventAggregationWindow", default, skip_serializing_if = "Option::is_none")]
    pub event_aggregation_window: Option<String>,
}
impl MotionDetectionProcessor {
    pub fn new(processor_node_base: ProcessorNodeBase) -> Self {
        Self {
            processor_node_base,
            sensitivity: None,
            output_motion_region: None,
            event_aggregation_window: None,
        }
    }
}
pub mod motion_detection_processor {
    use super::*;
    #[doc = "Motion detection sensitivity: low, medium, high."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Sensitivity")]
    pub enum Sensitivity {
        #[serde(rename = "low")]
        Low,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "high")]
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Sensitivity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Sensitivity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Sensitivity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("Sensitivity", 0u32, "low"),
                Self::Medium => serializer.serialize_unit_variant("Sensitivity", 1u32, "medium"),
                Self::High => serializer.serialize_unit_variant("Sensitivity", 2u32, "high"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for named lines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedLineBase {
    #[doc = "The Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "Line name. Must be unique within the node."]
    pub name: String,
}
impl NamedLineBase {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "Describes a line configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedLineString {
    #[serde(flatten)]
    pub named_line_base: NamedLineBase,
    #[doc = "Point coordinates for the line start and end, respectively. Example: '[[0.3, 0.2],[0.9, 0.8]]'. Each point is expressed as [LEFT, TOP] coordinate ratios ranging from 0.0 to 1.0, where [0,0] is the upper-left frame corner and [1, 1] is the bottom-right frame corner."]
    pub line: String,
}
impl NamedLineString {
    pub fn new(named_line_base: NamedLineBase, line: String) -> Self {
        Self { named_line_base, line }
    }
}
#[doc = "Describes the named polygon."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedPolygonBase {
    #[doc = "The Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "Polygon name. Must be unique within the node."]
    pub name: String,
}
impl NamedPolygonBase {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "Describes a closed polygon configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedPolygonString {
    #[serde(flatten)]
    pub named_polygon_base: NamedPolygonBase,
    #[doc = "Point coordinates for the polygon. Example: '[[0.3, 0.2],[0.9, 0.8],[0.7, 0.6]]'. Each point is expressed as [LEFT, TOP] coordinate ratios ranging from 0.0 to 1.0, where [0,0] is the upper-left frame corner and [1, 1] is the bottom-right frame corner."]
    pub polygon: String,
}
impl NamedPolygonString {
    pub fn new(named_polygon_base: NamedPolygonBase, polygon: String) -> Self {
        Self {
            named_polygon_base,
            polygon,
        }
    }
}
#[doc = "Describes an input signal to be used on a pipeline node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeInput {
    #[doc = "The name of the upstream node in the pipeline which output is used as input of the current node."]
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[doc = "Allows for the selection of specific data streams (eg. video only) from another node."]
    #[serde(rename = "outputSelectors", default, skip_serializing_if = "Vec::is_empty")]
    pub output_selectors: Vec<OutputSelector>,
}
impl NodeInput {
    pub fn new(node_name: String) -> Self {
        Self {
            node_name,
            output_selectors: Vec::new(),
        }
    }
}
#[doc = "Object tracker processor allows for continuous tracking of one of more objects over a finite sequence of video frames. It must be used downstream of an object detector extension node, thus allowing for the extension to be configured to to perform inferences on sparse frames through the use of the 'maximumSamplesPerSecond' sampling property. The object tracker node will then track the detected objects over the frames in which the detector is not invoked resulting on a smother tracking of detected objects across the continuum of video frames. The tracker will stop tracking objects which are not subsequently detected by the upstream detector on the subsequent detections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectTrackingProcessor {
    #[serde(flatten)]
    pub processor_node_base: ProcessorNodeBase,
    #[doc = "Object tracker accuracy: low, medium, high. Higher accuracy leads to higher CPU consumption in average."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<object_tracking_processor::Accuracy>,
}
impl ObjectTrackingProcessor {
    pub fn new(processor_node_base: ProcessorNodeBase) -> Self {
        Self {
            processor_node_base,
            accuracy: None,
        }
    }
}
pub mod object_tracking_processor {
    use super::*;
    #[doc = "Object tracker accuracy: low, medium, high. Higher accuracy leads to higher CPU consumption in average."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Accuracy")]
    pub enum Accuracy {
        #[serde(rename = "low")]
        Low,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "high")]
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Accuracy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Accuracy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Accuracy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("Accuracy", 0u32, "low"),
                Self::Medium => serializer.serialize_unit_variant("Accuracy", 1u32, "medium"),
                Self::High => serializer.serialize_unit_variant("Accuracy", 2u32, "high"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Allows for the selection of particular streams from another node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutputSelector {
    #[doc = "The property of the data stream to be used as the selection criteria."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<output_selector::Property>,
    #[doc = "The operator to compare properties by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<output_selector::Operator>,
    #[doc = "Value to compare against."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl OutputSelector {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod output_selector {
    use super::*;
    #[doc = "The property of the data stream to be used as the selection criteria."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Property")]
    pub enum Property {
        #[serde(rename = "mediaType")]
        MediaType,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Property {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Property {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Property {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MediaType => serializer.serialize_unit_variant("Property", 0u32, "mediaType"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The operator to compare properties by."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        #[serde(rename = "is")]
        Is,
        #[serde(rename = "isNot")]
        IsNot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Is => serializer.serialize_unit_variant("Operator", 0u32, "is"),
                Self::IsNot => serializer.serialize_unit_variant("Operator", 1u32, "isNot"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Single topology parameter declaration. Declared parameters can and must be referenced throughout the topology and can optionally have default values to be used when they are not defined in the pipeline instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDeclaration {
    #[doc = "Name of the parameter."]
    pub name: String,
    #[doc = "Type of the parameter."]
    #[serde(rename = "type")]
    pub type_: parameter_declaration::Type,
    #[doc = "Description of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The default value for the parameter to be used if the live pipeline does not specify a value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}
impl ParameterDeclaration {
    pub fn new(name: String, type_: parameter_declaration::Type) -> Self {
        Self {
            name,
            type_,
            description: None,
            default: None,
        }
    }
}
pub mod parameter_declaration {
    use super::*;
    #[doc = "Type of the parameter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "secretString")]
        SecretString,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "double")]
        Double,
        #[serde(rename = "bool")]
        Bool,
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "string"),
                Self::SecretString => serializer.serialize_unit_variant("Type", 1u32, "secretString"),
                Self::Int => serializer.serialize_unit_variant("Type", 2u32, "int"),
                Self::Double => serializer.serialize_unit_variant("Type", 3u32, "double"),
                Self::Bool => serializer.serialize_unit_variant("Type", 4u32, "bool"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameter value of an specific pipeline topology parameter. See pipeline topology parameters for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    #[doc = "Name of the parameter declared in the pipeline topology."]
    pub name: String,
    #[doc = "Parameter value to be applied on this specific live pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ParameterDefinition {
    pub fn new(name: String) -> Self {
        Self { name, value: None }
    }
}
#[doc = "A list of PEM formatted certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PemCertificateList {
    #[serde(flatten)]
    pub certificate_source: CertificateSource,
    #[doc = "PEM formatted public certificates. One certificate per entry."]
    pub certificates: Vec<String>,
}
impl PemCertificateList {
    pub fn new(certificate_source: CertificateSource, certificates: Vec<String>) -> Self {
        Self {
            certificate_source,
            certificates,
        }
    }
}
#[doc = "Pipeline topology describes the processing steps to be applied when processing media for a particular outcome. The topology should be defined according to the scenario to be achieved and can be reused across many pipeline instances which share the same processing characteristics. For instance, a pipeline topology which acquires data from a RTSP camera, process it with an specific AI model and stored the data on the cloud can be reused across many different cameras, as long as the same processing should be applied across all the cameras. Individual instance properties can be defined through the use of user-defined parameters, which allow for a topology to be parameterized, thus allowing individual pipelines to refer to different values, such as individual cameras RTSP endpoints and credentials. Overall a topology is composed of the following:\r\n\r\n  - Parameters: list of user defined parameters that can be references across the topology nodes.\r\n  - Sources: list of one or more data sources nodes such as an RTSP source which allows for media to be ingested from cameras.\r\n  - Processors: list of nodes which perform data analysis or transformations.\r\n  -Sinks: list of one or more data sinks which allow for data to be stored or exported to other destinations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTopology {
    #[doc = "Pipeline topology unique identifier."]
    pub name: String,
    #[doc = "Read-only system metadata associated with a resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Pipeline topology properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PipelineTopologyProperties>,
}
impl PipelineTopology {
    pub fn new(name: String) -> Self {
        Self {
            name,
            system_data: None,
            properties: None,
        }
    }
}
#[doc = "A collection of pipeline topologies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTopologyCollection {
    #[doc = "List of pipeline topologies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PipelineTopology>,
    #[doc = "A continuation token to be used in subsequent calls when enumerating through the collection. This is returned when the collection results won't fit in a single response."]
    #[serde(rename = "@continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl PipelineTopologyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deletes an existing pipeline topology."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTopologyDeleteRequest {
    #[serde(flatten)]
    pub method_request_empty_body_base: MethodRequestEmptyBodyBase,
}
impl PipelineTopologyDeleteRequest {
    pub fn new(method_request_empty_body_base: MethodRequestEmptyBodyBase) -> Self {
        Self {
            method_request_empty_body_base,
        }
    }
}
#[doc = "Retrieves an existing pipeline topology."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTopologyGetRequest {
    #[serde(flatten)]
    pub method_request_empty_body_base: MethodRequestEmptyBodyBase,
}
impl PipelineTopologyGetRequest {
    pub fn new(method_request_empty_body_base: MethodRequestEmptyBodyBase) -> Self {
        Self {
            method_request_empty_body_base,
        }
    }
}
#[doc = "List all existing pipeline topologies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTopologyListRequest {
    #[serde(flatten)]
    pub method_request: MethodRequest,
}
impl PipelineTopologyListRequest {
    pub fn new(method_request: MethodRequest) -> Self {
        Self { method_request }
    }
}
#[doc = "Pipeline topology properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTopologyProperties {
    #[doc = "An optional description of the pipeline topology. It is recommended that the expected use of the topology to be described here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of the topology parameter declarations. Parameters declared here can be referenced throughout the topology nodes through the use of \"${PARAMETER_NAME}\" string pattern. Parameters can have optional default values and can later be defined in individual instances of the pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterDeclaration>,
    #[doc = "List of the topology source nodes. Source nodes enable external data to be ingested by the pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceNodeBase>,
    #[doc = "List of the topology processor nodes. Processor nodes enable pipeline data to be analyzed, processed or transformed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processors: Vec<ProcessorNodeBase>,
    #[doc = "List of the topology sink nodes. Sink nodes allow pipeline data to be stored or exported."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sinks: Vec<SinkNodeBase>,
}
impl PipelineTopologyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Creates a new pipeline topology or updates an existing one."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTopologySetRequest {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    #[doc = "Pipeline topology describes the processing steps to be applied when processing media for a particular outcome. The topology should be defined according to the scenario to be achieved and can be reused across many pipeline instances which share the same processing characteristics. For instance, a pipeline topology which acquires data from a RTSP camera, process it with an specific AI model and stored the data on the cloud can be reused across many different cameras, as long as the same processing should be applied across all the cameras. Individual instance properties can be defined through the use of user-defined parameters, which allow for a topology to be parameterized, thus allowing individual pipelines to refer to different values, such as individual cameras RTSP endpoints and credentials. Overall a topology is composed of the following:\r\n\r\n  - Parameters: list of user defined parameters that can be references across the topology nodes.\r\n  - Sources: list of one or more data sources nodes such as an RTSP source which allows for media to be ingested from cameras.\r\n  - Processors: list of nodes which perform data analysis or transformations.\r\n  -Sinks: list of one or more data sinks which allow for data to be stored or exported to other destinations."]
    #[serde(rename = "pipelineTopology")]
    pub pipeline_topology: PipelineTopology,
}
impl PipelineTopologySetRequest {
    pub fn new(method_request: MethodRequest, pipeline_topology: PipelineTopology) -> Self {
        Self {
            method_request,
            pipeline_topology,
        }
    }
}
#[doc = "Pipeline topology resource representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTopologySetRequestBody {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    #[serde(flatten)]
    pub pipeline_topology: PipelineTopology,
}
impl PipelineTopologySetRequestBody {
    pub fn new(method_request: MethodRequest, pipeline_topology: PipelineTopology) -> Self {
        Self {
            method_request,
            pipeline_topology,
        }
    }
}
#[doc = "Base class for topology processor nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorNodeBase {
    #[doc = "Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "Node name. Must be unique within the topology."]
    pub name: String,
    #[doc = "An array of upstream node references within the topology to be used as inputs for this node."]
    pub inputs: Vec<NodeInput>,
}
impl ProcessorNodeBase {
    pub fn new(type_: String, name: String, inputs: Vec<NodeInput>) -> Self {
        Self { type_, name, inputs }
    }
}
#[doc = "RTSP source allows for media from an RTSP camera or generic RTSP server to be ingested into a live pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RtspSource {
    #[serde(flatten)]
    pub source_node_base: SourceNodeBase,
    #[doc = "Network transport utilized by the RTSP and RTP exchange: TCP or HTTP. When using TCP, the RTP packets are interleaved on the TCP RTSP connection. When using HTTP, the RTSP messages are exchanged through long lived HTTP connections, and the RTP packages are interleaved in the HTTP connections alongside the RTSP messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<rtsp_source::Transport>,
    #[doc = "Base class for endpoints."]
    pub endpoint: EndpointBase,
}
impl RtspSource {
    pub fn new(source_node_base: SourceNodeBase, endpoint: EndpointBase) -> Self {
        Self {
            source_node_base,
            transport: None,
            endpoint,
        }
    }
}
pub mod rtsp_source {
    use super::*;
    #[doc = "Network transport utilized by the RTSP and RTP exchange: TCP or HTTP. When using TCP, the RTP packets are interleaved on the TCP RTSP connection. When using HTTP, the RTSP messages are exchanged through long lived HTTP connections, and the RTP packages are interleaved in the HTTP connections alongside the RTSP messages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Transport")]
    pub enum Transport {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Transport {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Transport {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Transport {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Transport", 0u32, "http"),
                Self::Tcp => serializer.serialize_unit_variant("Transport", 1u32, "tcp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines how often media is submitted to the extension plugin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SamplingOptions {
    #[doc = "When set to 'true', prevents frames without upstream inference data to be sent to the extension plugin. This is useful to limit the frames sent to the extension to pre-analyzed frames only. For example, when used downstream from a motion detector, this can enable for only frames in which motion has been detected to be further analyzed."]
    #[serde(rename = "skipSamplesWithoutAnnotation", default, skip_serializing_if = "Option::is_none")]
    pub skip_samples_without_annotation: Option<String>,
    #[doc = "Maximum rate of samples submitted to the extension. This prevents an extension plugin to be overloaded with data."]
    #[serde(rename = "maximumSamplesPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub maximum_samples_per_second: Option<String>,
}
impl SamplingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A signal gate determines when to block (gate) incoming media, and when to allow it through. It gathers input events over the activationEvaluationWindow, and determines whether to open or close the gate. See https://aka.ms/ava-signalgate for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalGateProcessor {
    #[serde(flatten)]
    pub processor_node_base: ProcessorNodeBase,
    #[doc = "The period of time over which the gate gathers input events before evaluating them."]
    #[serde(rename = "activationEvaluationWindow", default, skip_serializing_if = "Option::is_none")]
    pub activation_evaluation_window: Option<String>,
    #[doc = "Signal offset once the gate is activated (can be negative). It determines the how much farther behind of after the signal will be let through based on the activation time. A negative offset indicates that data prior the activation time must be included on the signal that is let through, once the gate is activated. When used upstream of a file or video sink, this allows for scenarios such as recording buffered media prior an event, such as: record video 5 seconds prior motions is detected."]
    #[serde(rename = "activationSignalOffset", default, skip_serializing_if = "Option::is_none")]
    pub activation_signal_offset: Option<String>,
    #[doc = "The minimum period for which the gate remains open in the absence of subsequent triggers (events). When used upstream of a file or video sink, it determines the minimum length of the recorded video clip."]
    #[serde(rename = "minimumActivationTime", default, skip_serializing_if = "Option::is_none")]
    pub minimum_activation_time: Option<String>,
    #[doc = "The maximum period for which the gate remains open in the presence of subsequent triggers (events). When used upstream of a file or video sink, it determines the maximum length of the recorded video clip."]
    #[serde(rename = "maximumActivationTime", default, skip_serializing_if = "Option::is_none")]
    pub maximum_activation_time: Option<String>,
}
impl SignalGateProcessor {
    pub fn new(processor_node_base: ProcessorNodeBase) -> Self {
        Self {
            processor_node_base,
            activation_evaluation_window: None,
            activation_signal_offset: None,
            minimum_activation_time: None,
            maximum_activation_time: None,
        }
    }
}
#[doc = "Base class for topology sink nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SinkNodeBase {
    #[doc = "Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "Node name. Must be unique within the topology."]
    pub name: String,
    #[doc = "An array of upstream node references within the topology to be used as inputs for this node."]
    pub inputs: Vec<NodeInput>,
}
impl SinkNodeBase {
    pub fn new(type_: String, name: String, inputs: Vec<NodeInput>) -> Self {
        Self { type_, name, inputs }
    }
}
#[doc = "Base class for topology source nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceNodeBase {
    #[doc = "Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "Node name. Must be unique within the topology."]
    pub name: String,
}
impl SourceNodeBase {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "Defines a Spatial Analysis custom operation. This requires the Azure Cognitive Services Spatial analysis module to be deployed alongside the Video Analyzer module, please see https://aka.ms/ava-spatial-analysis for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisCustomOperation {
    #[serde(flatten)]
    pub spatial_analysis_operation_base: SpatialAnalysisOperationBase,
    #[doc = "Custom configuration to pass to the Azure Cognitive Services Spatial Analysis module."]
    #[serde(rename = "extensionConfiguration")]
    pub extension_configuration: String,
}
impl SpatialAnalysisCustomOperation {
    pub fn new(spatial_analysis_operation_base: SpatialAnalysisOperationBase, extension_configuration: String) -> Self {
        Self {
            spatial_analysis_operation_base,
            extension_configuration,
        }
    }
}
#[doc = "Base class for Azure Cognitive Services Spatial Analysis operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisOperationBase {
    #[doc = "The Type discriminator for the derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl SpatialAnalysisOperationBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Defines the Azure Cognitive Services Spatial Analysis operation eventing configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpatialAnalysisOperationEventBase {
    #[doc = "The event threshold."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
    #[doc = "The operation focus type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus: Option<spatial_analysis_operation_event_base::Focus>,
}
impl SpatialAnalysisOperationEventBase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spatial_analysis_operation_event_base {
    use super::*;
    #[doc = "The operation focus type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Focus")]
    pub enum Focus {
        #[serde(rename = "center")]
        Center,
        #[serde(rename = "bottomCenter")]
        BottomCenter,
        #[serde(rename = "footprint")]
        Footprint,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Focus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Focus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Focus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Center => serializer.serialize_unit_variant("Focus", 0u32, "center"),
                Self::BottomCenter => serializer.serialize_unit_variant("Focus", 1u32, "bottomCenter"),
                Self::Footprint => serializer.serialize_unit_variant("Focus", 2u32, "footprint"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines a Spatial Analysis person count operation eventing configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpatialAnalysisPersonCountEvent {
    #[serde(flatten)]
    pub spatial_analysis_operation_event_base: SpatialAnalysisOperationEventBase,
    #[doc = "The event trigger type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<spatial_analysis_person_count_event::Trigger>,
    #[doc = "The event or interval output frequency."]
    #[serde(rename = "outputFrequency", default, skip_serializing_if = "Option::is_none")]
    pub output_frequency: Option<String>,
}
impl SpatialAnalysisPersonCountEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spatial_analysis_person_count_event {
    use super::*;
    #[doc = "The event trigger type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Trigger")]
    pub enum Trigger {
        #[serde(rename = "event")]
        Event,
        #[serde(rename = "interval")]
        Interval,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Trigger {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Trigger {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Trigger {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Event => serializer.serialize_unit_variant("Trigger", 0u32, "event"),
                Self::Interval => serializer.serialize_unit_variant("Trigger", 1u32, "interval"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines a Spatial Analysis person count operation. This requires the Azure Cognitive Services Spatial analysis module to be deployed alongside the Video Analyzer module, please see https://aka.ms/ava-spatial-analysis for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisPersonCountOperation {
    #[serde(flatten)]
    pub spatial_analysis_typed_operation_base: SpatialAnalysisTypedOperationBase,
    #[doc = "The list of zones and optional events."]
    pub zones: Vec<SpatialAnalysisPersonCountZoneEvents>,
}
impl SpatialAnalysisPersonCountOperation {
    pub fn new(
        spatial_analysis_typed_operation_base: SpatialAnalysisTypedOperationBase,
        zones: Vec<SpatialAnalysisPersonCountZoneEvents>,
    ) -> Self {
        Self {
            spatial_analysis_typed_operation_base,
            zones,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisPersonCountZoneEvents {
    #[doc = "Describes the named polygon."]
    pub zone: NamedPolygonBase,
    #[doc = "The event configuration."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<SpatialAnalysisPersonCountEvent>,
}
impl SpatialAnalysisPersonCountZoneEvents {
    pub fn new(zone: NamedPolygonBase) -> Self {
        Self { zone, events: Vec::new() }
    }
}
#[doc = "Defines a Spatial Analysis person distance operation eventing configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpatialAnalysisPersonDistanceEvent {
    #[serde(flatten)]
    pub spatial_analysis_operation_event_base: SpatialAnalysisOperationEventBase,
    #[doc = "The event trigger type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<spatial_analysis_person_distance_event::Trigger>,
    #[doc = "The event or interval output frequency."]
    #[serde(rename = "outputFrequency", default, skip_serializing_if = "Option::is_none")]
    pub output_frequency: Option<String>,
    #[doc = "The minimum distance threshold"]
    #[serde(rename = "minimumDistanceThreshold", default, skip_serializing_if = "Option::is_none")]
    pub minimum_distance_threshold: Option<String>,
    #[doc = "The maximum distance threshold"]
    #[serde(rename = "maximumDistanceThreshold", default, skip_serializing_if = "Option::is_none")]
    pub maximum_distance_threshold: Option<String>,
}
impl SpatialAnalysisPersonDistanceEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spatial_analysis_person_distance_event {
    use super::*;
    #[doc = "The event trigger type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Trigger")]
    pub enum Trigger {
        #[serde(rename = "event")]
        Event,
        #[serde(rename = "interval")]
        Interval,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Trigger {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Trigger {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Trigger {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Event => serializer.serialize_unit_variant("Trigger", 0u32, "event"),
                Self::Interval => serializer.serialize_unit_variant("Trigger", 1u32, "interval"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines a Spatial Analysis person distance operation. This requires the Azure Cognitive Services Spatial analysis module to be deployed alongside the Video Analyzer module, please see https://aka.ms/ava-spatial-analysis for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisPersonDistanceOperation {
    #[serde(flatten)]
    pub spatial_analysis_typed_operation_base: SpatialAnalysisTypedOperationBase,
    #[doc = "The list of zones with optional events."]
    pub zones: Vec<SpatialAnalysisPersonDistanceZoneEvents>,
}
impl SpatialAnalysisPersonDistanceOperation {
    pub fn new(
        spatial_analysis_typed_operation_base: SpatialAnalysisTypedOperationBase,
        zones: Vec<SpatialAnalysisPersonDistanceZoneEvents>,
    ) -> Self {
        Self {
            spatial_analysis_typed_operation_base,
            zones,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisPersonDistanceZoneEvents {
    #[doc = "Describes the named polygon."]
    pub zone: NamedPolygonBase,
    #[doc = "The event configuration."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<SpatialAnalysisPersonDistanceEvent>,
}
impl SpatialAnalysisPersonDistanceZoneEvents {
    pub fn new(zone: NamedPolygonBase) -> Self {
        Self { zone, events: Vec::new() }
    }
}
#[doc = "Defines a Spatial Analysis person line crossing operation eventing configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpatialAnalysisPersonLineCrossingEvent {
    #[serde(flatten)]
    pub spatial_analysis_operation_event_base: SpatialAnalysisOperationEventBase,
}
impl SpatialAnalysisPersonLineCrossingEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisPersonLineCrossingLineEvents {
    #[doc = "Base class for named lines."]
    pub line: NamedLineBase,
    #[doc = "The event configuration."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<SpatialAnalysisPersonLineCrossingEvent>,
}
impl SpatialAnalysisPersonLineCrossingLineEvents {
    pub fn new(line: NamedLineBase) -> Self {
        Self { line, events: Vec::new() }
    }
}
#[doc = "Defines a Spatial Analysis person line crossing operation. This requires the Azure Cognitive Services Spatial analysis module to be deployed alongside the Video Analyzer module, please see https://aka.ms/ava-spatial-analysis for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisPersonLineCrossingOperation {
    #[serde(flatten)]
    pub spatial_analysis_typed_operation_base: SpatialAnalysisTypedOperationBase,
    #[doc = "The list of lines with optional events."]
    pub lines: Vec<SpatialAnalysisPersonLineCrossingLineEvents>,
}
impl SpatialAnalysisPersonLineCrossingOperation {
    pub fn new(
        spatial_analysis_typed_operation_base: SpatialAnalysisTypedOperationBase,
        lines: Vec<SpatialAnalysisPersonLineCrossingLineEvents>,
    ) -> Self {
        Self {
            spatial_analysis_typed_operation_base,
            lines,
        }
    }
}
#[doc = "Defines a Spatial Analysis person crossing zone operation eventing configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpatialAnalysisPersonZoneCrossingEvent {
    #[serde(flatten)]
    pub spatial_analysis_operation_event_base: SpatialAnalysisOperationEventBase,
    #[doc = "The event type."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<spatial_analysis_person_zone_crossing_event::EventType>,
}
impl SpatialAnalysisPersonZoneCrossingEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spatial_analysis_person_zone_crossing_event {
    use super::*;
    #[doc = "The event type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EventType")]
    pub enum EventType {
        #[serde(rename = "zoneCrossing")]
        ZoneCrossing,
        #[serde(rename = "zoneDwellTime")]
        ZoneDwellTime,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EventType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EventType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EventType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ZoneCrossing => serializer.serialize_unit_variant("EventType", 0u32, "zoneCrossing"),
                Self::ZoneDwellTime => serializer.serialize_unit_variant("EventType", 1u32, "zoneDwellTime"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines a Spatial Analysis person zone crossing operation. This requires the Azure Cognitive Services Spatial analysis module to be deployed alongside the Video Analyzer module, please see https://aka.ms/ava-spatial-analysis for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisPersonZoneCrossingOperation {
    #[serde(flatten)]
    pub spatial_analysis_typed_operation_base: SpatialAnalysisTypedOperationBase,
    #[doc = "The list of zones with optional events."]
    pub zones: Vec<SpatialAnalysisPersonZoneCrossingZoneEvents>,
}
impl SpatialAnalysisPersonZoneCrossingOperation {
    pub fn new(
        spatial_analysis_typed_operation_base: SpatialAnalysisTypedOperationBase,
        zones: Vec<SpatialAnalysisPersonZoneCrossingZoneEvents>,
    ) -> Self {
        Self {
            spatial_analysis_typed_operation_base,
            zones,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisPersonZoneCrossingZoneEvents {
    #[doc = "Describes the named polygon."]
    pub zone: NamedPolygonBase,
    #[doc = "The event configuration."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<SpatialAnalysisPersonZoneCrossingEvent>,
}
impl SpatialAnalysisPersonZoneCrossingZoneEvents {
    pub fn new(zone: NamedPolygonBase) -> Self {
        Self { zone, events: Vec::new() }
    }
}
#[doc = "Base class for Azure Cognitive Services Spatial Analysis typed operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnalysisTypedOperationBase {
    #[serde(flatten)]
    pub spatial_analysis_operation_base: SpatialAnalysisOperationBase,
    #[doc = "If set to 'true', enables debugging mode for this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<String>,
    #[doc = "Advanced camera configuration."]
    #[serde(rename = "cameraConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub camera_configuration: Option<String>,
    #[doc = "Advanced detector node configuration."]
    #[serde(rename = "detectorNodeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub detector_node_configuration: Option<String>,
    #[doc = "If set to 'true', enables face mask detection for this operation."]
    #[serde(rename = "enableFaceMaskClassifier", default, skip_serializing_if = "Option::is_none")]
    pub enable_face_mask_classifier: Option<String>,
}
impl SpatialAnalysisTypedOperationBase {
    pub fn new(spatial_analysis_operation_base: SpatialAnalysisOperationBase) -> Self {
        Self {
            spatial_analysis_operation_base,
            debug: None,
            camera_configuration: None,
            detector_node_configuration: None,
            enable_face_mask_classifier: None,
        }
    }
}
#[doc = "Read-only system metadata associated with a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "Date and time when this resource was first created. Value is represented in UTC according to the ISO8601 date format."]
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Date and time when this resource was last modified. Value is represented in UTC according to the ISO8601 date format."]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "TLS endpoint describes an endpoint that the pipeline can connect to over TLS transport (data is encrypted in transit)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TlsEndpoint {
    #[serde(flatten)]
    pub endpoint_base: EndpointBase,
    #[doc = "Base class for certificate sources."]
    #[serde(rename = "trustedCertificates", default, skip_serializing_if = "Option::is_none")]
    pub trusted_certificates: Option<CertificateSource>,
    #[doc = "Options for controlling the validation of TLS endpoints."]
    #[serde(rename = "validationOptions", default, skip_serializing_if = "Option::is_none")]
    pub validation_options: Option<TlsValidationOptions>,
}
impl TlsEndpoint {
    pub fn new(endpoint_base: EndpointBase) -> Self {
        Self {
            endpoint_base,
            trusted_certificates: None,
            validation_options: None,
        }
    }
}
#[doc = "Options for controlling the validation of TLS endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TlsValidationOptions {
    #[doc = "When set to 'true' causes the certificate subject name validation to be skipped. Default is 'false'."]
    #[serde(rename = "ignoreHostname", default, skip_serializing_if = "Option::is_none")]
    pub ignore_hostname: Option<String>,
    #[doc = "When set to 'true' causes the certificate chain trust validation to be skipped. Default is 'false'."]
    #[serde(rename = "ignoreSignature", default, skip_serializing_if = "Option::is_none")]
    pub ignore_signature: Option<String>,
}
impl TlsValidationOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Unsecured endpoint describes an endpoint that the pipeline can connect to over clear transport (no encryption in transit)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnsecuredEndpoint {
    #[serde(flatten)]
    pub endpoint_base: EndpointBase,
}
impl UnsecuredEndpoint {
    pub fn new(endpoint_base: EndpointBase) -> Self {
        Self { endpoint_base }
    }
}
#[doc = "Username and password credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsernamePasswordCredentials {
    #[serde(flatten)]
    pub credentials_base: CredentialsBase,
    #[doc = "Username to be presented as part of the credentials."]
    pub username: String,
    #[doc = "Password to be presented as part of the credentials. It is recommended that this value is parameterized as a secret string in order to prevent this value to be returned as part of the resource on API requests."]
    pub password: String,
}
impl UsernamePasswordCredentials {
    pub fn new(credentials_base: CredentialsBase, username: String, password: String) -> Self {
        Self {
            credentials_base,
            username,
            password,
        }
    }
}
#[doc = "Optional video properties to be used in case a new video resource needs to be created on the service. These will not take effect if the video already exists."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoCreationProperties {
    #[doc = "Optional video title provided by the user. Value can be up to 256 characters long."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Optional video description provided by the user. Value can be up to 2048 characters long."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Video segment length indicates the length of individual video files (segments) which are persisted to storage. Smaller segments provide lower archive playback latency but generate larger volume of storage transactions. Larger segments reduce the amount of storage transactions while increasing the archive playback latency. Value must be specified in ISO8601 duration format (i.e. \"PT30S\" equals 30 seconds) and can vary between 30 seconds to 5 minutes, in 30 seconds increments. Changing this value after the video is initially created can lead to errors when uploading media to the archive. Default value is 30 seconds."]
    #[serde(rename = "segmentLength", default, skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<String>,
}
impl VideoCreationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Video sink allows for video and audio to be recorded to the Video Analyzer service. The recorded video can be played from anywhere and further managed from the cloud. Due to security reasons, a given Video Analyzer edge module instance can only record content to new video entries, or existing video entries previously recorded by the same module. Any attempt to record content to an existing video which has not been created by the same module instance will result in failure to record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoSink {
    #[serde(flatten)]
    pub sink_node_base: SinkNodeBase,
    #[doc = "Name of a new or existing Video Analyzer video resource used for the media recording."]
    #[serde(rename = "videoName")]
    pub video_name: String,
    #[doc = "Optional video properties to be used in case a new video resource needs to be created on the service. These will not take effect if the video already exists."]
    #[serde(rename = "videoCreationProperties", default, skip_serializing_if = "Option::is_none")]
    pub video_creation_properties: Option<VideoCreationProperties>,
    #[doc = "Path to a local file system directory for caching of temporary media files. This will also be used to store content which cannot be immediately uploaded to Azure due to Internet connectivity issues."]
    #[serde(rename = "localMediaCachePath")]
    pub local_media_cache_path: String,
    #[doc = "Maximum amount of disk space that can be used for caching of temporary media files. Once this limit is reached, the oldest segments of the media archive will be continuously deleted in order to make space for new media, thus leading to gaps in the cloud recorded content."]
    #[serde(rename = "localMediaCacheMaximumSizeMiB")]
    pub local_media_cache_maximum_size_mi_b: String,
}
impl VideoSink {
    pub fn new(
        sink_node_base: SinkNodeBase,
        video_name: String,
        local_media_cache_path: String,
        local_media_cache_maximum_size_mi_b: String,
    ) -> Self {
        Self {
            sink_node_base,
            video_name,
            video_creation_properties: None,
            local_media_cache_path,
            local_media_cache_maximum_size_mi_b,
        }
    }
}
