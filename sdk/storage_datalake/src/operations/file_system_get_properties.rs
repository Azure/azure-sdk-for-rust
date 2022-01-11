use crate::clients::FileSystemClient;
use crate::{util::*, Properties};
use azure_core::prelude::*;
use azure_core::{
    headers::{add_mandatory_header2, add_optional_header2},
    AppendToUrlQuery, Response as HttpResponse,
};
use azure_core::{
    headers::{etag_from_headers, last_modified_from_headers},
    Etag,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

/// A future of a file system get properties response
type GetFileSystemProperties =
    futures::future::BoxFuture<'static, crate::Result<GetFileSystemPropertiesResponse>>;

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
            let mut url = this.client.url().clone();
            self.timeout.append_to_url_query(&mut url);

            debug!("url = {}", url);

            let mut request = this
                .client
                .prepare_request_pipeline(url.as_str(), http::Method::HEAD);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_mandatory_header2(&ContentLength::new(0), &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            GetFileSystemPropertiesResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetFileSystemPropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub last_modified: DateTime<Utc>,
    pub namespace_enabled: bool,
    pub properties: Properties,
}

impl GetFileSystemPropertiesResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(GetFileSystemPropertiesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            etag: Etag::from(etag_from_headers(&headers)?),
            last_modified: last_modified_from_headers(&headers)?,
            namespace_enabled: namespace_enabled_from_headers(&headers)?,
            properties: Properties::try_from(&headers)?,
        })
    }
}
