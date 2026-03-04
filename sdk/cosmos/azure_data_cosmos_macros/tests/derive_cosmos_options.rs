// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for `#[derive(CosmosOptions)]`.

use azure_data_cosmos_macros::CosmosOptions;
use std::collections::HashMap;
use std::sync::Arc;

// --- Basic three-layer option group ---

#[derive(CosmosOptions, Clone)]
#[options(layers(runtime, account, operation))]
pub struct RequestOptions {
    #[option(env = "AZURE_COSMOS_TEST_CONSISTENCY")]
    pub consistency_level: Option<String>,

    pub throughput_bucket: Option<usize>,

    #[option(merge = "extend")]
    pub custom_headers: Option<HashMap<String, String>>,
}

#[test]
fn shadow_resolution_order() {
    // Operation wins over account, runtime, env.
    let env = Arc::new(RequestOptions {
        consistency_level: Some("Eventual".to_string()),
        throughput_bucket: Some(1),
        custom_headers: None,
    });
    let runtime = Arc::new(RequestOptions {
        consistency_level: Some("Session".to_string()),
        throughput_bucket: Some(2),
        custom_headers: None,
    });
    let account = Arc::new(RequestOptions {
        consistency_level: Some("Strong".to_string()),
        throughput_bucket: None,
        custom_headers: None,
    });
    let operation = RequestOptions {
        consistency_level: None,
        throughput_bucket: Some(5),
        custom_headers: None,
    };

    let view = RequestOptionsView::new(env, runtime, account, operation);

    // consistency_level: operation is None, so account wins ("Strong")
    assert_eq!(view.consistency_level(), Some(&"Strong".to_string()));

    // throughput_bucket: operation has Some(5), wins
    assert_eq!(view.throughput_bucket(), Some(&5));
}

#[test]
fn shadow_falls_through_to_env() {
    let env = Arc::new(RequestOptions {
        consistency_level: Some("Eventual".to_string()),
        throughput_bucket: None,
        custom_headers: None,
    });
    let runtime = Arc::new(RequestOptions::default());
    let account = Arc::new(RequestOptions::default());
    let operation = RequestOptions::default();

    let view = RequestOptionsView::new(env, runtime, account, operation);

    assert_eq!(view.consistency_level(), Some(&"Eventual".to_string()));
    assert_eq!(view.throughput_bucket(), None);
}

#[test]
fn merge_combines_all_layers() {
    let env = Arc::new(RequestOptions {
        consistency_level: None,
        throughput_bucket: None,
        custom_headers: Some(HashMap::from([(
            "env-key".to_string(),
            "env-val".to_string(),
        )])),
    });
    let runtime = Arc::new(RequestOptions {
        consistency_level: None,
        throughput_bucket: None,
        custom_headers: Some(HashMap::from([(
            "runtime-key".to_string(),
            "runtime-val".to_string(),
        )])),
    });
    let account = Arc::new(RequestOptions {
        consistency_level: None,
        throughput_bucket: None,
        custom_headers: Some(HashMap::from([(
            "env-key".to_string(),
            "account-override".to_string(),
        )])),
    });
    let operation = RequestOptions {
        consistency_level: None,
        throughput_bucket: None,
        custom_headers: None,
    };

    let view = RequestOptionsView::new(env, runtime, account, operation);
    let merged = view.custom_headers();

    // env-key was overridden by account layer
    assert_eq!(merged.get("env-key"), Some(&"account-override".to_string()));
    assert_eq!(merged.get("runtime-key"), Some(&"runtime-val".to_string()));
}

#[test]
fn builder_produces_correct_struct() {
    let opts = RequestOptionsBuilder::new()
        .with_consistency_level("Session".to_string())
        .with_throughput_bucket(10)
        .build();

    assert_eq!(opts.consistency_level, Some("Session".to_string()));
    assert_eq!(opts.throughput_bucket, Some(10));
    assert!(opts.custom_headers.is_none());
}

#[test]
fn default_impl_all_none() {
    let opts = RequestOptions::default();
    assert!(opts.consistency_level.is_none());
    assert!(opts.throughput_bucket.is_none());
    assert!(opts.custom_headers.is_none());
}

