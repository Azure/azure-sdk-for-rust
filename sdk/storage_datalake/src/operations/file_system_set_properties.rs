use crate::{clients::FileSystemClient, Properties};
use azure_core::{
    headers::{etag_from_headers, last_modified_from_headers},
    prelude::*,
    Etag, Request, Response as HttpResponse,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;
use time::OffsetDateTime;

operation! {
    SetFileSystemProperties,
    client: FileSystemClient,
    properties: Properties,
    ?if_modified_since_condition: IfModifiedSinceCondition
}

impl SetFileSystemPropertiesBuilder {
    pub fn into_future(self) -> SetFileSystemProperties {
        let this = self.clone();
        let mut ctx = self.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;
            url.query_pairs_mut().append_pair("resource", "filesystem");

            let mut request = Request::new(url, azure_core::Method::Patch);

            request.insert_headers(&this.if_modified_since_condition);
            request.insert_headers(&this.properties);
            request.insert_headers(&ContentLength::new(0));

            let response = self.client.send(&mut ctx, &mut request).await?;

            SetFileSystemPropertiesResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetFileSystemPropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub last_modified: OffsetDateTime,
}

impl SetFileSystemPropertiesResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(SetFileSystemPropertiesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            etag: Etag::from(etag_from_headers(&headers)?),
            last_modified: last_modified_from_headers(&headers)?,
        })
    }
}
