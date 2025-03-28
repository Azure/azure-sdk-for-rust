// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{request::options::ClientRequestId, ClientMethodOptions};
use std::borrow::Cow;

/// Telemetry options.
#[derive(Clone, Debug, Default)]
pub struct TelemetryOptions {
    /// Optional application ID to telemetry.
    pub application_id: Option<String>,
}

/// Extension methods to set telemetry options on [`ClientMethodOptions`].
pub trait ClientMethodTelemetryOptionsExt: crate::private::Sealed {
    /// Set the client request ID.
    ///
    /// Most often this is the `x-ms-client-request-id` header but may be different for some clients.
    fn set_client_request_id<S>(&mut self, value: S)
    where
        S: Into<Cow<'static, str>>;
}

impl ClientMethodTelemetryOptionsExt for ClientMethodOptions<'_> {
    fn set_client_request_id<S>(&mut self, value: S)
    where
        S: Into<Cow<'static, str>>,
    {
        let request_id = ClientRequestId::new(value);
        self.context.insert(request_id);
    }
}