#[test]
fn from_env_reads_environment() {
    let mock_env = |key: &str| -> Result<String, std::env::VarError> {
        match key {
            "AZURE_COSMOS_TEST_CONSISTENCY" => Ok("BoundedStaleness".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        }
    };

    let opts = RequestOptions::from_env_vars(mock_env);
    assert_eq!(opts.consistency_level, Some("BoundedStaleness".to_string()));
    // Fields without env attr should be None
    assert!(opts.throughput_bucket.is_none());
}

// --- Two-layer option group (no operation layer) ---

#[derive(CosmosOptions, Clone)]
#[options(layers(runtime, account))]
pub struct ConnectionOptions {
    #[option(env = "AZURE_COSMOS_TEST_TIMEOUT")]
    pub request_timeout: Option<u64>,

    #[option(nested)]
    pub pool: Option<PoolOptions>,
}

#[derive(CosmosOptions, Clone)]
#[options(layers(runtime, account))]
pub struct PoolOptions {
    #[option(env = "AZURE_COSMOS_TEST_MAX_CONNECTIONS")]
    pub max_connections: Option<usize>,

    pub idle_timeout: Option<u64>,
}

#[test]
fn two_layer_view_resolution() {
    let runtime = Arc::new(ConnectionOptions {
        request_timeout: Some(30),
        pool: None,
    });
    let account = ConnectionOptions {
        request_timeout: None,
        pool: Some(PoolOptions {
            max_connections: Some(100),
            idle_timeout: None,
        }),
    };

    let view = ConnectionOptionsView::new(
        Arc::new(ConnectionOptions::default()),
        runtime.clone(),
        account,
    );

    // request_timeout: account is None, falls to runtime (30)
    assert_eq!(view.request_timeout(), Some(&30));

    // Nested pool view
    let pool_view = view.pool();
    assert_eq!(pool_view.max_connections(), Some(&100));
    assert!(pool_view.idle_timeout().is_none());
}

#[test]
fn two_layer_with_env() {
    let mock_env = |key: &str| -> Result<String, std::env::VarError> {
        match key {
            "AZURE_COSMOS_TEST_TIMEOUT" => Ok("60".to_string()),
            "AZURE_COSMOS_TEST_MAX_CONNECTIONS" => Ok("50".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        }
    };

    let conn_env = ConnectionOptions::from_env_vars(mock_env);
    assert_eq!(conn_env.request_timeout, Some(60));

    let pool_env = PoolOptions::from_env_vars(mock_env);
    assert_eq!(pool_env.max_connections, Some(50));
}

#[test]
fn nested_resolution_delegates_correctly() {
    let runtime = Arc::new(ConnectionOptions {
        request_timeout: None,
        pool: Some(PoolOptions {
            max_connections: None,
            idle_timeout: Some(300),
        }),
    });
    let account = ConnectionOptions {
        request_timeout: None,
        pool: Some(PoolOptions {
            max_connections: Some(200),
            idle_timeout: None,
        }),
    };

    let view = ConnectionOptionsView::new(Arc::new(ConnectionOptions::default()), runtime, account);
    let pool_view = view.pool();

    // max_connections: account has Some(200), wins
    assert_eq!(pool_view.max_connections(), Some(&200));
    // idle_timeout: account is None, runtime has Some(300)
    assert_eq!(pool_view.idle_timeout(), Some(&300));
}

// --- Option group with no env fields ---

#[derive(CosmosOptions, Clone)]
#[options(layers(runtime, account))]
pub struct RegionOptions {
    pub application_region: Option<String>,
    pub preferred_regions: Option<Vec<String>>,
}

#[test]
fn no_env_group_has_no_env_field_in_view() {
    let runtime = Arc::new(RegionOptions {
        application_region: Some("West US".to_string()),
        preferred_regions: None,
    });
    let account = RegionOptions::default();

    let view = RegionOptionsView::new(runtime, account);
    assert_eq!(view.application_region(), Some(&"West US".to_string()));
}
