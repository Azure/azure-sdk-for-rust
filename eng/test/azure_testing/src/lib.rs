// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod mock_request;
mod mock_response;
mod mock_transaction;
mod player_policy;
mod recorder_policy;
mod sanitation;

use http_types::Mime;
use mock_transaction::MockTransaction;
use player_policy::MockTransportPlayerPolicy;
use recorder_policy::MockTransportRecorderPolicy;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};

use azure_core::{
    credentials::{AccessToken, TokenCredential},
    date::OffsetDateTime,
    Policy, TransportOptions,
};

pub const TESTING_MODE_KEY: &str = "AZSDK_TESTING_MODE";
pub const TESTING_MODE_REPLAY: &str = "REPLAY";
pub const TESTING_MODE_RECORD: &str = "RECORD";

pub enum TestingMode {
    Record,
    Replay,
}

pub fn testing_mode() -> TestingMode {
    let mode = std::env::var(TESTING_MODE_KEY)
        .unwrap_or(TESTING_MODE_REPLAY.to_string())
        .to_uppercase();

    match mode.as_str() {
        TESTING_MODE_RECORD => TestingMode::Record,
        _ => TestingMode::Replay,
    }
}

/// The context that describes the test being recorded or replayed.
///
/// This context is used to determine the path to the test transaction files.
#[derive(Debug, Clone)]
pub struct TestContext {
    pub package_path: String,
    pub module_under_test: String,
    pub transaction_name: String,
}

impl TestContext {
    /// Create a new [`TransactionContext`] with the provided package path, module-under-test, and transaction name.
    pub fn new(package_path: String, module_under_test: String, transaction_name: String) -> Self {
        Self {
            package_path,
            module_under_test,
            transaction_name,
        }
    }

    /// Create a new [`TransactionContext`] with the provided package path and transaction name. The 'module-under-test' will be inferred from the test module path.
    pub fn from_test_module_path(
        package_path: String,
        test_module_path: String,
        transaction_name: String,
    ) -> Self {
        let path_segments: Vec<&str> = test_module_path.split("::").collect();

        let module_under_test = match path_segments.split_last() {
            Some((&"test", rest)) => rest.last().map(|x| *x),
            Some((&"tests", rest)) => rest.last().map(|x| *x),
            Some((x, _)) => Some(*x),
            None => None,
        };
        let Some(module_under_test) = module_under_test else {
            panic!("unable to determine module path");
        };
        let module_under_test = module_under_test.to_string();

        Self {
            package_path,
            module_under_test,
            transaction_name,
        }
    }

    /// Create a new mock transport policy.
    ///
    /// Returns a reply mock policy unless the environment variable  "`TESTING_MODE`" is set to "RECORD".
    pub fn create_transport(&self) -> TransportOptions {
        let policy: Arc<dyn Policy> = match testing_mode() {
            TestingMode::Record => {
                log::warn!("mock testing framework record mode enabled");
                Arc::new(MockTransportRecorderPolicy::new(
                    self.clone(),
                    azure_core::new_http_client(),
                ))
            }
            TestingMode::Replay => {
                log::info!("mock testing framework replay mode enabled");
                Arc::new(MockTransportPlayerPolicy::new(self.clone()))
            }
        };
        TransportOptions::new_custom_policy(policy)
    }

    /// Creates a mock token credential.
    ///
    /// In record mode, the provided closure will be called to create the real token credential.
    /// When the recording is saved, the real token will be replaced by the mock token.
    ///
    /// In replay mode, the provided closure will **never** be called.
    /// Instead, a mock token will be provided.
    pub fn create_credentials(
        &self,
        record_credential: impl Fn() -> Option<Arc<dyn TokenCredential>> + Send + Sync + 'static,
    ) -> Arc<dyn TokenCredential> {
        Arc::new(MockTokenCredential::new(
            self.clone(),
            Box::new(record_credential),
        ))
    }
}

