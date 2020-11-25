use crate::de;
use azure_core::errors::AzureError;
use chrono::{DateTime, Utc};
use http::header;
use http::HeaderMap;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoData {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableEntity<T> {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,

    #[serde(skip_serializing, rename = "odata.etag")]
    pub etag: Option<String>,

    #[serde(
        skip_serializing,
        deserialize_with = "de::optional_timestamp",
        rename = "Timestamp"
    )]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(flatten)]
    pub payload: T,
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for TableEntity<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;

    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;
        log::trace!("headers == {:?}", headers);
        log::trace!("body == {:?}", std::str::from_utf8(body));

        let mut entity: Self = serde_json::from_slice(&body)?;

        if let Some(etag) = headers.get(header::ETAG) {
            entity.etag = Some(etag.to_str()?.to_owned());
        }

        Ok(entity)
    }
}
