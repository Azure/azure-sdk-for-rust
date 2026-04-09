// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::Arc;

/// Policy options to enable distributed tracing.
///
/// # Notes
///
/// [`LoggingOptions::additional_allowed_query_params`](super::LoggingOptions::additional_allowed_query_params)
/// is used to sanitize query parameters in traced URLs as well.
/// Query parameters not in the default or additional allow list will have their values
/// replaced with `REDACTED` in the `url.full` span attribute.
#[derive(Clone, Debug, Default)]
pub struct InstrumentationOptions {
    /// Set the tracer provider for distributed tracing.
    pub tracer_provider: Option<Arc<dyn crate::tracing::TracerProvider>>,
}
