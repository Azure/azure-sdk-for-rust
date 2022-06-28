use crate::{blob::BlobProperties, prelude::*};
use azure_core::prelude::*;
use azure_core::{
    headers::{
        date_from_headers, etag_from_headers, request_id_from_headers, server_from_headers, Headers,
    },
    Method, RequestId,
};
use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct SetPropertiesBuilder {
    blob_client: BlobClient,
    lease_id: Option<LeaseId>,
    timeout: Option<Timeout>,
    cache_control: Option<BlobCacheControl>,
    content_type: Option<BlobContentType>,
    content_encoding: Option<BlobContentEncoding>,
    content_language: Option<BlobContentLanguage>,
    content_disposition: Option<BlobContentDisposition>,
    content_md5: Option<BlobContentMD5>,
    context: Context,
}

impl SetPropertiesBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            lease_id: None,
            timeout: None,
            cache_control: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            content_md5: None,
            context: Context::new(),
        }
    }

    pub fn set_from_blob_properties(self, blob_properties: BlobProperties) -> Self {
        let mut s = self;

        if let Some(cc) = blob_properties.cache_control {
            s = s.cache_control(cc);
        }
        if !blob_properties.content_type.is_empty() {
            s = s.content_type(blob_properties.content_type);
        }
        if let Some(ce) = blob_properties.content_encoding {
            s = s.content_encoding(ce);
        }
        if let Some(cl) = blob_properties.content_language {
            s = s.content_language(cl);
        }
        if let Some(cd) = blob_properties.content_disposition {
            s = s.content_disposition(cd);
        }
        if let Some(cmd5) = blob_properties.content_md5 {
            s = s.content_md5(cmd5);
        }
        s
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
        cache_control: BlobCacheControl => Some(cache_control),
        content_type: BlobContentType => Some(content_type),
        content_encoding: BlobContentEncoding => Some(content_encoding),
        content_language: BlobContentLanguage => Some(content_language),
        content_disposition: BlobContentDisposition => Some(content_disposition),
        content_md5: BlobContentMD5 => Some(content_md5),
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "properties");
            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.cache_control);
            headers.add(self.content_type);
            headers.add(self.content_encoding);
            headers.add(self.content_language);
            headers.add(self.content_disposition);
            headers.add(self.content_md5);

            let mut request = self
                .blob_client
                .prepare_request(url, Method::Put, headers, None)?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
            response.headers().try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetPropertiesResponse {
    pub request_id: RequestId,
    pub etag: String,
    pub server: String,
    pub date: DateTime<Utc>,
}

impl TryFrom<&Headers> for SetPropertiesResponse {
    type Error = crate::Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(SetPropertiesResponse {
            request_id: request_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
        })
    }
}
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<SetPropertiesResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for SetPropertiesBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
