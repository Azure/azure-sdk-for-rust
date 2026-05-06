// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::InstrumentOptions;

use super::{
    matchers::Matcher,
    models::{
        PlaybackStartResult, RecordStartResult, RemovedSanitizers, SanitizerList, StartPayload,
        VariablePayload,
    },
    sanitizers::Sanitizer,
    RecordingId, RECORDING_ID,
};
use azure_core::{
    error::{ErrorKind, ResultExt},
    http::{
        headers::{AsHeaders, ACCEPT, CONTENT_TYPE},
        request::{Request, RequestContent},
        ClientMethodOptions, ClientOptions, HttpClient, Method, Pipeline, PipelineSendOptions,
        Response, Transport, Url,
    },
    Bytes, Result,
};
use reqwest::Certificate;
use std::{io::Read, path::Path, sync::Arc, time::Duration};
use tracing::{debug, Span};

static CA_PEM: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();

/// Options [`Recording::instrument()`].
///
/// This should be a subset of [`azure_core::http::HttpClientOptions`].
#[derive(Clone, Debug)]
pub struct HttpClientOptions {
    /// Automatically decompress responses if the `content-encoding` header indicates a supported compression encoding.
    /// Defaults to `true`.
    pub automatic_decompression: bool,
}

impl Default for HttpClientOptions {
    fn default() -> Self {
        Self {
            automatic_decompression: true,
        }
    }
}

impl From<InstrumentOptions> for HttpClientOptions {
    fn from(options: InstrumentOptions) -> Self {
        Self {
            automatic_decompression: options.automatic_decompression,
        }
    }
}

/// Creates a new [`reqwest::Client`] configured to test-proxy.
///
/// This should work like [`azure_core::http::new_http_client()`] but with appropriate TLS configuration for self-signed TLS certificates.
pub fn new_http_client(options: Option<HttpClientOptions>) -> Result<Arc<dyn HttpClient>> {
    // TODO: As we design https://github.com/Azure/azure-sdk-for-rust/issues/4217 we should consider how we can use some sort of callback or `#[cfg(test)]`-guarded field to obviate this function.

    debug!("creating an http client using `reqwest`");
    let options = options.unwrap_or_default();

    const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(20);
    const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(60);

    let pem = CA_PEM.get().ok_or_else(|| {
        azure_core::Error::new(
            ErrorKind::Other,
            "expected self-signed CA to be initialized",
        )
    })?;
    let ca = Certificate::from_pem(pem)
        .with_context(ErrorKind::Other, "failed to decode self-signed CA")?;

    let client = ::reqwest::ClientBuilder::new()
        .connect_timeout(DEFAULT_CONNECTION_TIMEOUT)
        .read_timeout(DEFAULT_READ_TIMEOUT)
        .gzip(options.automatic_decompression)
        .deflate(options.automatic_decompression)
        // By default, reqwest will chase 3xx redirects up to 10 links. REST API guidelines
        // discourage services from using 3xx redirects, so disabling the reqwest redirect logic
        // simplifies the client logic.
        .redirect(::reqwest::redirect::Policy::none())
        // Accept self-signed CA for test-proxy on localhost.
        .tls_backend_rustls()
        .tls_certs_merge([ca])
        // BUGBUG: reqwest/rustls does not accept self-signed CAs for TLS: https://github.com/seanmonstar/reqwest/issues/1554
        .tls_danger_accept_invalid_certs(true)
        .build()
        .with_context(ErrorKind::Other, "failed to build test-proxy client")?;

    Ok(Arc::new(client))
}

/// The test-proxy client.
///
/// See <https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md> for usage.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Client {
    endpoint: Url,
    pipeline: Pipeline,
}

