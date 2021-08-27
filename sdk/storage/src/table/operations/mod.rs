use azure_core::HTTPHeaderError;
use http::HeaderValue;
pub mod create_table;
pub mod delete_table;
pub mod list_tables;

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

pub fn header_value<T: AsRef<str>>(value: &Option<T>) -> Result<HeaderValue, HTTPHeaderError> {
    HeaderValue::from_str(
        value
            .as_ref()
            .ok_or(azure_core::HTTPHeaderError::HeaderValidationError(
                "header value was none".into(),
            ))?
            .as_ref(),
    )
    .map_err(|e| azure_core::HTTPHeaderError::InvalidHeaderValue(e))
}
