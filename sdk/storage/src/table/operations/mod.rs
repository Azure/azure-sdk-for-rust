use azure_core::{Error, HTTPHeaderError, Request};
use chrono::{DateTime, Utc};
use http::HeaderValue;
pub mod Insert_entity;
pub mod create_table;
pub mod delete_table;
pub mod get_entity;
pub mod list_tables;
pub mod update_entity;

/// api version enum
#[derive(Debug, Clone)]
pub enum ApiVersion {
    Version2019_12_12,
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self::Version2019_12_12
    }
}

impl AsRef<str> for ApiVersion {
    fn as_ref(&self) -> &str {
        match self {
            ApiVersion::Version2019_12_12 => "2019-12-12",
        }
    }
}

/// open data protocol response details level
#[derive(Debug, Clone)]
pub enum OdataMetadataLevel {
    NoMetadata,
    MinimalMetadata,
    FullMetadata,
}

impl AsRef<str> for OdataMetadataLevel {
    fn as_ref(&self) -> &str {
        match self {
            OdataMetadataLevel::NoMetadata => "application/json;odata=nometadata",
            OdataMetadataLevel::MinimalMetadata => "application/json;odata=minimalmetadata",
            OdataMetadataLevel::FullMetadata => "application/json;odata=fullmetadata",
        }
    }
}

/// Sets if the resource should be included in the response.
#[derive(Debug, Clone)]
pub enum EchoContent {
    ReturnNoContent,
    ReturnContent,
}

impl AsRef<str> for EchoContent {
    fn as_ref(&self) -> &str {
        match self {
            EchoContent::ReturnNoContent => "return-no-content",
            EchoContent::ReturnContent => "return-content",
        }
    }
}

///The client may specify the ETag for the entity on the request in order to compare to the ETag maintained by the service
/// for the purpose of optimistic concurrency.
enum ETag {
    ForceUpdate,
    OnlyIfMatch(String),
}

impl Default for ETag {
    fn default() -> Self {
        Self::ForceUpdate
    }
}

impl AsRef<str> for ETag {
    fn as_ref(&self) -> &str {
        match self {
            ETag::ForceUpdate => "*",
            ETag::OnlyIfMatch(etag) => etag.as_str(),
        }
    }
}

/// This trait represents a table entity.
/// User should implement this trait for thir custom models
pub trait TableEntity<'a> {
    type Entity: serde::Serialize + 'a;

    /// Return partition key value as reference.
    fn partition_key(&self) -> &str;

    /// Return partition key value as reference.
    fn row_key(&self) -> &str;
}

pub fn header_time_value(utc_time: DateTime<Utc>) -> Result<HeaderValue, HTTPHeaderError> {
    const FMT: &str = "%a, %d %h %Y %T GMT";
    HeaderValue::from_str(utc_time.format(FMT).to_string().as_str())
        .map_err(azure_core::HTTPHeaderError::InvalidHeaderValue)
}

pub fn header_value<T: AsRef<str>>(value: &Option<T>) -> Result<HeaderValue, HTTPHeaderError> {
    HeaderValue::from_str(
        value
            .as_ref()
            .ok_or_else(|| {
                azure_core::HTTPHeaderError::HeaderValidationError("header value was none".into())
            })?
            .as_ref(),
    )
    .map_err(azure_core::HTTPHeaderError::InvalidHeaderValue)
}
