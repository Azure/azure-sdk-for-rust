use crate::EntityWithMetadata;
use azure_core::{
    error::{Error, ErrorKind},
    headers::{etag_from_headers, get_str_from_headers},
    CollectedResponse, Etag,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use serde::de::DeserializeOwned;
use std::convert::{TryFrom, TryInto};
use url::Url;

#[derive(Debug, Clone)]
pub struct InsertEntityResponse<E>
where
    E: DeserializeOwned,
{
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub location: Option<Url>,
    pub entity_with_metadata: Option<EntityWithMetadata<E>>,
}

impl<E> TryFrom<CollectedResponse> for InsertEntityResponse<E>
where
    E: DeserializeOwned,
{
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let headers = response.headers();
        let entity_with_metadata = match get_str_from_headers(headers, "preference-applied")? {
            "return-no-content" => None,
            "return-content" => Some(response.clone().try_into()?),
            _ => {
                return Err(Error::message(
                    ErrorKind::Other,
                    "Unexpected value for preference-applied header",
                ))
            }
        };

        Ok(InsertEntityResponse {
            common_storage_response_headers: headers.try_into()?,
            etag: etag_from_headers(headers)?.into(),
            location: headers
                .get("location")
                .map(|location| Url::parse(location.as_str()))
                .transpose()?,
            entity_with_metadata,
        })
    }
}
