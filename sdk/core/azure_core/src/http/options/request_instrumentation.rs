// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::Arc;

/// Policy options to enable distributed tracing.
#[derive(Clone, Debug, Default)]
pub struct RequestInstrumentationOptions {
    /// Set the tracer provider for distributed tracing.
    pub tracer_provider: Option<Arc<dyn crate::tracing::TracerProvider>>,
}
