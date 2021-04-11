use azure_core::{errors::AzureError, headers::CommonStorageResponseHeaders};
use bytes::Bytes;
use http::Response;
use serde::de::DeserializeOwned;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct GetEntityResponse<E>
where
    E: DeserializeOwned,
{
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: String,
    pub entity: E,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetEntityResponseInternal<E> {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(flatten)]
    pub value: E
}

impl<E> TryFrom<&Response<Bytes>> for GetEntityResponse<E>
where
    E: DeserializeOwned,
{
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("{}", std::str::from_utf8(response.body())?);
        debug!("headers == {:#?}", response.headers());

        let get_entity_response_internal: GetEntityResponseInternal<E> =
            serde_json::from_slice(response.body())?;

        Ok(GetEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
            metadata: get_entity_response_internal.metadata,
            entity: get_entity_response_internal.value,
        })
    }
}