/// Create a [`TransactionContext`] for the current test module and the provided transaction name.
///
/// If the current test module is named `test` or `tests`, the default "module under test" will be the parent module.
/// For example, given `azure_data_cosmos::clients::database_client::test`, the "module under test" will be `database_client`.
/// If this isn't appropriate, use the two-parameter variant that allows you to specify the "module under test".
///
/// # Examples
///
/// Specifying just the transaction name and using the default module-under-test detection:
///
/// ```rust,no_run
/// # use azure_testing::context;
/// let context = context!("my_transaction");
/// ```
///
/// Specifying the module-under-test manually:
///
/// ```rust,no_run
/// # use azure_testing::context;
/// let context = context!("frob_client", "my_transaction");
/// ```
#[macro_export]
macro_rules! context {
    ($transaction_name: expr) => {
        $crate::TestContext::from_test_module_path(
            env!("CARGO_MANIFEST_DIR").into(),
            module_path!().into(),
            $transaction_name.into(),
        )
    };
    ($module_under_test: expr, $transaction_name: expr) => {
        $crate::TestContext::new(
            env!("CARGO_MANIFEST_DIR").into(),
            $module_under_test.into(),
            $transaction_name.into(),
        )
    };
}

/// Represents a mock token credential.
///
/// In record mode, the provided closure will be called to create the real token credential.
/// When the recording is saved, the real token will be replaced by the mock token.
///
/// In replay mode, the provided closure will never be called.
/// Instead, a mock token will be provided.
pub struct MockTokenCredential {
    tx_context: TestContext,
    record_credential_fn: Box<dyn Fn() -> Option<Arc<dyn TokenCredential>> + Send + Sync>,
}

/// Gets the test endpoint URL.
///
/// If running in record mode, this expects 'AZSDK_RECORD_TARGET_URL' to be set and returns it as the endpoint URL.
/// When running in replay mode, this returns a dummy URL.
///
/// In a test, if a single endpoint URL is needed, this can be used to get the endpoint URL.
/// If your test requires multiple endpoint URLs, you'll have to configure them manually.
pub fn test_endpoint_url() -> String {
    match testing_mode() {
        TestingMode::Record => std::env::var("AZSDK_RECORD_TARGET_URL")
            .expect("AZSDK_RECORD_TARGET_URL must be set when in record mode."),
        TestingMode::Replay => "https://example.com/".to_owned(), // Replay mode expects URLs to be valid, but doesn't actually make requests.
    }
}

impl std::fmt::Debug for MockTokenCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We have to implement Debug as part of implementing TokenCredential, but we don't want to expose the real token.
        f.debug_struct("MockTokenCredential")
            .field("tx_context", &self.tx_context)
            .finish()
    }
}

impl MockTokenCredential {
    pub fn new(
        tx_context: TestContext,
        record_credential: Box<dyn Fn() -> Option<Arc<dyn TokenCredential>> + Send + Sync>,
    ) -> Self {
        Self {
            tx_context,
            record_credential_fn: record_credential,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for MockTokenCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        if let TestingMode::Record = testing_mode() {
            let cred = (self.record_credential_fn)();
            if let Some(record_credential) = cred {
                return record_credential.get_token(scopes).await;
            }
        }
        Ok(AccessToken::new(
            format!(
                "mock_token::{}::{}",
                self.tx_context.module_under_test, self.tx_context.transaction_name,
            ),
            OffsetDateTime::now_utc().saturating_add(time::Duration::minutes(5)),
        ))
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        Ok(())
    }
}

fn is_json_content_type(content_type: Option<&str>) -> bool {
    let Some(content_type) = content_type else {
        return false;
    };

    let Ok(content_type) = Mime::from_str(content_type) else {
        return false;
    };

    // Check if the content type is JSON or a JSON-encoded other type (e.g. application/vnd.api+json)
    content_type.subtype() == "json" || content_type.subtype().ends_with("+json")
}

fn is_utf8_safe_content_type(content_type: Option<&str>) -> bool {
    is_json_content_type(content_type)
}

#[derive(Serialize, Deserialize)]
pub(crate) enum BodyEncoding {
    Empty,
    Utf8,
    Base64,
    Json,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializedBody {
    encoding: BodyEncoding,

    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    #[serde(default)]
    content: serde_json::Value,
}
