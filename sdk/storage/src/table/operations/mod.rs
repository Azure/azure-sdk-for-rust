//pub mod create_table;
//pub mod delete_table;
pub mod list_tables;

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
