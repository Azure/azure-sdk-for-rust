// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{convert::Infallible, str::FromStr};

use super::{
    matchers::Matcher,
    models::{PlaybackStartResult, RecordStartResult, StartPayload, VariablePayload},
    sanitizers::Sanitizer,
};
use azure_core::{
    headers::{AsHeaders, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
    ClientMethodOptions, ClientOptions, Context, Header, Method, Pipeline, Request, RequestContent,
    Result, Url,
};

const X_RECORDING_ID: HeaderName = HeaderName::from_static("x-recording-id");

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
    pub fn new(endpoint: Url) -> Result<Self> {
        Ok(Self {
            endpoint,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                ClientOptions::default(),
                Vec::default(),
                Vec::default(),
            ),
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub async fn record_start(
        &self,
        body: RequestContent<StartPayload>,
        options: Option<ClientRecordStartOptions<'_>>,
    ) -> Result<RecordStartResult> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Record/Start")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.set_body(body);
        let resp = self
            .pipeline
            .send::<RecordStartResult>(&ctx, &mut request)
            .await?;
        let recording_id = resp.headers().get_str(&X_RECORDING_ID)?.to_string();
        let mut result: RecordStartResult = resp.into_json_body().await?;
        result.recording_id = recording_id;
        Ok(result)
    }

    pub async fn record_stop(
        &self,
        recording_id: &str,
        body: RequestContent<VariablePayload>,
        options: Option<ClientRecordStopOptions<'_>>,
    ) -> Result<()> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Record/Stop")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.insert_header(X_RECORDING_ID, recording_id.to_string());
        request.set_body(body);
        self.pipeline.send::<()>(&ctx, &mut request).await?;
        Ok(())
    }

    pub async fn playback_start(
        &self,
        body: RequestContent<StartPayload>,
        options: Option<ClientPlaybackStartOptions<'_>>,
    ) -> Result<PlaybackStartResult> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Playback/Start")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.add_optional_header(&options.recording_id);
        request.set_body(body);
        let resp = self
            .pipeline
            .send::<RecordStartResult>(&ctx, &mut request)
            .await?;
        let recording_id = resp.headers().get_str(&X_RECORDING_ID)?.to_string();
        let mut result: PlaybackStartResult = resp.into_json_body().await?;
        result.recording_id = recording_id;
        Ok(result)
    }

    pub async fn playback_stop(
        &self,
        recording_id: &str,
        options: Option<ClientPlaybackStopOptions<'_>>,
    ) -> Result<()> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Playback/Stop")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.insert_header(X_RECORDING_ID, recording_id.to_string());
        self.pipeline.send::<()>(&ctx, &mut request).await?;
        Ok(())
    }

    pub async fn set_matcher(
        &self,
        matcher: Matcher,
        options: Option<ClientSetMatcherOptions<'_>>,
    ) -> Result<()> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/SetMatcher")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_headers(&matcher)?;
        request.add_optional_header(&options.recording_id);
        self.pipeline.send::<()>(&ctx, &mut request).await?;
        Ok(())
    }

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
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/AddSanitizer")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_headers(&sanitizer)?;
        request.add_optional_header(&options.recording_id);
        self.pipeline.send::<()>(&ctx, &mut request).await?;
        Ok(())
    }

    pub async fn reset(&self, options: Option<ClientResetOptions<'_>>) -> Result<()> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/Reset")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.add_optional_header(&options.recording_id);
        self.pipeline.send::<()>(&ctx, &mut request).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RecordingId(String);

impl Header for &RecordingId {
    fn name(&self) -> HeaderName {
        X_RECORDING_ID
    }

    fn value(&self) -> HeaderValue {
        self.0.clone().into()
    }
}

impl AsRef<str> for RecordingId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for RecordingId {
    type Err = Infallible;
    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        Ok(RecordingId(value.to_string()))
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
