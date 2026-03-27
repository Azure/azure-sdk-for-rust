// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Retry behavior options.

use azure_data_cosmos_macros::CosmosOptions;

use crate::options::{SessionRetryOptions, SessionRetryOptionsView};

/// Options controlling retry behavior.
///
/// These options follow a hierarchy where account-level settings override
/// runtime-level defaults. Not available at the operation layer because
/// retry policy is an infrastructure concern.
#[derive(CosmosOptions, Clone, Debug)]
#[options(layers(runtime, account))]
#[non_exhaustive]
pub struct RetryOptions {
    /// Session retry options for 404/1002 error handling.
    #[option(nested)]
    pub session_retry: Option<SessionRetryOptions>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::SessionRetryOptionsBuilder;
    use std::sync::Arc;

    #[test]
    fn default_all_none() {
        let options = RetryOptions::default();
        assert!(options.session_retry.is_none());
    }

    #[test]
    fn builder_with_nested_session_retry() {
        let session = SessionRetryOptionsBuilder::new()
            .with_max_session_retry_count(5)
            .build();

        let options = RetryOptionsBuilder::new()
            .with_session_retry(session)
            .build();

        let inner = options.session_retry.as_ref().expect("should be Some");
        assert_eq!(inner.max_session_retry_count, Some(5));
    }

    #[test]
    fn view_resolves_nested_across_layers() {
        let runtime = Arc::new(RetryOptions {
            session_retry: Some(SessionRetryOptions {
                max_session_retry_count: Some(2),
            }),
        });

        let account = RetryOptions {
            session_retry: Some(SessionRetryOptions {
                max_session_retry_count: Some(10),
            }),
        };

        let view = RetryOptionsView::new(None, Some(runtime), Some(&account));
        let session_view = view.session_retry();

        // Account layer wins over runtime.
        assert_eq!(session_view.max_session_retry_count(), Some(&10));
    }

    #[test]
    fn view_falls_through_when_account_absent() {
        let runtime = Arc::new(RetryOptions {
            session_retry: Some(SessionRetryOptions {
                max_session_retry_count: Some(3),
            }),
        });

        let view = RetryOptionsView::new(None, Some(runtime), None);
        let session_view = view.session_retry();

        assert_eq!(session_view.max_session_retry_count(), Some(&3));
    }

    #[test]
    fn view_returns_none_when_all_layers_empty() {
        let view = RetryOptionsView::new(None, None, None);
        let session_view = view.session_retry();

        assert_eq!(session_view.max_session_retry_count(), None);
    }
}
