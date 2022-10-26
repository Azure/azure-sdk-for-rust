#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Schema of application data resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationData {
    #[doc = "Application product details."]
    #[serde(
        rename = "applicationProductDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub application_product_details: Vec<ApplicationProductDetail>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgMaterial", default, skip_serializing_if = "Option::is_none")]
    pub avg_material: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "totalMaterial", default, skip_serializing_if = "Option::is_none")]
    pub total_material: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<Measure>,
    #[doc = "Modified date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ.\r\nNote: this will be specified by the source provider itself."]
    #[serde(rename = "operationModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Start date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "operationStartDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "operationEndDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Link for attachments."]
    #[serde(rename = "attachmentsLink", default, skip_serializing_if = "Option::is_none")]
    pub attachments_link: Option<String>,
    #[doc = "Optional boundary ID of the field for which operation was applied."]
    #[serde(rename = "associatedBoundaryId", default, skip_serializing_if = "Option::is_none")]
    pub associated_boundary_id: Option<String>,
    #[doc = "Farmer ID which belongs to the operation data."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ApplicationData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationDataListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ApplicationData>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationDataListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApplicationDataListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of product used during application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProductDetail {
    #[doc = "Name of the product applied."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "A flag indicating whether product is a carrier for a tank mix."]
    #[serde(rename = "isCarrier", default, skip_serializing_if = "Option::is_none")]
    pub is_carrier: Option<bool>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgMaterial", default, skip_serializing_if = "Option::is_none")]
    pub avg_material: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "totalMaterial", default, skip_serializing_if = "Option::is_none")]
    pub total_material: Option<Measure>,
}
impl ApplicationProductDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of attachment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Attachment {
    #[doc = "Associated Resource id for this attachment."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Associated Resource type for this attachment."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<attachment::ResourceType>,
    #[doc = "Original File Name for this attachment."]
    #[serde(rename = "originalFileName", default, skip_serializing_if = "Option::is_none")]
    pub original_file_name: Option<String>,
    #[doc = "Farmer id for this attachment."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Unique id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date when resource was created."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date when resource was last modified."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl Attachment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod attachment {
    use super::*;
    #[doc = "Associated Resource type for this attachment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceType")]
    pub enum ResourceType {
        Farmer,
        Farm,
        Field,
        SeasonalField,
        Boundary,
        ApplicationData,
        HarvestData,
        TillageData,
        PlantingData,
        PlantTissueAnalysis,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Farmer => serializer.serialize_unit_variant("ResourceType", 0u32, "Farmer"),
                Self::Farm => serializer.serialize_unit_variant("ResourceType", 1u32, "Farm"),
                Self::Field => serializer.serialize_unit_variant("ResourceType", 2u32, "Field"),
                Self::SeasonalField => serializer.serialize_unit_variant("ResourceType", 3u32, "SeasonalField"),
                Self::Boundary => serializer.serialize_unit_variant("ResourceType", 4u32, "Boundary"),
                Self::ApplicationData => serializer.serialize_unit_variant("ResourceType", 5u32, "ApplicationData"),
                Self::HarvestData => serializer.serialize_unit_variant("ResourceType", 6u32, "HarvestData"),
                Self::TillageData => serializer.serialize_unit_variant("ResourceType", 7u32, "TillageData"),
                Self::PlantingData => serializer.serialize_unit_variant("ResourceType", 8u32, "PlantingData"),
                Self::PlantTissueAnalysis => serializer.serialize_unit_variant("ResourceType", 9u32, "PlantTissueAnalysis"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachmentListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Attachment>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AttachmentListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AttachmentListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of biomass model job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BiomassModelJob {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "The id of the boundary object for which biomass is being calculated."]
    #[serde(rename = "boundaryId")]
    pub boundary_id: String,
    #[doc = "The version of the biomass model to be run. Available Value: 1.0 ."]
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    #[doc = "Crop name for biomass model. Available Value: Corn."]
    #[serde(rename = "cropName")]
    pub crop_name: biomass_model_job::CropName,
    #[doc = "Planting datetime for biomass calculations. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "plantingStartDateTime", with = "azure_core::date::rfc3339")]
    pub planting_start_date_time: time::OffsetDateTime,
    #[doc = "End datetime till which biomass will be calculated. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "inferenceEndDateTime", with = "azure_core::date::rfc3339")]
    pub inference_end_date_time: time::OffsetDateTime,
    #[doc = "ExtensionId of weather data. Available values: DTN.ClearAg, DTN.ContentServices."]
    #[serde(rename = "weatherExtensionId")]
    pub weather_extension_id: String,
    #[doc = "Provider of satellite data. Available Value: Microsoft."]
    #[serde(rename = "satelliteProvider")]
    pub satellite_provider: biomass_model_job::SatelliteProvider,
    #[doc = "Source of satellite data. Available Value: Sentinel_2_L2A."]
    #[serde(rename = "satelliteSource")]
    pub satellite_source: biomass_model_job::SatelliteSource,
    #[doc = "ImageResolution in meters. Available values: 10, 20, 60."]
    #[serde(rename = "imageResolution")]
    pub image_resolution: f64,
    #[doc = "ImageFormat. Available value: TIF."]
    #[serde(rename = "imageFormat")]
    pub image_format: biomass_model_job::ImageFormat,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl BiomassModelJob {
    pub fn new(
        farmer_id: String,
        boundary_id: String,
        model_version: String,
        crop_name: biomass_model_job::CropName,
        planting_start_date_time: time::OffsetDateTime,
        inference_end_date_time: time::OffsetDateTime,
        weather_extension_id: String,
        satellite_provider: biomass_model_job::SatelliteProvider,
        satellite_source: biomass_model_job::SatelliteSource,
        image_resolution: f64,
        image_format: biomass_model_job::ImageFormat,
    ) -> Self {
        Self {
            farmer_id,
            boundary_id,
            model_version,
            crop_name,
            planting_start_date_time,
            inference_end_date_time,
            weather_extension_id,
            satellite_provider,
            satellite_source,
            image_resolution,
            image_format,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
pub mod biomass_model_job {
    use super::*;
    #[doc = "Crop name for biomass model. Available Value: Corn."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CropName")]
    pub enum CropName {
        Corn,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CropName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CropName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CropName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Corn => serializer.serialize_unit_variant("CropName", 0u32, "Corn"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provider of satellite data. Available Value: Microsoft."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SatelliteProvider")]
    pub enum SatelliteProvider {
        Microsoft,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SatelliteProvider {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SatelliteProvider {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SatelliteProvider {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Microsoft => serializer.serialize_unit_variant("SatelliteProvider", 0u32, "Microsoft"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Source of satellite data. Available Value: Sentinel_2_L2A."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SatelliteSource")]
    pub enum SatelliteSource {
        #[serde(rename = "Sentinel_2_L2A")]
        Sentinel2L2a,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SatelliteSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SatelliteSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SatelliteSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sentinel2L2a => serializer.serialize_unit_variant("SatelliteSource", 0u32, "Sentinel_2_L2A"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "ImageFormat. Available value: TIF."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ImageFormat")]
    pub enum ImageFormat {
        #[serde(rename = "TIF")]
        Tif,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ImageFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ImageFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ImageFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tif => serializer.serialize_unit_variant("ImageFormat", 0u32, "TIF"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema of boundary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Boundary {
    #[doc = "GeoJSON (For more details: https://geojson.org/). Note: Coordinates are expected in [Longitude, Latitude] format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeoJsonObject>,
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Id of the parent it belongs to."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "Is the boundary primary."]
    #[serde(rename = "isPrimary", default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[doc = "Boundary area in acres."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acreage: Option<f64>,
    #[doc = "Type of the parent it belongs to.\r\ni.e. Field, SeasonalField, Zone, Prescription, PlantTissueAnalysis, ApplicationData, HarvestData, TillageData, PlantingData."]
    #[serde(rename = "parentType", default, skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<boundary::ParentType>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Boundary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod boundary {
    use super::*;
    #[doc = "Type of the parent it belongs to.\r\ni.e. Field, SeasonalField, Zone, Prescription, PlantTissueAnalysis, ApplicationData, HarvestData, TillageData, PlantingData."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ParentType")]
    pub enum ParentType {
        Field,
        SeasonalField,
        Zone,
        Prescription,
        PlantTissueAnalysis,
        ApplicationData,
        PlantingData,
        TillageData,
        HarvestData,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ParentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ParentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ParentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Field => serializer.serialize_unit_variant("ParentType", 0u32, "Field"),
                Self::SeasonalField => serializer.serialize_unit_variant("ParentType", 1u32, "SeasonalField"),
                Self::Zone => serializer.serialize_unit_variant("ParentType", 2u32, "Zone"),
                Self::Prescription => serializer.serialize_unit_variant("ParentType", 3u32, "Prescription"),
                Self::PlantTissueAnalysis => serializer.serialize_unit_variant("ParentType", 4u32, "PlantTissueAnalysis"),
                Self::ApplicationData => serializer.serialize_unit_variant("ParentType", 5u32, "ApplicationData"),
                Self::PlantingData => serializer.serialize_unit_variant("ParentType", 6u32, "PlantingData"),
                Self::TillageData => serializer.serialize_unit_variant("ParentType", 7u32, "TillageData"),
                Self::HarvestData => serializer.serialize_unit_variant("ParentType", 8u32, "HarvestData"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema of boundary resource metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoundaryMetadata {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Id of the parent it belongs to."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "Is the boundary primary."]
    #[serde(rename = "isPrimary", default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[doc = "Boundary area in acres."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acreage: Option<f64>,
    #[doc = "Type of the parent it belongs to."]
    #[serde(rename = "parentType", default, skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<boundary_metadata::ParentType>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl BoundaryMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod boundary_metadata {
    use super::*;
    #[doc = "Type of the parent it belongs to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ParentType")]
    pub enum ParentType {
        Field,
        SeasonalField,
        Zone,
        Prescription,
        PlantTissueAnalysis,
        ApplicationData,
        PlantingData,
        TillageData,
        HarvestData,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ParentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ParentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ParentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Field => serializer.serialize_unit_variant("ParentType", 0u32, "Field"),
                Self::SeasonalField => serializer.serialize_unit_variant("ParentType", 1u32, "SeasonalField"),
                Self::Zone => serializer.serialize_unit_variant("ParentType", 2u32, "Zone"),
                Self::Prescription => serializer.serialize_unit_variant("ParentType", 3u32, "Prescription"),
                Self::PlantTissueAnalysis => serializer.serialize_unit_variant("ParentType", 4u32, "PlantTissueAnalysis"),
                Self::ApplicationData => serializer.serialize_unit_variant("ParentType", 5u32, "ApplicationData"),
                Self::PlantingData => serializer.serialize_unit_variant("ParentType", 6u32, "PlantingData"),
                Self::TillageData => serializer.serialize_unit_variant("ParentType", 7u32, "TillageData"),
                Self::HarvestData => serializer.serialize_unit_variant("ParentType", 8u32, "HarvestData"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoundaryMetadataListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BoundaryMetadata>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BoundaryMetadataListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BoundaryMetadataListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of boundary overlap response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoundaryOverlapResponse {
    #[doc = "Acreage of Main boundary."]
    #[serde(rename = "boundaryAcreage", default, skip_serializing_if = "Option::is_none")]
    pub boundary_acreage: Option<f64>,
    #[doc = "Acreage of other boundary."]
    #[serde(rename = "otherBoundaryAcreage", default, skip_serializing_if = "Option::is_none")]
    pub other_boundary_acreage: Option<f64>,
    #[doc = "Acreage of intersecting boundary."]
    #[serde(rename = "intersectingAcreage", default, skip_serializing_if = "Option::is_none")]
    pub intersecting_acreage: Option<f64>,
}
impl BoundaryOverlapResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of cascade delete job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CascadeDeleteJob {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "The id of the resource."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The type of the resource."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<cascade_delete_job::Status>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl CascadeDeleteJob {
    pub fn new(farmer_id: String, resource_id: String, resource_type: String) -> Self {
        Self {
            farmer_id,
            resource_id,
            resource_type,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
        }
    }
}
pub mod cascade_delete_job {
    use super::*;
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Waiting,
        Running,
        Succeeded,
        Failed,
        Cancelled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Waiting => serializer.serialize_unit_variant("Status", 0u32, "Waiting"),
                Self::Running => serializer.serialize_unit_variant("Status", 1u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 4u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema of crop resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Crop {
    #[doc = "Crop phenotype."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phenotype: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Crop {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CropListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Crop>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CropListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CropListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of crop variety resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CropVariety {
    #[doc = "Ids of the crops it belongs to.\r\nNote: A maximum of 25 crops can be associated with a cropVariety."]
    #[serde(
        rename = "cropIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub crop_ids: Vec<String>,
    #[doc = "CropVariety Brand."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[doc = "CropVariety product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl CropVariety {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CropVarietyListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CropVariety>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CropVarietyListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CropVarietyListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider of satellite data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataProvider")]
pub enum DataProvider {
    Microsoft,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataProvider {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataProvider {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataProvider {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Microsoft => serializer.serialize_unit_variant("DataProvider", 0u32, "Microsoft"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Device API model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Device {
    #[doc = "Id of the associated device data model."]
    #[serde(rename = "deviceDataModelId", default, skip_serializing_if = "Option::is_none")]
    pub device_data_model_id: Option<String>,
    #[doc = "Integration id for the device."]
    #[serde(rename = "integrationId", default, skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    #[doc = "Type of device."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Device hardwareId."]
    #[serde(rename = "hardwareId", default, skip_serializing_if = "Option::is_none")]
    pub hardware_id: Option<String>,
    #[doc = "Interval at which the device sends data in seconds."]
    #[serde(rename = "reportingIntervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub reporting_interval_in_seconds: Option<i32>,
    #[doc = "Parent device Id for this device."]
    #[serde(rename = "parentDeviceId", default, skip_serializing_if = "Option::is_none")]
    pub parent_device_id: Option<String>,
    #[doc = "Location model class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[doc = "Id of the associated sensor partner."]
    #[serde(rename = "sensorPartnerId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_partner_id: Option<String>,
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and\r\nonly string, numeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Device {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DeviceDataModel API model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceDataModel {
    #[doc = "Type of device."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Device manufacturer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[doc = "Device productCode."]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "List of device ports supported."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ports: Vec<Port>,
    #[doc = "Id of the associated sensor partner."]
    #[serde(rename = "sensorPartnerId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_partner_id: Option<String>,
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and\r\nonly string, numeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl DeviceDataModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceDataModelListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeviceDataModel>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceDataModelListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceDataModelListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Device>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error from the Azure AgPlatform service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<Error>,
    #[doc = "Inner error containing list of errors.\r\n<see href=\"https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#innererror--object\">InnerError reference document</see>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Azure AgPlatform service.\r\n<see href=\"https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses\">ErrorResponse reference document.</see>."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "An error from the Azure AgPlatform service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[doc = "Unique trace Id."]
    #[serde(rename = "traceId", default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of farm resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Farm {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Farm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FarmListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Farm>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FarmListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FarmListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of farm operation data ingestion job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FarmOperationDataIngestionJob {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "Authentication provider Id."]
    #[serde(rename = "authProviderId")]
    pub auth_provider_id: String,
    #[doc = "List of operation types for which data needs to be downloaded. Available values: AllOperations, Application, Planting, Harvest, Tillage."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<String>,
    #[doc = "Start Year (Minimum = 2000, Maximum = CurrentYear)."]
    #[serde(rename = "startYear")]
    pub start_year: i32,
    #[doc = "Use this to pull only the incremental changes from the last run."]
    #[serde(rename = "isIncremental", default, skip_serializing_if = "Option::is_none")]
    pub is_incremental: Option<bool>,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl FarmOperationDataIngestionJob {
    pub fn new(farmer_id: String, auth_provider_id: String, start_year: i32) -> Self {
        Self {
            farmer_id,
            auth_provider_id,
            operations: Vec::new(),
            start_year,
            is_incremental: None,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
#[doc = "Schema of farmer resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Farmer {
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Farmer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FarmerListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Farmer>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FarmerListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FarmerListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of field resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Field {
    #[doc = "Id of the associated Farm."]
    #[serde(rename = "farmId", default, skip_serializing_if = "Option::is_none")]
    pub farm_id: Option<String>,
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Primary boundary id."]
    #[serde(rename = "primaryBoundaryId", default, skip_serializing_if = "Option::is_none")]
    pub primary_boundary_id: Option<String>,
    #[doc = "Boundary Ids."]
    #[serde(
        rename = "boundaryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub boundary_ids: Vec<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Field {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Field>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FieldListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FieldListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GeoJSON (For more details: https://geojson.org/). Note: Coordinates are expected in [Longitude, Latitude] format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoJsonObject {
    #[doc = "GeoJSON object type."]
    #[serde(rename = "type")]
    pub type_: GeoJsonObjectType,
}
impl GeoJsonObject {
    pub fn new(type_: GeoJsonObjectType) -> Self {
        Self { type_ }
    }
}
#[doc = "GeoJSON object type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GeoJsonObjectType")]
pub enum GeoJsonObjectType {
    Point,
    Polygon,
    MultiPolygon,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GeoJsonObjectType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GeoJsonObjectType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GeoJsonObjectType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Point => serializer.serialize_unit_variant("GeoJsonObjectType", 0u32, "Point"),
            Self::Polygon => serializer.serialize_unit_variant("GeoJsonObjectType", 1u32, "Polygon"),
            Self::MultiPolygon => serializer.serialize_unit_variant("GeoJsonObjectType", 2u32, "MultiPolygon"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of harvest data resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HarvestData {
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "totalYield", default, skip_serializing_if = "Option::is_none")]
    pub total_yield: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgYield", default, skip_serializing_if = "Option::is_none")]
    pub avg_yield: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "totalWetMass", default, skip_serializing_if = "Option::is_none")]
    pub total_wet_mass: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgWetMass", default, skip_serializing_if = "Option::is_none")]
    pub avg_wet_mass: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgMoisture", default, skip_serializing_if = "Option::is_none")]
    pub avg_moisture: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgSpeed", default, skip_serializing_if = "Option::is_none")]
    pub avg_speed: Option<Measure>,
    #[doc = "Harvest product details."]
    #[serde(
        rename = "harvestProductDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub harvest_product_details: Vec<HarvestProductDetail>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<Measure>,
    #[doc = "Modified date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ.\r\nNote: this will be specified by the source provider itself."]
    #[serde(rename = "operationModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Start date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "operationStartDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "operationEndDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Link for attachments."]
    #[serde(rename = "attachmentsLink", default, skip_serializing_if = "Option::is_none")]
    pub attachments_link: Option<String>,
    #[doc = "Optional boundary ID of the field for which operation was applied."]
    #[serde(rename = "associatedBoundaryId", default, skip_serializing_if = "Option::is_none")]
    pub associated_boundary_id: Option<String>,
    #[doc = "Farmer ID which belongs to the operation data."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl HarvestData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HarvestDataListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<HarvestData>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HarvestDataListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HarvestDataListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of product used during harvesting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HarvestProductDetail {
    #[doc = "Name of the product."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "totalYield", default, skip_serializing_if = "Option::is_none")]
    pub total_yield: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgYield", default, skip_serializing_if = "Option::is_none")]
    pub avg_yield: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgMoisture", default, skip_serializing_if = "Option::is_none")]
    pub avg_moisture: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "totalWetMass", default, skip_serializing_if = "Option::is_none")]
    pub total_wet_mass: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgWetMass", default, skip_serializing_if = "Option::is_none")]
    pub avg_wet_mass: Option<Measure>,
}
impl HarvestProductDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of image file resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFile {
    #[doc = "Link of the image file."]
    #[serde(rename = "fileLink", default, skip_serializing_if = "Option::is_none")]
    pub file_link: Option<String>,
    #[doc = "Name of the image file."]
    pub name: String,
    #[doc = "Supported image formats for scene resource."]
    #[serde(rename = "imageFormat", default, skip_serializing_if = "Option::is_none")]
    pub image_format: Option<ImageFormat>,
    #[doc = "Resolution of image file in meters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<f64>,
}
impl ImageFile {
    pub fn new(name: String) -> Self {
        Self {
            file_link: None,
            name,
            image_format: None,
            resolution: None,
        }
    }
}
#[doc = "Supported image formats for scene resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageFormat")]
pub enum ImageFormat {
    #[serde(rename = "TIF")]
    Tif,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Tif => serializer.serialize_unit_variant("ImageFormat", 0u32, "TIF"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Image Processing Rasterize Job to convert shapefile into tiff file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageProcessingRasterizeJob {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "Shapefile attachment Id."]
    #[serde(rename = "shapefileAttachmentId")]
    pub shapefile_attachment_id: String,
    #[doc = "List of shapefile column names to create raster attachments."]
    #[serde(rename = "shapefileColumnNames")]
    pub shapefile_column_names: Vec<String>,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ImageProcessingRasterizeJob {
    pub fn new(farmer_id: String, shapefile_attachment_id: String, shapefile_column_names: Vec<String>) -> Self {
        Self {
            farmer_id,
            shapefile_attachment_id,
            shapefile_column_names,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
#[doc = "Inner error containing list of errors.\r\n<see href=\"https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#innererror--object\">InnerError reference document</see>."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "Specific error code than was provided by the\r\ncontaining error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Inner error containing list of errors.\r\n<see href=\"https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#innererror--object\">InnerError reference document</see>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of insight resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Insight {
    #[doc = "Id of the associated farmer."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Id of the associated model."]
    #[serde(rename = "modelId", default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[doc = "Resource type associated with the record."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<insight::ResourceType>,
    #[doc = "Id of the associated resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Version of the associated model."]
    #[serde(rename = "modelVersion", default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    #[doc = "Gets link for attachments."]
    #[serde(rename = "attachmentsLink", default, skip_serializing_if = "Option::is_none")]
    pub attachments_link: Option<String>,
    #[doc = "Start date to which the insight is related."]
    #[serde(rename = "insightStartDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub insight_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End date to which the insight is related."]
    #[serde(rename = "insightEndDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub insight_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Measures to capture insights results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measures: Option<serde_json::Value>,
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and\r\nonly string, numeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Insight {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod insight {
    use super::*;
    #[doc = "Resource type associated with the record."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceType")]
    pub enum ResourceType {
        Farmer,
        Farm,
        Field,
        SeasonalField,
        Boundary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Farmer => serializer.serialize_unit_variant("ResourceType", 0u32, "Farmer"),
                Self::Farm => serializer.serialize_unit_variant("ResourceType", 1u32, "Farm"),
                Self::Field => serializer.serialize_unit_variant("ResourceType", 2u32, "Field"),
                Self::SeasonalField => serializer.serialize_unit_variant("ResourceType", 3u32, "SeasonalField"),
                Self::Boundary => serializer.serialize_unit_variant("ResourceType", 4u32, "Boundary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema of insight attachment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsightAttachment {
    #[doc = "InsightID for this InsightAttachment."]
    #[serde(rename = "insightId")]
    pub insight_id: String,
    #[doc = "ModelID for this InsightAttachment."]
    #[serde(rename = "modelId", default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[doc = "Associated Resource type for this attachment."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<insight_attachment::ResourceType>,
    #[doc = "Associated Resource id for this attachment."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Original File Name for this attachment."]
    #[serde(rename = "originalFileName", default, skip_serializing_if = "Option::is_none")]
    pub original_file_name: Option<String>,
    #[doc = "Farmer id for this attachment."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Unique id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date when resource was created."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date when resource was last modified."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl InsightAttachment {
    pub fn new(insight_id: String) -> Self {
        Self {
            insight_id,
            model_id: None,
            resource_type: None,
            resource_id: None,
            original_file_name: None,
            farmer_id: None,
            id: None,
            status: None,
            created_date_time: None,
            modified_date_time: None,
            source: None,
            name: None,
            description: None,
            e_tag: None,
        }
    }
}
pub mod insight_attachment {
    use super::*;
    #[doc = "Associated Resource type for this attachment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceType")]
    pub enum ResourceType {
        Farmer,
        Farm,
        Field,
        SeasonalField,
        Boundary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Farmer => serializer.serialize_unit_variant("ResourceType", 0u32, "Farmer"),
                Self::Farm => serializer.serialize_unit_variant("ResourceType", 1u32, "Farm"),
                Self::Field => serializer.serialize_unit_variant("ResourceType", 2u32, "Field"),
                Self::SeasonalField => serializer.serialize_unit_variant("ResourceType", 3u32, "SeasonalField"),
                Self::Boundary => serializer.serialize_unit_variant("ResourceType", 4u32, "Boundary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InsightAttachmentListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InsightAttachment>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InsightAttachmentListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InsightAttachmentListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InsightListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Insight>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InsightListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InsightListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authentication via connection string to IoTHub devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTHubDeviceAuthentication {
    #[doc = "Primary connection string of the ioTHub device."]
    #[serde(rename = "primaryDeviceConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_device_connection_string: Option<String>,
    #[doc = "Secondary connection string of the ioTHub device."]
    #[serde(rename = "secondaryDeviceConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_device_connection_string: Option<String>,
}
impl IoTHubDeviceAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model for representing LabDetails object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabDetails {
    #[doc = "Code of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Address of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
impl LabDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Location model class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    #[doc = "Latitude of the location."]
    pub latitude: f64,
    #[doc = "Longitude of the location."]
    pub longitude: f64,
}
impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
}
#[doc = "Api Model for ManagementZone object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementZone {
    #[doc = "Farmer Id associated with the ManagementZone."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Type of the ManagementZone."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Season Id associated with the ManagementZone."]
    #[serde(rename = "seasonId", default, skip_serializing_if = "Option::is_none")]
    pub season_id: Option<String>,
    #[doc = "Crop Id associated with the ManagementZone."]
    #[serde(rename = "cropId", default, skip_serializing_if = "Option::is_none")]
    pub crop_id: Option<String>,
    #[doc = "Field Id associated with the ManagementZone."]
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ManagementZone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementZoneListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagementZone>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagementZoneListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagementZoneListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for storing measurement reading and unit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Measure {
    #[doc = "Data unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Data value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl Measure {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MultiPolygon geometry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiPolygon {
    #[serde(flatten)]
    pub geo_json_object: GeoJsonObject,
    #[serde(flatten)]
    pub multi_polygon_coordinates: MultiPolygonCoordinates,
}
impl MultiPolygon {
    pub fn new(geo_json_object: GeoJsonObject, multi_polygon_coordinates: MultiPolygonCoordinates) -> Self {
        Self {
            geo_json_object,
            multi_polygon_coordinates,
        }
    }
}
#[doc = "Schema of multi polygon coordinates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiPolygonCoordinates {
    #[doc = "Gets or sets coordinates of GeoJSON Object.\r\nIt must be an array of polygons, each polygon contains list of linear rings.\r\nFor Polygons with more than one of these rings, the first MUST be the exterior ring,\r\nand any others MUST be interior rings."]
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}
impl MultiPolygonCoordinates {
    pub fn new(coordinates: Vec<Vec<Vec<Vec<f64>>>>) -> Self {
        Self { coordinates }
    }
}
#[doc = "Api Model for nutrient analysis object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NutrientAnalysis {
    #[doc = "Farmer id for this nutrient analysis."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Parent id for this nutrient analysis."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "Parent type for this nutrient analysis.\r\ni.e. PlantTissueAnalysis."]
    #[serde(rename = "parentType", default, skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<nutrient_analysis::ParentType>,
    #[doc = "Unit for this nutrient analysis."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Value for this nutrient analysis."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "Reference value low for this nutrient analysis."]
    #[serde(rename = "referenceValueLow", default, skip_serializing_if = "Option::is_none")]
    pub reference_value_low: Option<f64>,
    #[doc = "Reference value high for this nutrient analysis."]
    #[serde(rename = "referenceValueHigh", default, skip_serializing_if = "Option::is_none")]
    pub reference_value_high: Option<f64>,
    #[doc = "Classification for this nutrient analysis."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[doc = "Recommendation for this nutrient analysis."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[doc = "Products for this nutrient analysis."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub products: Vec<ProductDetails>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl NutrientAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nutrient_analysis {
    use super::*;
    #[doc = "Parent type for this nutrient analysis.\r\ni.e. PlantTissueAnalysis."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ParentType")]
    pub enum ParentType {
        PlantTissueAnalysis,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ParentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ParentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ParentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PlantTissueAnalysis => serializer.serialize_unit_variant("ParentType", 0u32, "PlantTissueAnalysis"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NutrientAnalysisListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NutrientAnalysis>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NutrientAnalysisListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NutrientAnalysisListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Get OAuth config query parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthConnectRequest {
    #[doc = "Id of the farmer."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "Id of the OAuthProvider."]
    #[serde(rename = "oAuthProviderId")]
    pub o_auth_provider_id: String,
    #[doc = "Link to redirect the user to, at the end of the oauth flow."]
    #[serde(rename = "userRedirectLink")]
    pub user_redirect_link: String,
    #[doc = "State to provide back when redirecting the user, at the end of the oauth flow."]
    #[serde(rename = "userRedirectState", default, skip_serializing_if = "Option::is_none")]
    pub user_redirect_state: Option<String>,
}
impl OAuthConnectRequest {
    pub fn new(farmer_id: String, o_auth_provider_id: String, user_redirect_link: String) -> Self {
        Self {
            farmer_id,
            o_auth_provider_id,
            user_redirect_link,
            user_redirect_state: None,
        }
    }
}
#[doc = "Schema of OAuth provider resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuthProvider {
    #[doc = "OAuth App Id for given OAuth Provider."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "OAuth App secret for given Provider.\r\nNote: Won't be sent in response."]
    #[serde(rename = "appSecret", default, skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
    #[doc = "OAuth Api key for given Provider.\r\nNote: currently Applicable to Climate provider. Won't be sent in response."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "An optional flag to determine if the App is ready to be used for Production scenarios in the provider side or not. (Default value: false)\r\nNote: Currently applicable for JohnDeere."]
    #[serde(rename = "isProductionApp", default, skip_serializing_if = "Option::is_none")]
    pub is_production_app: Option<bool>,
    #[doc = "Unique OAuth provider ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and\r\nonly string, numeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OAuthProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of oauth provider cascade delete job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthProviderCascadeDeleteJob {
    #[doc = "The id of the oauth provider."]
    #[serde(rename = "oauthProviderId")]
    pub oauth_provider_id: String,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OAuthProviderCascadeDeleteJob {
    pub fn new(oauth_provider_id: String) -> Self {
        Self {
            oauth_provider_id,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuthProviderListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OAuthProvider>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OAuthProviderListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OAuthProviderListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of OAuth token resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthToken {
    #[doc = "Farmer ID for this OAuth config."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "ID of the OAuth provider resource containing app information."]
    #[serde(rename = "authProviderId")]
    pub auth_provider_id: String,
    #[doc = "An optional flag indicating whether the token is a valid or expired (Default value: true)."]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
}
impl OAuthToken {
    pub fn new(farmer_id: String, auth_provider_id: String) -> Self {
        Self {
            farmer_id,
            auth_provider_id,
            is_valid: None,
            e_tag: None,
            created_date_time: None,
            modified_date_time: None,
        }
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuthTokenListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OAuthToken>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OAuthTokenListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OAuthTokenListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api Model for plant tissue analysis object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlantTissueAnalysis {
    #[doc = "Id of the associated Farmer."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Id of the associated Field."]
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[doc = "Id of the associated Crop."]
    #[serde(rename = "cropId", default, skip_serializing_if = "Option::is_none")]
    pub crop_id: Option<String>,
    #[doc = "Id of the associated Crop variety."]
    #[serde(rename = "cropVarietyId", default, skip_serializing_if = "Option::is_none")]
    pub crop_variety_id: Option<String>,
    #[doc = "Id of the associated Season."]
    #[serde(rename = "seasonId", default, skip_serializing_if = "Option::is_none")]
    pub season_id: Option<String>,
    #[doc = "Planting datetime for this plant tissue analysis."]
    #[serde(rename = "plantingDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub planting_date_time: Option<time::OffsetDateTime>,
    #[doc = "Growth stage for this plant tissue analysis."]
    #[serde(rename = "growthStage", default, skip_serializing_if = "Option::is_none")]
    pub growth_stage: Option<String>,
    #[doc = "Plant part for this plant tissue analysis."]
    #[serde(rename = "plantPart", default, skip_serializing_if = "Option::is_none")]
    pub plant_part: Option<String>,
    #[doc = "Plant position for this plant tissue analysis."]
    #[serde(rename = "plantPosition", default, skip_serializing_if = "Option::is_none")]
    pub plant_position: Option<String>,
    #[doc = "Plant appearance for this plant tissue analysis."]
    #[serde(rename = "plantAppearance", default, skip_serializing_if = "Option::is_none")]
    pub plant_appearance: Option<String>,
    #[doc = "Sample collection condition for this plant tissue analysis."]
    #[serde(rename = "sampleCollectionCondition", default, skip_serializing_if = "Option::is_none")]
    pub sample_collection_condition: Option<String>,
    #[doc = "Sample collection dateTime for this plant tissue analysis."]
    #[serde(rename = "sampleCollectionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub sample_collection_date_time: Option<time::OffsetDateTime>,
    #[doc = "Sample received dateTime."]
    #[serde(rename = "sampleReceivedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub sample_received_date_time: Option<time::OffsetDateTime>,
    #[doc = "Sample test result dateTime for this plant tissue analysis."]
    #[serde(rename = "sampleTestResultDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub sample_test_result_date_time: Option<time::OffsetDateTime>,
    #[doc = "Model for representing LabDetails object."]
    #[serde(rename = "labDetails", default, skip_serializing_if = "Option::is_none")]
    pub lab_details: Option<LabDetails>,
    #[doc = "Link for attachments."]
    #[serde(rename = "attachmentsLink", default, skip_serializing_if = "Option::is_none")]
    pub attachments_link: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl PlantTissueAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlantTissueAnalysisListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PlantTissueAnalysis>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PlantTissueAnalysisListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PlantTissueAnalysisListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of planting data resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlantingData {
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgPlantingRate", default, skip_serializing_if = "Option::is_none")]
    pub avg_planting_rate: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "totalMaterial", default, skip_serializing_if = "Option::is_none")]
    pub total_material: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgMaterial", default, skip_serializing_if = "Option::is_none")]
    pub avg_material: Option<Measure>,
    #[doc = "Planting product details."]
    #[serde(
        rename = "plantingProductDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub planting_product_details: Vec<PlantingProductDetail>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<Measure>,
    #[doc = "Modified date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ.\r\nNote: this will be specified by the source provider itself."]
    #[serde(rename = "operationModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Start date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "operationStartDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "operationEndDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Link for attachments."]
    #[serde(rename = "attachmentsLink", default, skip_serializing_if = "Option::is_none")]
    pub attachments_link: Option<String>,
    #[doc = "Optional boundary ID of the field for which operation was applied."]
    #[serde(rename = "associatedBoundaryId", default, skip_serializing_if = "Option::is_none")]
    pub associated_boundary_id: Option<String>,
    #[doc = "Farmer ID which belongs to the operation data."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl PlantingData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlantingDataListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PlantingData>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PlantingDataListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PlantingDataListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for Planting product detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlantingProductDetail {
    #[doc = "Name of the product."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "totalMaterial", default, skip_serializing_if = "Option::is_none")]
    pub total_material: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "avgMaterial", default, skip_serializing_if = "Option::is_none")]
    pub avg_material: Option<Measure>,
}
impl PlantingProductDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Point geometry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    #[serde(flatten)]
    pub geo_json_object: GeoJsonObject,
    #[serde(flatten)]
    pub point_coordinates: PointCoordinates,
}
impl Point {
    pub fn new(geo_json_object: GeoJsonObject, point_coordinates: PointCoordinates) -> Self {
        Self {
            geo_json_object,
            point_coordinates,
        }
    }
}
#[doc = "Schema of the coordinates of a point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PointCoordinates {
    #[doc = "Gets or sets the coordinate of this point.\r\nIt must be an array of 2 or 3 elements for a 2D or 3D system respectively."]
    pub coordinates: Vec<f64>,
}
impl PointCoordinates {
    pub fn new(coordinates: Vec<f64>) -> Self {
        Self { coordinates }
    }
}
#[doc = "Polygon geometry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    #[serde(flatten)]
    pub geo_json_object: GeoJsonObject,
    #[serde(flatten)]
    pub polygon_coordinates: PolygonCoordinates,
}
impl Polygon {
    pub fn new(geo_json_object: GeoJsonObject, polygon_coordinates: PolygonCoordinates) -> Self {
        Self {
            geo_json_object,
            polygon_coordinates,
        }
    }
}
#[doc = "Schema of polygon coordinates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolygonCoordinates {
    #[doc = "Gets or sets type of the GeoJSON Object.\r\nIt must be an array of linear ring coordinate arrays.\r\nFor Polygons with more than one of these rings, the first MUST be the exterior ring,\r\nand any others MUST be interior rings."]
    pub coordinates: Vec<Vec<Vec<f64>>>,
}
impl PolygonCoordinates {
    pub fn new(coordinates: Vec<Vec<Vec<f64>>>) -> Self {
        Self { coordinates }
    }
}
#[doc = "Schema for storing port values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Port {
    #[doc = "Name of the port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of port digital/analog."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Port {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api Model for Prescription object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Prescription {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Prescription map Id."]
    #[serde(rename = "prescriptionMapId", default, skip_serializing_if = "Option::is_none")]
    pub prescription_map_id: Option<String>,
    #[doc = "Product Code."]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "Product name."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Prescription type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Measures."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measures: Option<serde_json::Value>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Prescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrescriptionListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Prescription>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrescriptionListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrescriptionListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api Model for Prescription Map object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrescriptionMap {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Prescription map type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Season Id."]
    #[serde(rename = "seasonId", default, skip_serializing_if = "Option::is_none")]
    pub season_id: Option<String>,
    #[doc = "Crop Id."]
    #[serde(rename = "cropId", default, skip_serializing_if = "Option::is_none")]
    pub crop_id: Option<String>,
    #[doc = "Field Id."]
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl PrescriptionMap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrescriptionMapListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrescriptionMap>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrescriptionMapListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrescriptionMapListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model for representing ProductDetails object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductDetails {
    #[doc = "Rate of the product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[doc = "Instruction of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[doc = "Product of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}
impl ProductDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Model for SatelliteIngestionJobRequest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SatelliteData {
    #[doc = "List of ImageNames."]
    #[serde(
        rename = "imageNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_names: Vec<String>,
    #[doc = "List of ImageFormats. Available value: TIF."]
    #[serde(
        rename = "imageFormats",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_formats: Vec<String>,
    #[doc = "List of ImageResolutions in meters. Available values: 10, 20, 60."]
    #[serde(
        rename = "imageResolutions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_resolutions: Vec<f64>,
}
impl SatelliteData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of satellite data ingestion job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SatelliteDataIngestionJob {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "The id of the boundary object for which satellite data is being fetched."]
    #[serde(rename = "boundaryId")]
    pub boundary_id: String,
    #[doc = "Start Date."]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339")]
    pub start_date_time: time::OffsetDateTime,
    #[doc = "End Date."]
    #[serde(rename = "endDateTime", with = "azure_core::date::rfc3339")]
    pub end_date_time: time::OffsetDateTime,
    #[doc = "Provider of satellite data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<DataProvider>,
    #[doc = "Source of satellite data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[doc = "Data Model for SatelliteIngestionJobRequest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<SatelliteData>,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SatelliteDataIngestionJob {
    pub fn new(farmer_id: String, boundary_id: String, start_date_time: time::OffsetDateTime, end_date_time: time::OffsetDateTime) -> Self {
        Self {
            farmer_id,
            boundary_id,
            start_date_time,
            end_date_time,
            provider: None,
            source: None,
            data: None,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
#[doc = "Schema of scene resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scene {
    #[doc = "Date-time of the scene, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "sceneDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub scene_date_time: Option<time::OffsetDateTime>,
    #[doc = "Data provider of the scene."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Data source of the scene."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Collection of image files."]
    #[serde(
        rename = "imageFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_files: Vec<ImageFile>,
    #[doc = "Supported image formats for scene resource."]
    #[serde(rename = "imageFormat", default, skip_serializing_if = "Option::is_none")]
    pub image_format: Option<ImageFormat>,
    #[doc = "Cloud cover percentage of the scene."]
    #[serde(rename = "cloudCoverPercentage", default, skip_serializing_if = "Option::is_none")]
    pub cloud_cover_percentage: Option<f64>,
    #[doc = "Dark pixel percentage of the scene."]
    #[serde(rename = "darkPixelPercentage", default, skip_serializing_if = "Option::is_none")]
    pub dark_pixel_percentage: Option<f64>,
    #[doc = "Median of NDVI of the scene."]
    #[serde(rename = "ndviMedianValue", default, skip_serializing_if = "Option::is_none")]
    pub ndvi_median_value: Option<f64>,
    #[doc = "Boundary ID which belongs to the scene."]
    #[serde(rename = "boundaryId", default, skip_serializing_if = "Option::is_none")]
    pub boundary_id: Option<String>,
    #[doc = "Farmer ID which belongs to the scene."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Unique scene resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl Scene {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SceneListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Scene>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SceneListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SceneListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SearchAllBoundaries and SearchBoundaries parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchBoundaryQuery {
    #[doc = "Ids of the resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ids: Vec<String>,
    #[doc = "Names of the resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub names: Vec<String>,
    #[doc = "Filters on key-value pairs within the Properties object.\r\ne.g. \"{testKey} eq {testValue}\"."]
    #[serde(
        rename = "propertyFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_filters: Vec<String>,
    #[doc = "Statuses of the resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub statuses: Vec<String>,
    #[doc = "Minimum creation date of resource (inclusive)."]
    #[serde(rename = "minCreatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub min_created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Maximum creation date of resource (inclusive)."]
    #[serde(rename = "maxCreatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub max_created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Minimum last modified date of resource (inclusive)."]
    #[serde(rename = "minLastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub min_last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Maximum last modified date of resource (inclusive)."]
    #[serde(rename = "maxLastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub max_last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Maximum number of items needed (inclusive).\r\nMinimum = 10, Maximum = 1000, Default value = 50."]
    #[serde(rename = "$maxPageSize", default, skip_serializing_if = "Option::is_none")]
    pub max_page_size: Option<i32>,
    #[doc = "Skip token for getting next set of results."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Is the boundary primary."]
    #[serde(rename = "isPrimary", default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[doc = "Type of the parent it belongs to."]
    #[serde(rename = "parentType", default, skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<search_boundary_query::ParentType>,
    #[doc = "Parent Ids of the resource."]
    #[serde(
        rename = "parentIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parent_ids: Vec<String>,
    #[doc = "Minimum acreage of the boundary (inclusive)."]
    #[serde(rename = "minAcreage", default, skip_serializing_if = "Option::is_none")]
    pub min_acreage: Option<f64>,
    #[doc = "Maximum acreage of the boundary (inclusive)."]
    #[serde(rename = "maxAcreage", default, skip_serializing_if = "Option::is_none")]
    pub max_acreage: Option<f64>,
    #[doc = "GeoJSON (For more details: https://geojson.org/). Note: Coordinates are expected in [Longitude, Latitude] format."]
    #[serde(rename = "intersectsWithGeometry", default, skip_serializing_if = "Option::is_none")]
    pub intersects_with_geometry: Option<GeoJsonObject>,
}
impl SearchBoundaryQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod search_boundary_query {
    use super::*;
    #[doc = "Type of the parent it belongs to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ParentType")]
    pub enum ParentType {
        Field,
        SeasonalField,
        Zone,
        Prescription,
        PlantTissueAnalysis,
        ApplicationData,
        PlantingData,
        TillageData,
        HarvestData,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ParentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ParentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ParentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Field => serializer.serialize_unit_variant("ParentType", 0u32, "Field"),
                Self::SeasonalField => serializer.serialize_unit_variant("ParentType", 1u32, "SeasonalField"),
                Self::Zone => serializer.serialize_unit_variant("ParentType", 2u32, "Zone"),
                Self::Prescription => serializer.serialize_unit_variant("ParentType", 3u32, "Prescription"),
                Self::PlantTissueAnalysis => serializer.serialize_unit_variant("ParentType", 4u32, "PlantTissueAnalysis"),
                Self::ApplicationData => serializer.serialize_unit_variant("ParentType", 5u32, "ApplicationData"),
                Self::PlantingData => serializer.serialize_unit_variant("ParentType", 6u32, "PlantingData"),
                Self::TillageData => serializer.serialize_unit_variant("ParentType", 7u32, "TillageData"),
                Self::HarvestData => serializer.serialize_unit_variant("ParentType", 8u32, "HarvestData"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema of season resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Season {
    #[doc = "Season start datetime, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Season end datetime, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Season year."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Season {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SeasonListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Season>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SeasonListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SeasonListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of seasonal field resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SeasonalField {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Primary boundary id."]
    #[serde(rename = "primaryBoundaryId", default, skip_serializing_if = "Option::is_none")]
    pub primary_boundary_id: Option<String>,
    #[doc = "Boundary Ids."]
    #[serde(
        rename = "boundaryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub boundary_ids: Vec<String>,
    #[doc = "Id of the associated Farm."]
    #[serde(rename = "farmId", default, skip_serializing_if = "Option::is_none")]
    pub farm_id: Option<String>,
    #[doc = "Id of the associated Field."]
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[doc = "Id of the season it belongs to."]
    #[serde(rename = "seasonId", default, skip_serializing_if = "Option::is_none")]
    pub season_id: Option<String>,
    #[doc = "CropVariety ids."]
    #[serde(
        rename = "cropVarietyIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub crop_variety_ids: Vec<String>,
    #[doc = "Id of the crop it belongs to."]
    #[serde(rename = "cropId", default, skip_serializing_if = "Option::is_none")]
    pub crop_id: Option<String>,
    #[doc = "Average yield value of the seasonal field."]
    #[serde(rename = "avgYieldValue", default, skip_serializing_if = "Option::is_none")]
    pub avg_yield_value: Option<f64>,
    #[doc = "Unit of the average yield value attribute."]
    #[serde(rename = "avgYieldUnit", default, skip_serializing_if = "Option::is_none")]
    pub avg_yield_unit: Option<String>,
    #[doc = "Average seed population value of the seasonal field."]
    #[serde(rename = "avgSeedPopulationValue", default, skip_serializing_if = "Option::is_none")]
    pub avg_seed_population_value: Option<f64>,
    #[doc = "Unit of average seed population value attribute."]
    #[serde(rename = "avgSeedPopulationUnit", default, skip_serializing_if = "Option::is_none")]
    pub avg_seed_population_unit: Option<String>,
    #[doc = "Planting datetime, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "plantingDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub planting_date_time: Option<time::OffsetDateTime>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SeasonalField {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SeasonalFieldListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SeasonalField>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SeasonalFieldListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SeasonalFieldListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sensor API model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sensor {
    #[doc = "Id of the associated sensor data model."]
    #[serde(rename = "sensorDataModelId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_data_model_id: Option<String>,
    #[doc = "Integration id for the device."]
    #[serde(rename = "integrationId", default, skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    #[doc = "Id of the associated hardware."]
    #[serde(rename = "hardwareId", default, skip_serializing_if = "Option::is_none")]
    pub hardware_id: Option<String>,
    #[doc = "Id of the associated device."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Type of sensor."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Location model class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[doc = "Schema for storing port values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<Port>,
    #[doc = "Depth of each sensor measure in meters.\r\nLike sensor moisture at 2m, 4m, 6m."]
    #[serde(
        rename = "depthInMeters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub depth_in_meters: Vec<f64>,
    #[doc = "Id of the associated sensor partner."]
    #[serde(rename = "sensorPartnerId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_partner_id: Option<String>,
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and\r\nonly string, numeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Sensor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SensorModel API model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorDataModel {
    #[doc = "Type of sensor."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Sensor manufacturer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[doc = "Sensor productCode."]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "Map of sensor type to sensor measures."]
    pub measures: serde_json::Value,
    #[doc = "Id of the associated sensor partner."]
    #[serde(rename = "sensorPartnerId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_partner_id: Option<String>,
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and\r\nonly string, numeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SensorDataModel {
    pub fn new(measures: serde_json::Value) -> Self {
        Self {
            type_: None,
            manufacturer: None,
            product_code: None,
            measures,
            sensor_partner_id: None,
            id: None,
            status: None,
            created_date_time: None,
            modified_date_time: None,
            e_tag: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorDataModelListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SensorDataModel>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SensorDataModelListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SensorDataModelListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sensor model measure details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorDataModelMeasure {
    #[doc = "Description of sensor measure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Sensor measure data type."]
    #[serde(rename = "dataType")]
    pub data_type: sensor_data_model_measure::DataType,
    #[doc = "Measurement type of sensor data."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Unit of sensor measure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "A collection of key value pairs for sensor data model.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a model and\r\nonly string, numeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SensorDataModelMeasure {
    pub fn new(data_type: sensor_data_model_measure::DataType) -> Self {
        Self {
            description: None,
            data_type,
            type_: None,
            unit: None,
            properties: None,
        }
    }
}
pub mod sensor_data_model_measure {
    use super::*;
    #[doc = "Sensor measure data type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataType")]
    pub enum DataType {
        Bool,
        Double,
        DateTime,
        Long,
        String,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Bool => serializer.serialize_unit_variant("DataType", 0u32, "Bool"),
                Self::Double => serializer.serialize_unit_variant("DataType", 1u32, "Double"),
                Self::DateTime => serializer.serialize_unit_variant("DataType", 2u32, "DateTime"),
                Self::Long => serializer.serialize_unit_variant("DataType", 3u32, "Long"),
                Self::String => serializer.serialize_unit_variant("DataType", 4u32, "String"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Sensor event response model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorEvent {
    #[doc = "Id of the sensor."]
    #[serde(rename = "sensorId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_id: Option<String>,
    #[doc = "Id of the sensor partner."]
    #[serde(rename = "sensorPartnerId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_partner_id: Option<String>,
    #[doc = "Id of the associated farmer."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Id of the associated boundary."]
    #[serde(rename = "boundaryId", default, skip_serializing_if = "Option::is_none")]
    pub boundary_id: Option<String>,
    #[doc = "DateTime of sensor event observation."]
    #[serde(rename = "eventDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub event_date_time: Option<time::OffsetDateTime>,
    #[doc = "DateTime of sensor event ingestion to data store."]
    #[serde(rename = "ingestionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub ingestion_date_time: Option<time::OffsetDateTime>,
    #[doc = "Sensor measures."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measures: Option<serde_json::Value>,
}
impl SensorEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorEventListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SensorEvent>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SensorEventListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Sensor>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SensorListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SensorListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SensorMapping API model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorMapping {
    #[doc = "Id of the associated sensor."]
    #[serde(rename = "sensorId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_id: Option<String>,
    #[doc = "Id of the associated sensor partner."]
    #[serde(rename = "sensorPartnerId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_partner_id: Option<String>,
    #[doc = "Id of the associated farmer."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Id of the associated boundary."]
    #[serde(rename = "boundaryId", default, skip_serializing_if = "Option::is_none")]
    pub boundary_id: Option<String>,
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and\r\nonly string, numeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SensorMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorMappingListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SensorMapping>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SensorMappingListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SensorMappingListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sensor partner integration check consent response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorPartnerIntegrationCheckConsentResponse {
    #[doc = "Flag to determine the status of partner integration consent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consented: Option<bool>,
    #[doc = "Id of the associated sensor partner."]
    #[serde(rename = "sensorPartnerId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_partner_id: Option<String>,
    #[doc = "Id of the integration."]
    #[serde(rename = "integrationId", default, skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
}
impl SensorPartnerIntegrationCheckConsentResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sensor partner integration generate consent link response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorPartnerIntegrationGenerateConsentLinkResponse {
    #[doc = "Consent link."]
    #[serde(rename = "consentLink", default, skip_serializing_if = "Option::is_none")]
    pub consent_link: Option<String>,
    #[doc = "Consent expiry date time, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "consentExpiryDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub consent_expiry_date_time: Option<time::OffsetDateTime>,
}
impl SensorPartnerIntegrationGenerateConsentLinkResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sensor partner integration model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorPartnerIntegrationModel {
    #[doc = "Id of the integration."]
    #[serde(rename = "integrationId", default, skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    #[doc = "Id of the farmer."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Id of the associated sensor partner."]
    #[serde(rename = "sensorPartnerId", default, skip_serializing_if = "Option::is_none")]
    pub sensor_partner_id: Option<String>,
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and\r\nonly string, numeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SensorPartnerIntegrationModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensorPartnerIntegrationModelListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SensorPartnerIntegrationModel>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SensorPartnerIntegrationModelListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SensorPartnerIntegrationModelListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of sensor placement model job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorPlacementModelJob {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "The id of the boundary object for which sensor placement is being calculated."]
    #[serde(rename = "boundaryId")]
    pub boundary_id: String,
    #[doc = "The version of the sensor placement model to be run."]
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    #[doc = "Start datetime for satellite data to be pulled."]
    #[serde(rename = "inferenceStartDateTime", with = "azure_core::date::rfc3339")]
    pub inference_start_date_time: time::OffsetDateTime,
    #[doc = "End datetime for satellite data to be pulled."]
    #[serde(rename = "inferenceEndDateTime", with = "azure_core::date::rfc3339")]
    pub inference_end_date_time: time::OffsetDateTime,
    #[doc = "Provider of satellite data. Available Value: Microsoft."]
    #[serde(rename = "satelliteProvider")]
    pub satellite_provider: sensor_placement_model_job::SatelliteProvider,
    #[doc = "Source of satellite data. Available Value: Sentinel_2_L2A."]
    #[serde(rename = "satelliteSource")]
    pub satellite_source: sensor_placement_model_job::SatelliteSource,
    #[doc = "SensorType. The sensor placement map generated for sensor type (e.g., soil moisture, soil temperature, npk). Available Value: SoilMoisture."]
    #[serde(rename = "sensorType")]
    pub sensor_type: String,
    #[doc = "IsRanked, if True the sensor placements will be ranked."]
    #[serde(rename = "isRanked")]
    pub is_ranked: bool,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SensorPlacementModelJob {
    pub fn new(
        farmer_id: String,
        boundary_id: String,
        model_version: String,
        inference_start_date_time: time::OffsetDateTime,
        inference_end_date_time: time::OffsetDateTime,
        satellite_provider: sensor_placement_model_job::SatelliteProvider,
        satellite_source: sensor_placement_model_job::SatelliteSource,
        sensor_type: String,
        is_ranked: bool,
    ) -> Self {
        Self {
            farmer_id,
            boundary_id,
            model_version,
            inference_start_date_time,
            inference_end_date_time,
            satellite_provider,
            satellite_source,
            sensor_type,
            is_ranked,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
pub mod sensor_placement_model_job {
    use super::*;
    #[doc = "Provider of satellite data. Available Value: Microsoft."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SatelliteProvider")]
    pub enum SatelliteProvider {
        Microsoft,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SatelliteProvider {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SatelliteProvider {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SatelliteProvider {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Microsoft => serializer.serialize_unit_variant("SatelliteProvider", 0u32, "Microsoft"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Source of satellite data. Available Value: Sentinel_2_L2A."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SatelliteSource")]
    pub enum SatelliteSource {
        #[serde(rename = "Sentinel_2_L2A")]
        Sentinel2L2a,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SatelliteSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SatelliteSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SatelliteSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sentinel2L2a => serializer.serialize_unit_variant("SatelliteSource", 0u32, "Sentinel_2_L2A"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Model for renewing sensor's connection string."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorRenewConnectionStringModel {
    #[doc = "Specifies the type of connection string key to be renewed valid values - Primary/Secondary/Both."]
    #[serde(rename = "connectionStringType")]
    pub connection_string_type: sensor_renew_connection_string_model::ConnectionStringType,
}
impl SensorRenewConnectionStringModel {
    pub fn new(connection_string_type: sensor_renew_connection_string_model::ConnectionStringType) -> Self {
        Self { connection_string_type }
    }
}
pub mod sensor_renew_connection_string_model {
    use super::*;
    #[doc = "Specifies the type of connection string key to be renewed valid values - Primary/Secondary/Both."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionStringType")]
    pub enum ConnectionStringType {
        Primary,
        Secondary,
        Both,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionStringType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionStringType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionStringType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("ConnectionStringType", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("ConnectionStringType", 1u32, "Secondary"),
                Self::Both => serializer.serialize_unit_variant("ConnectionStringType", 2u32, "Both"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema of soil moisture model job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoilMoistureModelJob {
    #[doc = "Farmer Id."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "The id of the boundary object for which soil moisture is being calculated."]
    #[serde(rename = "boundaryId")]
    pub boundary_id: String,
    #[doc = "Sensor data model Id."]
    #[serde(rename = "sensorDataModelId")]
    pub sensor_data_model_id: String,
    #[doc = "Sensor partner Id."]
    #[serde(rename = "sensorPartnerId")]
    pub sensor_partner_id: String,
    #[doc = "Inference start date time for soil moisture calculations."]
    #[serde(rename = "inferenceStartDateTime", with = "azure_core::date::rfc3339")]
    pub inference_start_date_time: time::OffsetDateTime,
    #[doc = "Inference end date time for soil moisture calculations."]
    #[serde(rename = "inferenceEndDateTime", with = "azure_core::date::rfc3339")]
    pub inference_end_date_time: time::OffsetDateTime,
    #[doc = "Provider of satellite data. Available Value: Microsoft."]
    #[serde(rename = "satelliteProvider")]
    pub satellite_provider: soil_moisture_model_job::SatelliteProvider,
    #[doc = "Source of satellite data. Available Value: Sentinel_2_L2A."]
    #[serde(rename = "satelliteSource")]
    pub satellite_source: soil_moisture_model_job::SatelliteSource,
    #[doc = "ImageResolution in meters. Available values: 10, 20, 60."]
    #[serde(rename = "imageResolution")]
    pub image_resolution: f64,
    #[doc = "ImageFormat. Available value: TIF."]
    #[serde(rename = "imageFormat")]
    pub image_format: soil_moisture_model_job::ImageFormat,
    #[doc = "The version of the soil moisture model to be run."]
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    #[doc = "Schema for storing sensor definition keywords."]
    #[serde(rename = "sensorDefinition")]
    pub sensor_definition: SoilMoistureModelSensorDefinition,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SoilMoistureModelJob {
    pub fn new(
        farmer_id: String,
        boundary_id: String,
        sensor_data_model_id: String,
        sensor_partner_id: String,
        inference_start_date_time: time::OffsetDateTime,
        inference_end_date_time: time::OffsetDateTime,
        satellite_provider: soil_moisture_model_job::SatelliteProvider,
        satellite_source: soil_moisture_model_job::SatelliteSource,
        image_resolution: f64,
        image_format: soil_moisture_model_job::ImageFormat,
        model_version: String,
        sensor_definition: SoilMoistureModelSensorDefinition,
    ) -> Self {
        Self {
            farmer_id,
            boundary_id,
            sensor_data_model_id,
            sensor_partner_id,
            inference_start_date_time,
            inference_end_date_time,
            satellite_provider,
            satellite_source,
            image_resolution,
            image_format,
            model_version,
            sensor_definition,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
pub mod soil_moisture_model_job {
    use super::*;
    #[doc = "Provider of satellite data. Available Value: Microsoft."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SatelliteProvider")]
    pub enum SatelliteProvider {
        Microsoft,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SatelliteProvider {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SatelliteProvider {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SatelliteProvider {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Microsoft => serializer.serialize_unit_variant("SatelliteProvider", 0u32, "Microsoft"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Source of satellite data. Available Value: Sentinel_2_L2A."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SatelliteSource")]
    pub enum SatelliteSource {
        #[serde(rename = "Sentinel_2_L2A")]
        Sentinel2L2a,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SatelliteSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SatelliteSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SatelliteSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sentinel2L2a => serializer.serialize_unit_variant("SatelliteSource", 0u32, "Sentinel_2_L2A"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "ImageFormat. Available value: TIF."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ImageFormat")]
    pub enum ImageFormat {
        #[serde(rename = "TIF")]
        Tif,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ImageFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ImageFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ImageFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tif => serializer.serialize_unit_variant("ImageFormat", 0u32, "TIF"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema for storing sensor definition keywords."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoilMoistureModelSensorDefinition {
    #[doc = "The measurement name for sensor measure in sensorDataModel."]
    #[serde(rename = "sensorMeasurement")]
    pub sensor_measurement: String,
    #[doc = "The measurement name for minimum measurement value."]
    #[serde(rename = "minProperty")]
    pub min_property: String,
    #[doc = "The measurement name for maximum measurement value."]
    #[serde(rename = "maxProperty")]
    pub max_property: String,
}
impl SoilMoistureModelSensorDefinition {
    pub fn new(sensor_measurement: String, min_property: String, max_property: String) -> Self {
        Self {
            sensor_measurement,
            min_property,
            max_property,
        }
    }
}
#[doc = "SolutionInference request model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionInference {
    #[doc = "RequestPath containing the api-version, query parameters and path route to be called for partner request.\r\nExpected format is \"/{api-version}/{resourceExposedByPartner}/{customerDefinedJobId}?query1=value1\".\r\nNot following this format may result into validation errors."]
    #[serde(rename = "requestPath")]
    pub request_path: String,
    #[doc = "Api input parameters required by partner to trigger/cancel job request."]
    #[serde(rename = "partnerRequestBody", default, skip_serializing_if = "Option::is_none")]
    pub partner_request_body: Option<serde_json::Value>,
}
impl SolutionInference {
    pub fn new(request_path: String) -> Self {
        Self {
            request_path,
            partner_request_body: None,
        }
    }
}
#[doc = "Source of satellite data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Source")]
pub enum Source {
    #[serde(rename = "Sentinel_2_L2A")]
    Sentinel2L2a,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Source {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sentinel2L2a => serializer.serialize_unit_variant("Source", 0u32, "Sentinel_2_L2A"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of tillage data resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TillageData {
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "tillageDepth", default, skip_serializing_if = "Option::is_none")]
    pub tillage_depth: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "tillagePressure", default, skip_serializing_if = "Option::is_none")]
    pub tillage_pressure: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<Measure>,
    #[doc = "Modified date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ.\r\nNote: this will be specified by the source provider itself."]
    #[serde(rename = "operationModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Start date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "operationStartDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End date-time of the operation data, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "operationEndDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Link for attachments."]
    #[serde(rename = "attachmentsLink", default, skip_serializing_if = "Option::is_none")]
    pub attachments_link: Option<String>,
    #[doc = "Optional boundary ID of the field for which operation was applied."]
    #[serde(rename = "associatedBoundaryId", default, skip_serializing_if = "Option::is_none")]
    pub associated_boundary_id: Option<String>,
    #[doc = "Farmer ID which belongs to the operation data."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TillageData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TillageDataListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TillageData>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TillageDataListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TillageDataListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of weather data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeatherData {
    #[doc = "Farmer ID."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "Boundary ID."]
    #[serde(rename = "boundaryId")]
    pub boundary_id: String,
    #[doc = "ID of the weather extension."]
    #[serde(rename = "extensionId")]
    pub extension_id: String,
    #[doc = "Location model class."]
    pub location: Location,
    #[doc = "Date-time of the weather data, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "dateTime", with = "azure_core::date::rfc3339")]
    pub date_time: time::OffsetDateTime,
    #[doc = "Unit System like US/SI etc."]
    #[serde(rename = "unitSystemCode", default, skip_serializing_if = "Option::is_none")]
    pub unit_system_code: Option<String>,
    #[doc = "Version of the weather data extension."]
    #[serde(rename = "extensionVersion")]
    pub extension_version: String,
    #[doc = "Type of weather data (forecast/historical)."]
    #[serde(rename = "weatherDataType")]
    pub weather_data_type: String,
    #[doc = "Granularity of weather data (daily/hourly)."]
    pub granularity: String,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "cloudCover", default, skip_serializing_if = "Option::is_none")]
    pub cloud_cover: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "dewPoint", default, skip_serializing_if = "Option::is_none")]
    pub dew_point: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "growingDegreeDay", default, skip_serializing_if = "Option::is_none")]
    pub growing_degree_day: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precipitation: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pressure: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "relativeHumidity", default, skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "soilMoisture", default, skip_serializing_if = "Option::is_none")]
    pub soil_moisture: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "soilTemperature", default, skip_serializing_if = "Option::is_none")]
    pub soil_temperature: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "wetBulbTemperature", default, skip_serializing_if = "Option::is_none")]
    pub wet_bulb_temperature: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "windChill", default, skip_serializing_if = "Option::is_none")]
    pub wind_chill: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "windDirection", default, skip_serializing_if = "Option::is_none")]
    pub wind_direction: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "windGust", default, skip_serializing_if = "Option::is_none")]
    pub wind_gust: Option<Measure>,
    #[doc = "Schema for storing measurement reading and unit."]
    #[serde(rename = "windSpeed", default, skip_serializing_if = "Option::is_none")]
    pub wind_speed: Option<Measure>,
    #[doc = "Weather data ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 250 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string and numeral values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl WeatherData {
    pub fn new(
        farmer_id: String,
        boundary_id: String,
        extension_id: String,
        location: Location,
        date_time: time::OffsetDateTime,
        extension_version: String,
        weather_data_type: String,
        granularity: String,
    ) -> Self {
        Self {
            farmer_id,
            boundary_id,
            extension_id,
            location,
            date_time,
            unit_system_code: None,
            extension_version,
            weather_data_type,
            granularity,
            cloud_cover: None,
            dew_point: None,
            growing_degree_day: None,
            precipitation: None,
            pressure: None,
            relative_humidity: None,
            soil_moisture: None,
            soil_temperature: None,
            temperature: None,
            visibility: None,
            wet_bulb_temperature: None,
            wind_chill: None,
            wind_direction: None,
            wind_gust: None,
            wind_speed: None,
            id: None,
            e_tag: None,
            created_date_time: None,
            modified_date_time: None,
            properties: None,
        }
    }
}
#[doc = "Schema of weather data delete job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeatherDataDeleteJob {
    #[doc = "Id of the extension to be used for the providerInput. eg. DTN.ClearAg."]
    #[serde(rename = "extensionId")]
    pub extension_id: String,
    #[doc = "The id of the farmer object for which weather data is being fetched."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "The id of the boundary object for which weather data is being fetched."]
    #[serde(rename = "boundaryId")]
    pub boundary_id: String,
    #[doc = "Type of weather data. Possible values include: 'forecast' , 'historical'."]
    #[serde(rename = "weatherDataType", default, skip_serializing_if = "Option::is_none")]
    pub weather_data_type: Option<String>,
    #[doc = "Granularity of weather data. Possible values include: 'daily' , 'hourly'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[doc = "Weather data start UTC date-time (inclusive), sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Weather data end UTC date-time (inclusive), sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl WeatherDataDeleteJob {
    pub fn new(extension_id: String, farmer_id: String, boundary_id: String) -> Self {
        Self {
            extension_id,
            farmer_id,
            boundary_id,
            weather_data_type: None,
            granularity: None,
            start_date_time: None,
            end_date_time: None,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
#[doc = "Schema of weather ingestion job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeatherDataIngestionJob {
    #[doc = "The id of the boundary object for which weather data is being fetched."]
    #[serde(rename = "boundaryId")]
    pub boundary_id: String,
    #[doc = "The id of the farmer object for which weather data is being fetched."]
    #[serde(rename = "farmerId")]
    pub farmer_id: String,
    #[doc = "Id of the extension to be used for the providerInput. eg. DTN.ClearAg."]
    #[serde(rename = "extensionId")]
    pub extension_id: String,
    #[doc = "Extension api name to which request is to be made."]
    #[serde(rename = "extensionApiName")]
    pub extension_api_name: String,
    #[doc = "Extension api input dictionary which would be used to feed request query/body/parameter information."]
    #[serde(rename = "extensionApiInput")]
    pub extension_api_input: serde_json::Value,
    #[doc = "App id of the weather data provider."]
    #[serde(rename = "extensionDataProviderAppId", default, skip_serializing_if = "Option::is_none")]
    pub extension_data_provider_app_id: Option<String>,
    #[doc = "Api key of the weather data provider."]
    #[serde(rename = "extensionDataProviderApiKey", default, skip_serializing_if = "Option::is_none")]
    pub extension_data_provider_api_key: Option<String>,
    #[doc = "Unique job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the job.\r\nPossible values: 'Waiting', 'Running', 'Succeeded', 'Failed', 'Cancelled'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Duration of the job in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Status message to capture more details of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Job created at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job was last acted upon at dateTime. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "Job start time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job end time when available. Sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl WeatherDataIngestionJob {
    pub fn new(
        boundary_id: String,
        farmer_id: String,
        extension_id: String,
        extension_api_name: String,
        extension_api_input: serde_json::Value,
    ) -> Self {
        Self {
            boundary_id,
            farmer_id,
            extension_id,
            extension_api_name,
            extension_api_input,
            extension_data_provider_app_id: None,
            extension_data_provider_api_key: None,
            id: None,
            status: None,
            duration_in_seconds: None,
            message: None,
            created_date_time: None,
            last_action_date_time: None,
            start_time: None,
            end_time: None,
            name: None,
            description: None,
            properties: None,
        }
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WeatherDataListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WeatherData>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WeatherDataListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WeatherDataListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api Model for Zone object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Zone {
    #[doc = "Farmer Id associated with the Zone."]
    #[serde(rename = "farmerId", default, skip_serializing_if = "Option::is_none")]
    pub farmer_id: Option<String>,
    #[doc = "Type of the Zone."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Management Zone Id associated with the Zone."]
    #[serde(rename = "managementZoneId", default, skip_serializing_if = "Option::is_none")]
    pub management_zone_id: Option<String>,
    #[doc = "Unique resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ETag value to implement optimistic concurrency."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Date-time when resource was created, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date-time when resource was last modified, sample format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "modifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Source of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Name to identify resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Textual description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A collection of key value pairs that belongs to the resource.\r\nEach pair must not have a key greater than 50 characters\r\nand must not have a value greater than 150 characters.\r\nNote: A maximum of 25 key value pairs can be provided for a resource and only string,\r\nnumeral and datetime (yyyy-MM-ddTHH:mm:ssZ) values are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Zone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response contains list of requested objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ZoneListResponse {
    #[doc = "List of requested objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Zone>,
    #[doc = "Token used in retrieving the next page. If null, there are no additional pages."]
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ZoneListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ZoneListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
