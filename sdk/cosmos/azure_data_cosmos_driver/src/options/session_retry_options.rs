// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Session retry options for 404/1002 error handling.

use azure_data_cosmos_macros::CosmosOptions;

/// Options controlling retry behavior for session-consistency 404/1002 errors.
///
/// These options follow a hierarchy where account-level settings override
/// runtime-level defaults.
#[derive(CosmosOptions, Clone, Debug)]
#[options(layers(runtime, account))]
#[non_exhaustive]
pub struct SessionRetryOptions {
    /// Maximum number of retries within the local region for session not found errors.
    #[option(env = "AZURE_COSMOS_MAX_SESSION_RETRY_COUNT")]
    pub max_session_retry_count: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_session_retry_options() {
        let options = SessionRetryOptions::default();
        assert!(options.max_session_retry_count.is_none());
    }

    #[test]
    fn builder_creates_options() {
        let options = SessionRetryOptionsBuilder::new()
            .with_max_session_retry_count(5)
            .build();

        assert_eq!(options.max_session_retry_count, Some(5));
    }

    #[test]
    fn view_resolves_across_layers() {
        use std::sync::Arc;

        let env = Arc::new(SessionRetryOptions {
            max_session_retry_count: Some(2),
        });

        let runtime = Arc::new(SessionRetryOptions {
            max_session_retry_count: None,
        });

        let account = SessionRetryOptions {
            max_session_retry_count: Some(5),
        };

        let view = SessionRetryOptionsView::new(Some(env), Some(runtime), Some(&account));

        // Account overrides env (runtime is None so falls through)
        assert_eq!(view.max_session_retry_count(), Some(&5));
    }

    #[test]
    fn view_falls_through_to_env() {
        use std::sync::Arc;

        let env = Arc::new(SessionRetryOptions {
            max_session_retry_count: Some(3),
        });

        let view = SessionRetryOptionsView::new(Some(env), None, None);

        assert_eq!(view.max_session_retry_count(), Some(&3));
    }

    #[test]
    fn from_env_vars_parses_known_vars() {
        let options = SessionRetryOptions::from_env_vars(|key| match key {
            "AZURE_COSMOS_MAX_SESSION_RETRY_COUNT" => Ok("3".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        });

        assert_eq!(options.max_session_retry_count, Some(3));
    }

    #[test]
    fn from_env_vars_returns_none_for_missing_vars() {
        let options = SessionRetryOptions::from_env_vars(|_| Err(std::env::VarError::NotPresent));

        assert!(options.max_session_retry_count.is_none());
    }
}
