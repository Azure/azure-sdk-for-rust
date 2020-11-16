use crate::{ContinuationToken, TableEntity};
use azure_core::errors::AzureError;
use http::HeaderMap;
use hyper::body;
use serde::{de::DeserializeOwned, Serialize};

pub struct QueryResult<T>
where
    T: DeserializeOwned + Serialize,
{
    pub entities: Vec<TableEntity<T>>,
    pub(crate) continuation_token: Option<ContinuationToken>,
}

impl<T> std::convert::TryFrom<(&str, &HeaderMap, &body::Bytes)> for QueryResult<T>
where
    T: DeserializeOwned + Serialize,
{
    type Error = AzureError;

    fn try_from(val: (&str, &HeaderMap, &body::Bytes)) -> Result<Self, Self::Error> {
        let query_path = val.0;
        let headers = val.1;
        let body = val.2;

        Ok(Self {
            entities: serde_json::from_slice::<EntityCollection<T>>(body)?.value,
            continuation_token: ContinuationToken::parse_from_headers_optional(
                query_path, headers,
            )?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EntityCollection<T> {
    value: Vec<TableEntity<T>>,
}
