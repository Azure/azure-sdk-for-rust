use super::prelude::*;
use super::rest_client::{AZURE_VERSION, HEADER_DATE, HEADER_VERSION};
use crate::PerformRequestResponse;
use azure_core::errors::AzureError;
use azure_core::util::{format_header_value, RequestBuilderExt};
use http::request::Builder;
use hyper::{header, Method};
use hyper_rustls::HttpsConnector;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct BearerTokenClient<'a> {
    account: Cow<'a, str>,
    bearer_token: Cow<'a, str>,
    hc: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
    blob_uri: String,
    table_uri: String,
    queue_uri: String,
    filesystem_uri: String,
}

impl<'a> BearerTokenClient<'a> {
    #[inline]
    pub(crate) fn new(
        account: Cow<'a, str>,
        bearer_token: Cow<'a, str>,
        hc: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
    ) -> Self {
        let blob_uri = format!("https://{}.blob.core.windows.net", account);
        let table_uri = format!("https://{}.table.core.windows.net", account);
        let queue_uri = format!("https://{}.queue.core.windows.net", account);
        let filesystem_uri = format!("https://{}.dfs.core.windows.net", account);

        Self {
            account,
            bearer_token,
            queue_uri,
            hc,
            blob_uri,
            table_uri,
            filesystem_uri,
        }
    }

    fn perform_request_internal(
        &self,
        uri: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&[u8]>,
    ) -> Result<PerformRequestResponse, AzureError> {
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

        let mut request = hyper::Request::builder();
        request = request.method(method).uri(uri);
        let uri = url::Url::parse(uri)?;

        // let's add content length to avoid "chunking" errors.
        match request_body {
            Some(ref b) => {
                request = request.header(header::CONTENT_LENGTH, &b.len().to_string() as &str)
            }
            None => request = request.header_static(header::CONTENT_LENGTH, "0"),
        };

        // This will give the caller the ability to add custom headers.
        // The closure is needed to because request.headers_mut().set_raw(...) requires
        // a Cow with 'static lifetime...
        request = http_header_adder(request);

        request = request
            .header_bytes(HEADER_DATE, time)
            .header_static(HEADER_VERSION, AZURE_VERSION);

        let b = request_body
            .map(|v| Vec::from(v).into())
            .unwrap_or_else(hyper::Body::empty);
        let mut request = request.body(b)?;

        request.headers_mut().insert(
            header::AUTHORIZATION,
            format_header_value(format!("Bearer {}", self.bearer_token))?,
        );

        Ok((uri, self.hc.request(request)).into())
    }
}

impl<'a> Client for BearerTokenClient<'a> {
    #[inline]
    fn blob_uri(&self) -> &str {
        &self.blob_uri
    }

    #[inline]
    fn table_uri(&self) -> &str {
        &self.table_uri
    }

    #[inline]
    fn queue_uri(&self) -> &str {
        &self.queue_uri
    }

    #[inline]
    fn filesystem_uri(&self) -> &str {
        &self.filesystem_uri
    }

    #[inline]
    fn perform_request(
        &self,
        uri: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&[u8]>,
    ) -> Result<PerformRequestResponse, AzureError> {
        self.perform_request_internal(uri, method, http_header_adder, request_body)
    }

    #[inline]
    fn perform_table_request(
        &self,
        segment: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&[u8]>,
    ) -> Result<PerformRequestResponse, AzureError> {
        self.perform_request_internal(segment, method, http_header_adder, request_body)
    }
}
