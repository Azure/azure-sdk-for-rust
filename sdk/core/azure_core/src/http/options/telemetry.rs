// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Telemetry options.
#[derive(Clone, Debug, Default)]
pub struct TelemetryOptions {
    /// Set the application ID in the `User-Agent` header that can be telemetered
    pub application_id: Option<String>,
}
