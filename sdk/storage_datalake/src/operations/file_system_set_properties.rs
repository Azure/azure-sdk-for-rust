use crate::clients::FileSystemClient;
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

/// A future of a file system set properties response
type SetFileSystemProperties =
    futures::future::BoxFuture<'static, crate::Result<SetFileSystemPropertiesResponse>>;

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
            let mut url = this.client.url().clone();
            self.timeout.append_to_url_query(&mut url);

            debug!("url = {}", url);

            let mut request = this
                .client
                .prepare_request_pipeline(url.as_str(), http::Method::PATCH);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_optional_header2(&this.if_modified_since_condition, &mut request)?;
            add_optional_header2(&this.properties, &mut request)?;
            add_mandatory_header2(&ContentLength::new(0), &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            SetFileSystemPropertiesResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetFileSystemPropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub last_modified: DateTime<Utc>,
}

impl SetFileSystemPropertiesResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        trace!("headers == {:?}", headers);

        Ok(SetFileSystemPropertiesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            etag: Etag::from(etag_from_headers(&headers)?),
            last_modified: last_modified_from_headers(&headers)?,
        })
    }
}
