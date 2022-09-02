use crate::clients::FileSystemClient;
use azure_core::{prelude::*, Request};
use azure_core::{AppendToUrlQuery, Response as HttpResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteFileSystemBuilder {
    client: FileSystemClient,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl DeleteFileSystemBuilder {
    pub(crate) fn new(client: FileSystemClient) -> Self {
        Self {
            client,
            if_modified_since_condition: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> DeleteFileSystem {
        let this = self.clone();
        let ctx = self.client.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;
            self.timeout.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("resource", "filesystem");

            let mut request = Request::new(url, azure_core::Method::Delete);

            request.insert_headers(&this.client_request_id);
            request.insert_headers(&this.if_modified_since_condition);
            request.insert_headers(&ContentLength::new(0));

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            DeleteFileSystemResponse::try_from(response).await
        })
    }
}

azure_core::future!(DeleteFileSystem);

#[derive(Debug, Clone)]
pub struct DeleteFileSystemResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl DeleteFileSystemResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
        })
    }
}
