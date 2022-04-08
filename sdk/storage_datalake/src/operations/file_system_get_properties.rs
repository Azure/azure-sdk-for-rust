use crate::clients::FileSystemClient;
use crate::{
    request_options::{PathGetPropertiesAction, PathGetPropertiesFsAction},
    util::*,
    Properties,
};
use azure_core::prelude::*;
use azure_core::{
    headers::{etag_from_headers, last_modified_from_headers},
    AppendToUrlQuery, Etag, Response as HttpResponse,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use chrono::{DateTime, Utc};
use std::convert::TryInto;

/// A future of a file system get properties response
type GetFileSystemProperties =
    futures::future::BoxFuture<'static, crate::Result<GetFileSystemPropertiesResponse>>;

#[derive(Debug, Clone)]
pub struct GetFileSystemPropertiesBuilder {
    client: FileSystemClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    action: Option<PathGetPropertiesAction>,
    fs_action: Option<PathGetPropertiesFsAction>,
}

impl GetFileSystemPropertiesBuilder {
    pub(crate) fn new(client: FileSystemClient) -> Self {
        Self {
            client,
            client_request_id: None,
            timeout: None,
            action: None,
            fs_action: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        action: PathGetPropertiesAction => Some(action),
        fs_action: PathGetPropertiesFsAction => Some(fs_action),
    }

    pub fn into_future(self) -> GetFileSystemProperties {
        let this = self.clone();
        let ctx = self.client.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;
            if self.action.is_some() {
                self.action.append_to_url_query(&mut url);

                // For CheckAccess, we must also specify the "fsAction"
                if let Some(PathGetPropertiesAction::CheckAccess) = self.action {
                    self.fs_action.append_to_url_query(&mut url);
                }
            }

            self.timeout.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("resource", "filesystem");

            let mut request = this
                .client
                .prepare_request(url.as_str(), http::Method::HEAD);

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
            properties: (&headers).try_into()?,
        })
    }
}
