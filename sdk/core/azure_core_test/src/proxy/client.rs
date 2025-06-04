// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

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
    http::{
        headers::{AsHeaders, ACCEPT, CONTENT_TYPE},
        request::{Request, RequestContent},
        ClientMethodOptions, ClientOptions, Context, Method, Pipeline, Response, Url,
    },
    Bytes, Result,
};
use tracing::Span;

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

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint), err)]
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
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.set_body(body);
        let resp: Response<RecordStartResult> =
            self.pipeline.send(&ctx, &mut request).await?.into();
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
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Record/Stop")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_header(RECORDING_ID, recording_id.to_string());
        request.set_body(body);
        self.pipeline.send(&ctx, &mut request).await?;
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
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Playback/Start")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.add_optional_header(&options.recording_id);
        request.set_body(body);
        let resp: Response<PlaybackStartResult> =
            self.pipeline.send(&ctx, &mut request).await?.into();
        let recording_id = resp.headers().get_str(&RECORDING_ID)?.to_string();
        let mut result: PlaybackStartResult = resp.into_body().await?;
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
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Playback/Stop")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_header(RECORDING_ID, recording_id.to_string());
        self.pipeline.send(&ctx, &mut request).await?;
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
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/SetMatcher")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_headers(&matcher)?;
        request.add_optional_header(&options.recording_id);
        let body: Bytes = matcher.try_into()?;
        request.set_body(body);
        self.pipeline.send(&ctx, &mut request).await?;
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
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/AddSanitizer")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_headers(&sanitizer)?;
        request.add_optional_header(&options.recording_id);
        self.pipeline.send(&ctx, &mut request).await?;
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
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/RemoveSanitizers")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.add_optional_header(&options.recording_id);
        request.set_body(body);
        self.pipeline
            .send(&ctx, &mut request)
            .await?
            .into_body()
            .json()
            .await
    }

    #[tracing::instrument(level = "trace", skip_all, fields(endpoint = %self.endpoint, recording_id), err)]
    pub async fn reset(&self, options: Option<ClientResetOptions<'_>>) -> Result<()> {
        let options = options.unwrap_or_default();
        Span::current().record(
            stringify!(recording_id),
            options.recording_id.map(AsRef::as_ref),
        );
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("/Admin/Reset")?;
        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.add_optional_header(&options.recording_id);
        self.pipeline.send(&ctx, &mut request).await?;
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
