use azure_sdk_core::errors::AzureError;
use http::header;
use http::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableEntry<T> {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,

    // etag is not serialized, it is parsed from the header
    #[serde(skip_serializing)]
    pub etag: Option<String>,
    
    #[serde(flatten)]
    pub payload: T,
}

impl<T> TableEntry<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(
        row_key: String,
        partition_key: String,
        etag: Option<String>,
        payload: T,
    ) -> TableEntry<T> {
        TableEntry {
            row_key,
            partition_key,
            etag,
            payload,
        }
    }
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for TableEntry<T>
where
    T: Serialize + DeserializeOwned,
{
    type Error = AzureError;

    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;
        debug!("headers == {:?}", headers);
        debug!("body == {:?}", body);

        let mut ret: TableEntry<T> = serde_json::from_slice(&body)?;

        // inject etag if present
        ret.etag = match headers.get(header::ETAG) {
            Some(etag) => Some(etag.to_str()?.to_owned()),
            None => None,
        };

        Ok(ret)
    }
}
