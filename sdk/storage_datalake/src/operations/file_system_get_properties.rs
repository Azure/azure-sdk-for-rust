use crate::{clients::FileSystemClient, util::*, Properties};
use azure_core::{
    headers::{etag_from_headers, last_modified_from_headers},
    prelude::*,
    Etag, Request, Response,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;
use time::OffsetDateTime;

operation! {
    GetFileSystemProperties,
    client: FileSystemClient,
}

impl GetFileSystemPropertiesBuilder {
    pub fn into_future(self) -> GetFileSystemProperties {
        Box::pin(async move {
            let mut url = self.client.url()?;
            url.query_pairs_mut().append_pair("resource", "filesystem");

            let mut request = Request::new(url, azure_core::Method::Head);

            request.insert_headers(&ContentLength::new(0));

            let response = self
                .client
                .send(&mut self.context.clone(), &mut request)
                .await?;

            GetFileSystemPropertiesResponse::try_from(response)
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetFileSystemPropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub last_modified: OffsetDateTime,
    pub namespace_enabled: bool,
    pub properties: Properties,
}

impl GetFileSystemPropertiesResponse {
    pub fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(GetFileSystemPropertiesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            etag: Etag::from(etag_from_headers(&headers)?),
            last_modified: last_modified_from_headers(&headers)?,
            namespace_enabled: namespace_enabled_from_headers(&headers)?,
            properties: (&headers).try_into()?,
        })
    }
}
