use crate::clients::FileSystemClient;
use crate::{util::*, Properties};
use azure_core::{
    headers::{etag_from_headers, last_modified_from_headers},
    AppendToUrlQuery, Etag, Response as HttpResponse,
};
use azure_core::{prelude::*, Request};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct GetFileSystemPropertiesBuilder {
    client: FileSystemClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl GetFileSystemPropertiesBuilder {
    pub(crate) fn new(client: FileSystemClient) -> Self {
        Self {
            client,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> GetFileSystemProperties {
        let this = self.clone();
        let ctx = self.client.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;
            self.timeout.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("resource", "filesystem");

            let mut request = Request::new(url, azure_core::Method::Head);

            request.insert_headers(&this.client_request_id);
            request.insert_headers(&ContentLength::new(0));

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            GetFileSystemPropertiesResponse::try_from(response).await
        })
    }
}

azure_core::future!(GetFileSystemProperties);

#[derive(Debug, Clone)]
pub struct GetFileSystemPropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub last_modified: OffsetDateTime,
    pub namespace_enabled: bool,
    pub properties: Properties,
}

impl GetFileSystemPropertiesResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
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
