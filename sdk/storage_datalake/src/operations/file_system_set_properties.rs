use crate::clients::prepare_file_system_url;
use crate::Properties;
use azure_core::prelude::*;
use azure_core::{
    headers::add_optional_header2, AppendToUrlQuery, Context, Response as HttpResponse,
};
use azure_core::{
    headers::{etag_from_headers, last_modified_from_headers},
    Etag,
};
use azure_storage::{core::headers::CommonStorageResponseHeaders, prelude::*};
use chrono::{DateTime, Utc};
use http::header::{HeaderValue, CONTENT_LENGTH};
use std::convert::TryFrom;

/// A future of a file system set properties response
type SetFileSystemProperties =
    futures::future::BoxFuture<'static, crate::Result<SetFileSystemPropertiesResponse>>;

#[derive(Debug, Clone)]
pub struct SetFileSystemPropertiesBuilder {
    client: StorageAccountClient,
    name: String,
    properties: Option<Properties>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    context: Option<Context>,
}

impl SetFileSystemPropertiesBuilder {
    pub(crate) fn new(
        client: StorageAccountClient,
        name: String,
        properties: Option<Properties>,
        context: Option<Context>,
    ) -> Self {
        Self {
            client,
            name,
            if_modified_since_condition: None,
            client_request_id: None,
            timeout: None,
            properties,
            context,
        }
    }

    setters! {
        name: String => name,
        properties: Properties => Some(properties),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        context: Context => Some(context),
    }

    pub fn into_future(self) -> SetFileSystemProperties {
        let this = self.clone();
        let ctx = self.context.clone().unwrap_or_default();

        Box::pin(async move {
            let mut url = prepare_file_system_url(&this.client, &this.name);
            self.timeout.append_to_url_query(&mut url);

            debug!("url = {}", url);

            let mut request = this
                .client
                .prepare_request_pipeline(url.as_str(), http::Method::PATCH);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_optional_header2(&this.if_modified_since_condition, &mut request)?;
            add_optional_header2(&this.properties, &mut request)?;

            request
                .headers_mut()
                .insert(CONTENT_LENGTH, HeaderValue::from_str("0").unwrap());

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
            common_storage_response_headers: CommonStorageResponseHeaders::try_from(&headers)?,
            etag: Etag::from(etag_from_headers(&headers)?),
            last_modified: last_modified_from_headers(&headers)?,
        })
    }
}
