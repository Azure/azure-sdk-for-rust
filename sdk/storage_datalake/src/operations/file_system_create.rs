use crate::clients::prepare_file_system_url;
use crate::util::*;
use crate::Properties;
use azure_core::prelude::*;
use azure_core::{
    collect_pinned_stream, headers::add_optional_header2, AppendToUrlQuery, Context,
    Response as HttpResponse,
};
use azure_core::{
    headers::{etag_from_headers, last_modified_from_headers},
    Etag,
};
use azure_storage::{core::headers::CommonStorageResponseHeaders, prelude::*};
use chrono::{DateTime, Utc};
use http::header::{HeaderValue, CONTENT_LENGTH};
use std::convert::TryFrom;

/// A future of a create file system response
type CreateFileSystem =
    futures::future::BoxFuture<'static, crate::Result<CreateFileSystemResponse>>;

#[derive(Debug, Clone)]
pub struct CreateFileSystemBuilder {
    client: StorageAccountClient,
    name: String,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    properties: Option<Properties>,
    context: Option<Context>,
}

impl CreateFileSystemBuilder {
    pub(crate) fn new(
        client: StorageAccountClient,
        name: String,
        context: Option<Context>,
    ) -> Self {
        Self {
            client,
            name,
            client_request_id: None,
            timeout: None,
            properties: None,
            context,
        }
    }

    setters! {
        name: String => name,
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        properties: Properties => Some(properties),
        context: Context => Some(context),
    }

    pub fn into_future(self) -> CreateFileSystem {
        let this = self.clone();
        let ctx = self.context.clone().unwrap_or_default();

        Box::pin(async move {
            let mut url = prepare_file_system_url(&this.client, &this.name);
            self.timeout.append_to_url_query(&mut url);

            debug!("url = {}", url);

            let mut request = this
                .client
                .prepare_request_pipeline(url.as_str(), http::Method::PUT);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_optional_header2(&this.properties, &mut request)?;

            request
                .headers_mut()
                .insert(CONTENT_LENGTH, HeaderValue::from_str("0").unwrap());

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
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        trace!("body == {}", std::str::from_utf8(&body)?);
        trace!("headers == {:?}", headers);

        Ok(Self {
            common_storage_response_headers: CommonStorageResponseHeaders::try_from(&headers)?,
            etag: Etag::from(etag_from_headers(&headers)?),
            last_modified: last_modified_from_headers(&headers)?,
            namespace_enabled: namespace_enabled_from_headers(&headers)?,
        })
    }
}