#[allow(dead_code)]
impl Client {
    pub fn new(endpoint: Url, ca_path: &Path, options: Option<HttpClientOptions>) -> Result<Self> {
        let _ = CA_PEM.get_or_init(|| {
            let mut pem = Vec::new();
            std::fs::File::open(ca_path)
                .unwrap_or_else(|_| panic!("failed to open '{}'", ca_path.display()))
                .read_to_end(&mut pem)
                .unwrap_or_else(|_| panic!("failed to read '{}'", ca_path.display()));
            pem
        });

        let client = new_http_client(options)?;
        let transport = Transport::new(client.clone());

        let options = ClientOptions {
            transport: Some(transport),
            ..Default::default()
        };
        Ok(Self {
            endpoint,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options,
                Vec::default(),
                Vec::default(),
                None,
            ),
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint), err)]
    pub async fn record_start(
        &self,
        body: RequestContent<StartPayload>,
        options: Option<ClientRecordStartOptions<'_>>,
    ) -> Result<RecordStartResult> {
        let options = options.unwrap_or_default();
        let ctx = options.method_options.context.to_borrowed();
        let mut url = self.endpoint.clone();
        url = url.join("/Record/Start")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.set_body(body);
        let resp: Response<RecordStartResult> = self
            .pipeline
            .send(
                &ctx,
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?
            .into();
        let recording_id = resp.headers().get_str(&RECORDING_ID)?.to_string();
        Ok(RecordStartResult { recording_id })
    }

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint, recording_id), err)]
    pub async fn record_stop(
        &self,
        recording_id: &str,
        body: RequestContent<VariablePayload>,
        options: Option<ClientRecordStopOptions<'_>>,
    ) -> Result<()> {
        let options = options.unwrap_or_default();
        let ctx = options.method_options.context.to_borrowed();
        let mut url = self.endpoint.clone();
        url = url.join("/Record/Stop")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_header(RECORDING_ID, recording_id.to_string());
        request.set_body(body);
        self.pipeline
            .send(
                &ctx,
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint, recording_id), err)]
    pub async fn playback_start(
        &self,
        body: RequestContent<StartPayload>,
        options: Option<ClientPlaybackStartOptions<'_>>,
    ) -> Result<PlaybackStartResult> {
        let options = options.unwrap_or_default();
        Span::current().record(
            stringify!(recording_id),
            options.recording_id.map(AsRef::as_ref),
        );
        let ctx = options.method_options.context.to_borrowed();
        let mut url = self.endpoint.clone();
        url = url.join("/Playback/Start")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.add_optional_header(&options.recording_id);
        request.set_body(body);
        let resp: Response<PlaybackStartResult> = self
            .pipeline
            .send(
                &ctx,
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?
            .into();
        let recording_id = resp.headers().get_str(&RECORDING_ID)?.to_string();
        let mut result: PlaybackStartResult = resp.into_model()?;
        result.recording_id = recording_id;
        Ok(result)
    }

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint, recording_id), err)]
    pub async fn playback_stop(
        &self,
        recording_id: &str,
        options: Option<ClientPlaybackStopOptions<'_>>,
    ) -> Result<()> {
        let options = options.unwrap_or_default();
        let ctx = options.method_options.context.to_borrowed();
        let mut url = self.endpoint.clone();
        url = url.join("/Playback/Stop")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_header(RECORDING_ID, recording_id.to_string());
        self.pipeline
            .send(
                &ctx,
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint, recording_id), err)]
    pub async fn set_matcher(
        &self,
        matcher: Matcher,
        options: Option<ClientSetMatcherOptions<'_>>,
    ) -> Result<()> {
        let options = options.unwrap_or_default();
        Span::current().record(
            stringify!(recording_id),
            options.recording_id.map(AsRef::as_ref),
        );
        let ctx = options.method_options.context.to_borrowed();
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/SetMatcher")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_headers(&matcher)?;
        request.add_optional_header(&options.recording_id);
        let body: Bytes = matcher.try_into()?;
        request.set_body(body);
        self.pipeline
            .send(
                &ctx,
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint, recording_id), err)]
    pub async fn add_sanitizer<S>(
        &self,
        sanitizer: S,
        options: Option<ClientAddSanitizerOptions<'_>>,
    ) -> Result<()>
    where
        S: Sanitizer,
        azure_core::Error: From<<S as AsHeaders>::Error>,
    {
        let options = options.unwrap_or_default();
        Span::current().record(
            stringify!(recording_id),
            options.recording_id.map(AsRef::as_ref),
        );
        let ctx = options.method_options.context.to_borrowed();
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/AddSanitizer")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_headers(&sanitizer)?;
        request.add_optional_header(&options.recording_id);
        self.pipeline
            .send(
                &ctx,
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint, recording_id), err)]
    pub async fn remove_sanitizers(
        &self,
        body: RequestContent<SanitizerList>,
        options: Option<ClientRemoveSanitizersOptions<'_>>,
    ) -> Result<RemovedSanitizers> {
        let options = options.unwrap_or_default();
        Span::current().record(
            stringify!(recording_id),
            options.recording_id.map(AsRef::as_ref),
        );
        let ctx = options.method_options.context.to_borrowed();
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/RemoveSanitizers")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.add_optional_header(&options.recording_id);
        request.set_body(body);
        self.pipeline
            .send(
                &ctx,
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?
            .into_body()
            .json()
    }

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint, recording_id), err)]
    pub async fn reset(&self, options: Option<ClientResetOptions<'_>>) -> Result<()> {
        let options = options.unwrap_or_default();
        Span::current().record(
            stringify!(recording_id),
            options.recording_id.map(AsRef::as_ref),
        );
        let ctx = options.method_options.context.to_borrowed();
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/Reset")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.add_optional_header(&options.recording_id);
        self.pipeline
            .send(
                &ctx,
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ClientRecordStartOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Debug, Default)]
pub struct ClientRecordStopOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Debug, Default)]
pub struct ClientPlaybackStartOptions<'a> {
    /// The recording ID.
    ///
    /// Note: this is really only meant for performance testing
    /// and should not normally be passed for normal client testing.
    pub recording_id: Option<&'a RecordingId>,
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Debug, Default)]
pub struct ClientPlaybackStopOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Debug, Default)]
pub struct ClientSetMatcherOptions<'a> {
    pub recording_id: Option<&'a RecordingId>,
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Debug, Default)]
pub struct ClientAddSanitizerOptions<'a> {
    pub recording_id: Option<&'a RecordingId>,
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Debug, Default)]
pub struct ClientResetOptions<'a> {
    /// Reset the test-proxy only for the given recording ID.
    ///
    /// If `None`, the test-proxy is reset including any per-instance defaults
    /// not hardcoded into the test-proxy itself.
    pub recording_id: Option<&'a RecordingId>,
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Debug, Default)]
pub struct ClientRemoveSanitizersOptions<'a> {
    /// Remove sanitizers only for the given recording ID.
    pub recording_id: Option<&'a RecordingId>,
    pub method_options: ClientMethodOptions<'a>,
}
