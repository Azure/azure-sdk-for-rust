use crate::EntityWithMetadata;
use azure_core::{
    headers::{etag_from_headers, get_str_from_headers, CommonStorageResponseHeaders},
    prelude::Etag,
    util::HeaderMapExt,
};
use bytes::Bytes;
use http::Response;
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

impl<E> TryFrom<&Response<Bytes>> for InsertEntityResponse<E>
where
    E: DeserializeOwned,
{
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("{}", std::str::from_utf8(response.body())?);
        debug!("headers == {:#?}", response.headers());

        let entity_with_metadata =
            match get_str_from_headers(response.headers(), "preference-applied")? {
                "return-no-content" => None,
                "return-content" => Some(response.try_into()?),
                _ => {
                    return Err(crate::Error::GenericErrorWithText(
                        "Unexpected value for preference-applied header".to_owned(),
                    ))
                }
            };

        Ok(InsertEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
            etag: etag_from_headers(response.headers())?.into(),
            location: response
                .headers()
                .get_as_str("location")
                .map(Url::parse)
                .transpose()?,
            entity_with_metadata,
        })
    }
}
