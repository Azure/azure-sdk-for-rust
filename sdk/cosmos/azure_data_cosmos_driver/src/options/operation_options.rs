// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation options that participate in runtime/account/operation resolution.

use azure_data_cosmos_macros::CosmosOptions;

use crate::options::{ContentResponseOnWrite, ExcludedRegions, ReadConsistencyStrategy};

/// Operation options that apply to individual service requests.
///
/// These options follow a hierarchy where operation-level settings override
/// account-level, which override runtime-level, which override environment defaults.
#[derive(CosmosOptions, Clone, Debug)]
#[options(layers(runtime, account, operation))]
#[non_exhaustive]
pub struct OperationOptions {
    /// Read consistency strategy for read operations.
    #[option(env = "AZURE_COSMOS_READ_CONSISTENCY_STRATEGY")]
    pub read_consistency_strategy: Option<ReadConsistencyStrategy>,

    /// Regions to exclude from routing.
    pub excluded_regions: Option<ExcludedRegions>,

    /// Content response on write setting.
    #[option(env = "AZURE_COSMOS_CONTENT_RESPONSE_ON_WRITE")]
    pub content_response_on_write: Option<ContentResponseOnWrite>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_operation_options() {
        let options = OperationOptions::default();
        assert!(options.read_consistency_strategy.is_none());
        assert!(options.excluded_regions.is_none());
        assert!(options.content_response_on_write.is_none());
    }

    #[test]
    fn builder_creates_options() {
        let options = OperationOptionsBuilder::new()
            .with_content_response_on_write(ContentResponseOnWrite::Disabled)
            .with_read_consistency_strategy(ReadConsistencyStrategy::Session)
            .build();

        assert_eq!(
            options.content_response_on_write,
            Some(ContentResponseOnWrite::Disabled)
        );
        assert_eq!(
            options.read_consistency_strategy,
            Some(ReadConsistencyStrategy::Session)
        );
    }

    #[test]
    fn view_resolves_across_layers() {
        use std::sync::Arc;

        let env = Arc::new(OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::Eventual),
            excluded_regions: None,
            content_response_on_write: Some(ContentResponseOnWrite::Enabled),
        });

        let runtime = Arc::new(OperationOptions {
            read_consistency_strategy: None,
            excluded_regions: None,
            content_response_on_write: None,
        });

        let account = Arc::new(OperationOptions {
            read_consistency_strategy: None,
            excluded_regions: None,
            content_response_on_write: Some(ContentResponseOnWrite::Disabled),
        });

        let operation = OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::Session),
            excluded_regions: None,
            content_response_on_write: None,
        };

        let view =
            OperationOptionsView::new(Some(env), Some(runtime), Some(account), Some(&operation));

        // Operation overrides env
        assert_eq!(
            view.read_consistency_strategy(),
            Some(&ReadConsistencyStrategy::Session)
        );
        // Account overrides env
        assert_eq!(
            view.content_response_on_write(),
            Some(&ContentResponseOnWrite::Disabled)
        );
        // Not set anywhere
        assert!(view.excluded_regions().is_none());
    }

    #[test]
    fn view_falls_through_to_env() {
        use std::sync::Arc;

        let env = Arc::new(OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::Eventual),
            excluded_regions: None,
            content_response_on_write: None,
        });

        let view = OperationOptionsView::new(Some(env), None, None, None);

        assert_eq!(
            view.read_consistency_strategy(),
            Some(&ReadConsistencyStrategy::Eventual)
        );
        assert!(view.content_response_on_write().is_none());
    }

    #[test]
    fn from_env_vars_parses_known_vars() {
        let options = OperationOptions::from_env_vars(|key| match key {
            "AZURE_COSMOS_READ_CONSISTENCY_STRATEGY" => Ok("Session".to_string()),
            "AZURE_COSMOS_CONTENT_RESPONSE_ON_WRITE" => Ok("true".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        });

        assert_eq!(
            options.read_consistency_strategy,
            Some(ReadConsistencyStrategy::Session)
        );
        assert_eq!(
            options.content_response_on_write,
            Some(ContentResponseOnWrite::Enabled)
        );
        // Fields without env annotation remain None
        assert!(options.excluded_regions.is_none());
    }

    #[test]
    fn from_env_vars_returns_none_for_missing_vars() {
        let options = OperationOptions::from_env_vars(|_| Err(std::env::VarError::NotPresent));

        assert!(options.read_consistency_strategy.is_none());
        assert!(options.content_response_on_write.is_none());
        assert!(options.excluded_regions.is_none());
    }
}
