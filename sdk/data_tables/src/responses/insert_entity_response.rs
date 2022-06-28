use crate::EntityWithMetadata;
use azure_core::{
    error::{Error, ErrorKind},
    headers::{self, etag_from_headers, HeaderName},
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
        let entity_with_metadata =
            match headers.get_str(&HeaderName::from_static("preference-applied"))? {
                "return-no-content" => None,
                "return-content" => Some(response.clone().try_into()?),
                _ => {
                    return Err(Error::message(
                        ErrorKind::DataConversion,
                        "Unexpected value for preference-applied header",
                    ))
                }
            };

        Ok(InsertEntityResponse {
            common_storage_response_headers: headers.try_into()?,
            etag: etag_from_headers(headers)?.into(),
            location: headers.get_optional_as(&headers::LOCATION)?,
            entity_with_metadata,
        })
    }
}
