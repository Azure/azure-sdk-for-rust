use crate::clients::FileSystemClient;
use azure_core::{prelude::*, Request, Response};
use azure_storage::headers::CommonStorageResponseHeaders;

operation! {
    DeleteFileSystem,
    client: FileSystemClient,
    ?if_modified_since_condition: IfModifiedSinceCondition
}

impl DeleteFileSystemBuilder {
    pub fn into_future(self) -> DeleteFileSystem {
        Box::pin(async move {
            let mut url = self.client.url()?;
            url.query_pairs_mut().append_pair("resource", "filesystem");

            let mut request = Request::new(url, azure_core::Method::DELETE);

            request.insert_headers(&self.if_modified_since_condition);
            request.insert_headers(&ContentLength::new(0));

            let response = self
                .client
                .send(&mut self.context.clone(), &mut request)
                .await?;

            DeleteFileSystemResponse::try_from(response)
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteFileSystemResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl DeleteFileSystemResponse {
    pub fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
        })
    }
}
