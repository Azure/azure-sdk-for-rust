use crate::{ContinuationToken, TableEntity};
use azure_core::errors::AzureError;
use http::HeaderMap;
use hyper::body;
use serde::{de::DeserializeOwned, Serialize};

pub struct PaginatedResponse<T>
where
    T: DeserializeOwned,
{
    pub entities: Vec<TableEntity<T>>,
    pub continuation_token: Option<ContinuationToken>,
}

impl<T> std::convert::TryFrom<(url::Url, &HeaderMap, &body::Bytes)> for PaginatedResponse<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;

    fn try_from(
        (url, headers, body): (url::Url, &HeaderMap, &body::Bytes),
    ) -> Result<Self, Self::Error> {
        log::debug!("body == {}", std::str::from_utf8(body)?);

        Ok(Self {
            entities: serde_json::from_slice::<EntityCollection<T>>(body)?.value,
            continuation_token: ContinuationToken::parse_from_headers_optional(url, headers)?,
        })
    }
}

impl<T> std::convert::TryFrom<(ContinuationToken, &HeaderMap, &body::Bytes)>
    for PaginatedResponse<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;

    fn try_from(
        (continuation_token, headers, body): (ContinuationToken, &HeaderMap, &body::Bytes),
    ) -> Result<Self, Self::Error> {
        log::debug!("body == {}", std::str::from_utf8(body)?);

        Ok(Self {
            entities: serde_json::from_slice::<EntityCollection<T>>(body)?.value,
            continuation_token: ContinuationToken::parse_from_headers_optional(
                continuation_token.new_url,
                headers,
            )?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EntityCollection<T> {
    value: Vec<TableEntity<T>>,
}
