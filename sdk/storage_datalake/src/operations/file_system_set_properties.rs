use crate::clients::FileSystemClient;
use crate::Properties;
use azure_core::{
    headers::{etag_from_headers, last_modified_from_headers},
    AppendToUrlQuery, Etag, Response as HttpResponse,
};
use azure_core::{prelude::*, Request};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct SetFileSystemPropertiesBuilder {
    client: FileSystemClient,
    properties: Option<Properties>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl SetFileSystemPropertiesBuilder {
    pub(crate) fn new(client: FileSystemClient, properties: Option<Properties>) -> Self {
        Self {
            client,
            if_modified_since_condition: None,
            client_request_id: None,
            timeout: None,
            properties,
        }
    }

    setters! {
        properties: Properties => Some(properties),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> SetFileSystemProperties {
        let this = self.clone();
        let ctx = self.client.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;
            self.timeout.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("resource", "filesystem");

            let mut request = Request::new(url, azure_core::Method::Patch);

            request.insert_headers(&this.client_request_id);
            request.insert_headers(&this.if_modified_since_condition);
            request.insert_headers(&this.properties);
            request.insert_headers(&ContentLength::new(0));

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            SetFileSystemPropertiesResponse::try_from(response).await
        })
    }
}

azure_core::future!(SetFileSystemProperties);

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
