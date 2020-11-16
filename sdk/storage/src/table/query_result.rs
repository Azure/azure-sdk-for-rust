use crate::{ContinuationToken, TableEntity};
use azure_core::errors::AzureError;
use http::HeaderMap;
use hyper::body;
use serde::{de::DeserializeOwned, Serialize};

pub struct QueryResult<T>
where
    T: DeserializeOwned,
{
    pub entities: Vec<TableEntity<T>>,
    pub continuation_token: Option<ContinuationToken>,
}

impl<T> std::convert::TryFrom<(&str, &HeaderMap, &body::Bytes)> for QueryResult<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;

    fn try_from((query_path, headers, body): (&str, &HeaderMap, &body::Bytes)) -> Result<Self, Self::Error> {

        log::debug!("body == {}", std::str::from_utf8(body)?);

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
