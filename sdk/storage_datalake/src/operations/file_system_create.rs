use crate::clients::FileSystemClient;
use crate::util::*;
use crate::Properties;
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
use std::convert::TryInto;

/// A future of a create file system response
type CreateFileSystem =
    futures::future::BoxFuture<'static, crate::Result<CreateFileSystemResponse>>;

#[derive(Debug, Clone)]
pub struct CreateFileSystemBuilder {
    client: FileSystemClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    properties: Option<Properties>,
}

impl CreateFileSystemBuilder {
    pub(crate) fn new(client: FileSystemClient) -> Self {
        Self {
            client,
            client_request_id: None,
            timeout: None,
            properties: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        properties: Properties => Some(properties),
    }

    pub fn into_future(self) -> CreateFileSystem {
        let this = self.clone();
        let ctx = self.client.context.clone();

        Box::pin(async move {
            let mut url = this.client.url().clone();
            url.query_pairs_mut().append_pair("resource", "filesystem");
            self.timeout.append_to_url_query(&mut url);

            let mut request = this
                .client
                .prepare_request_pipeline(url.as_str(), http::Method::PUT);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_optional_header2(&this.properties, &mut request)?;
            add_mandatory_header2(&ContentLength::new(0), &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            CreateFileSystemResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateFileSystemResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub last_modified: DateTime<Utc>,
    pub namespace_enabled: bool,
}

impl CreateFileSystemResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            etag: Etag::from(etag_from_headers(&headers)?),
            last_modified: last_modified_from_headers(&headers)?,
            namespace_enabled: namespace_enabled_from_headers(&headers)?,
        })
    }
}
