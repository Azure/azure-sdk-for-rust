use crate::clients::prepare_file_system_url;
use azure_core::prelude::*;
use azure_core::{
    collect_pinned_stream,
    headers::{add_mandatory_header2, add_optional_header2},
    AppendToUrlQuery, Context, Response as HttpResponse,
};
use azure_storage::{core::headers::CommonStorageResponseHeaders, prelude::*};
use std::convert::TryFrom;

/// A future of a create file system response
type DeleteFileSystem =
    futures::future::BoxFuture<'static, crate::Result<DeleteFileSystemResponse>>;

#[derive(Debug, Clone)]
pub struct DeleteFileSystemBuilder {
    client: StorageAccountClient,
    name: String,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    context: Option<Context>,
}

impl DeleteFileSystemBuilder {
    pub(crate) fn new(
        client: StorageAccountClient,
        name: String,
        context: Option<Context>,
    ) -> Self {
        Self {
            client,
            name,
            if_modified_since_condition: None,
            client_request_id: None,
            timeout: None,
            context,
        }
    }

    setters! {
        name: String => name,
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        context: Context => Some(context),
    }

    pub fn into_future(self) -> DeleteFileSystem {
        let this = self.clone();
        let ctx = self.context.clone().unwrap_or_default();

        Box::pin(async move {
            let mut url = prepare_file_system_url(&this.client, &this.name);
            self.timeout.append_to_url_query(&mut url);

            debug!("url = {}", url);

            let mut request = this
                .client
                .prepare_request_pipeline(url.as_str(), http::Method::DELETE);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_optional_header2(&this.if_modified_since_condition, &mut request)?;
            add_mandatory_header2(&ContentLength::new(0), &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            DeleteFileSystemResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteFileSystemResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl DeleteFileSystemResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        trace!("body == {}", std::str::from_utf8(&body)?);
        trace!("headers == {:?}", headers);

        Ok(Self {
            common_storage_response_headers: CommonStorageResponseHeaders::try_from(&headers)?,
        })
    }
}
