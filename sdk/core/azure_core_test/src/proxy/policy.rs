// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    proxy::{RecordingId, RECORDING_MODE, RECORDING_UPSTREAM_BASE_URI},
    Skip,
};
use async_trait::async_trait;
use azure_core::{
    error::ErrorKind,
    http::{
        headers::{AsHeaders, HeaderName, HeaderValue},
        policies::{Policy, PolicyResult},
        request::Request,
        Context, Url,
    },
    test::TestMode,
};
use std::{
    convert::Infallible,
    sync::{Arc, RwLock},
};
use tracing::Instrument;
use url::Origin;

#[derive(Debug, Default)]
pub struct RecordingPolicy {
    pub test_mode: TestMode,
    pub host: Option<Url>,
    pub recording_id: Option<RecordingId>,
    pub options: RwLock<RecordingOptions>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for RecordingPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let span = tracing::trace_span!("request", mode = ?self.test_mode);

        // Replace the upstream host with the test-proxy host, which will make and record the upstream call.
        let mut origin = None;
        if let Some(host) = &self.host {
            let url = request.url_mut();
            origin = Some(url.origin());

            url.set_scheme(host.scheme()).map_err(|_| {
                azure_core::Error::with_message(
                    ErrorKind::Other,
                    "failed to set recording url scheme",
                )
            })?;
            url.set_host(host.host_str()).map_err(|_| {
                azure_core::Error::with_message(
                    ErrorKind::Other,
                    "failed to set recording url host",
                )
            })?;
            url.set_port(host.port()).map_err(|_| {
                azure_core::Error::with_message(
                    ErrorKind::Other,
                    "failed to set recording url port",
                )
            })?;
        }

        if let Some(origin) = &origin {
            request.insert_header(RECORDING_UPSTREAM_BASE_URI, origin.ascii_serialization());
        }

        request.insert_headers(&self.recording_id)?;
        request.insert_header(
            RECORDING_MODE,
            HeaderValue::from_static(self.test_mode.into()),
        );
        if let Ok(options) = self.options.read() {
            request.insert_headers(&*options)?;
        }

        async move {
            let resp = next[0].send(ctx, request, &next[1..]).await?;

            // Restore the upstream host to support pageables and pollers that may need the original URL.
            if let Some(Origin::Tuple(scheme, host, port)) = origin {
                let url = request.url_mut();

                url.set_scheme(scheme.as_ref()).map_err(|_| {
                    azure_core::Error::with_message(
                        ErrorKind::Other,
                        "failed to set recording url scheme",
                    )
                })?;
                url.set_host(Some(host.to_string().as_ref())).map_err(|_| {
                    azure_core::Error::with_message(
                        ErrorKind::Other,
                        "failed to set recording url host",
                    )
                })?;
                url.set_port(Some(port)).map_err(|_| {
                    azure_core::Error::with_message(
                        ErrorKind::Other,
                        "failed to set recording url port",
                    )
                })?;
            }

            Ok(resp)
        }
        .instrument(span)
        .await
    }
}

#[derive(Debug, Default)]
pub struct RecordingOptions {
    pub skip: Option<Skip>,
}

impl AsHeaders for RecordingOptions {
    type Error = Infallible;
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        self.skip.as_headers()
    }
}
