use crate::{blob::BlobProperties, prelude::*};
use azure_core::prelude::*;
use azure_core::{
    headers::{
        date_from_headers, etag_from_headers, request_id_from_headers, server_from_headers, Headers,
    },
    Method, RequestId,
};
use std::convert::{TryFrom, TryInto};
use time::OffsetDateTime;

operation! {
    SetProperties,
    client: BlobClient,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
    ?lease_id: LeaseId,
    ?cache_control: BlobCacheControl,
    ?content_type: BlobContentType,
    ?content_encoding: BlobContentEncoding,
    ?content_language: BlobContentLanguage,
    ?content_disposition: BlobContentDisposition,
    ?content_md5: BlobContentMD5
}

impl SetPropertiesBuilder {
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

    pub fn into_future(mut self) -> SetProperties {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "properties");

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.cache_control);
            headers.add(self.content_type);
            headers.add(self.content_encoding);
            headers.add(self.content_language);
            headers.add(self.content_disposition);
            headers.add(self.content_md5);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.if_tags);

            let mut request = self
                .client
                .finalize_request(url, Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            response.headers().try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetPropertiesResponse {
    pub request_id: RequestId,
    pub etag: String,
    pub server: String,
    pub date: OffsetDateTime,
}

impl TryFrom<&Headers> for SetPropertiesResponse {
    type Error = crate::Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(SetPropertiesResponse {
            request_id: request_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?,
            date: date_from_headers(headers)?,
        })
    }
}
