// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Telemetry options.
#[derive(Clone, Debug, Default)]
pub struct TelemetryOptions {
    /// Optional application ID to telemetry.
    pub application_id: Option<String>,
}
