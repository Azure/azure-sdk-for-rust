// Licensed under the MIT License.

#![cfg(feature = "key_auth")]

use std::sync::{Arc, Mutex};
use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
use azure_core::time::{Duration, OffsetDateTime};
use azure_core_test::{recorded, TestContext};
use serde_json::json;

mod framework;
use framework::TestAccount;
use azure_data_cosmos::{CosmosClient, models::ContainerProperties};

//
// ========== Helpers for capturing scopes & simulating failures ==========
//

#[derive(Debug, Clone)]
struct CapturedScopes(Arc<Mutex<Vec<String>>>);

impl CapturedScopes {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
    fn push(&self, scope: &str) {
        self.0.lock().unwrap().push(scope.to_string());
    }
    fn take(&self) -> Vec<String> {
        std::mem::take(&mut *self.0.lock().unwrap())
    }
}

#[derive(Debug)]
struct RecordingCredential {
    tag: &'static str,
    captured: CapturedScopes,
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for RecordingCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        _opts: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let scope = scopes.join(",");
        self.captured.push(&scope);
        Ok(AccessToken::new(
            format!("{}_token_for_{}", self.tag, scope),
            OffsetDateTime::now_utc().saturating_add(Duration::minutes(5)),
        ))
    }
}

#[derive(Debug)]
struct AlwaysFailCredential {
    captured: CapturedScopes,
    message: &'static str,
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AlwaysFailCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        _opts: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let scope = scopes.join(",");
        self.captured.push(&scope);
        Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            self.message,
        ))
    }
}

/// Fails once with AADSTS500011 on account scope, then succeeds
#[derive(Debug)]
struct FailOnceThenSucceedCredential {
    captured: CapturedScopes,
    first_call_done: Arc<Mutex<bool>>,
    account_scope_prefix: String,
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for FailOnceThenSucceedCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        _opts: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let scope = scopes.join(",");
        self.captured.push(&scope);

        let mut done = self.first_call_done.lock().unwrap();
        if !*done && scope.starts_with(&self.account_scope_prefix) {
            *done = true;
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "AADSTS500011: Simulated error for fallback",
            ));
        }

        Ok(AccessToken::new(
            format!("ok_token_for_{}", scope),
            OffsetDateTime::now_utc().saturating_add(Duration::minutes(5)),
        ))
    }
}

// Env override guard

struct TestEnvGuard {
    key: &'static str,
    original: Option<String>,
}
impl TestEnvGuard {
    fn set(key: &'static str, val: &str) -> Self {
        let original = std::env::var(key).ok();
        std::env::set_var(key, val);
        Self { key, original }
    }
}
impl Drop for TestEnvGuard {
    fn drop(&mut self) {
        if let Some(ref v) = self.original {
            std::env::set_var(self.key, v);
        } else {
            std::env::remove_var(self.key);
        }
    }
}


async fn create_db_container_and_item(
    client: &CosmosClient,
    db_id: &str,
    container_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    let _ = client.create_database(db_id, None).await?;

    let db_client = client.database_client(db_id);

    db_client
        .create_container(
            ContainerProperties {
                id: container_id.to_string().into(),
                partition_key: "/pk".into(),
                ..Default::default()
            },
            None,
        )
        .await?;

    let cont = db_client.container_client(container_id);
    let doc = json!({"id":"Item_1","pk":"pk"});
    cont.create_item("pk", &doc, None).await?;
    Ok(())
}

// AAD Tests

const ENV_SCOPE_OVERRIDE: &str = "AZURE_COSMOS_AAD_SCOPE_OVERRIDE";
const PUBLIC_COSMOS_SCOPE: &str = "https://cosmos.azure.com/.default";

#[recorded::test]
async fn aad_override_scope_no_fallback(context: TestContext) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = TestEnvGuard::set(ENV_SCOPE_OVERRIDE, "https://cosmos.azure.com/.default");

    let captured = CapturedScopes::new();
    let cred = Arc::new(RecordingCredential { tag: "override", captured: captured.clone() });

    let account = TestAccount::from_env(context, None).await?;
    let client = account.connect_with_token(cred)?;

    create_db_container_and_item(&client, "AAD_Override_DB", "AAD_Override_Cont").await?;

    let scopes = captured.take();
    assert!(scopes.iter().all(|s| s == "https://cosmos.azure.com/.default"));
    Ok(())
}

#[recorded::test]
async fn aad_override_scope_auth_error_no_fallback(context: TestContext) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = TestEnvGuard::set(ENV_SCOPE_OVERRIDE, "https://my.custom.scope/.default");

    let captured = CapturedScopes::new();
    let cred = Arc::new(AlwaysFailCredential { captured: captured.clone(), message: "fail" });

    let account = TestAccount::from_env(context, None).await?;
    let client = account.connect_with_token(cred)?;

    let result = create_db_container_and_item(&client, "AAD_OverrideFail_DB", "AAD_OverrideFail_Cont").await;
    assert!(result.is_err());

    let scopes = captured.take();
    assert_eq!(scopes, vec!["https://my.custom.scope/.default"]);
    Ok(())
}

#[recorded::test]
async fn aad_account_scope_only(context: TestContext) -> Result<(), Box<dyn std::error::Error>> {
    // Empty override -> use account scope (no fallback unless error)
    let _guard = TestEnvGuard::set(ENV_SCOPE_OVERRIDE, "");

    let captured = CapturedScopes::new();
    let cred = Arc::new(RecordingCredential { tag: "account", captured: captured.clone() });

    let account = TestAccount::from_env(context, None).await?;
    let client = account.connect_with_token(cred)?;

    create_db_container_and_item(&client, "AAD_Account_DB", "AAD_Account_Cont").await?;

    let scopes = captured.take();
    assert!(!scopes.is_empty());
    Ok(())
}

#[recorded::test]
async fn aad_account_scope_fallback_on_error(context: TestContext) -> Result<(), Box<dyn std::error::Error>> {
    // Empty override -> use account/host scope, and if AADSTS500011 then fallback to public scope
    let _guard = TestEnvGuard::set(ENV_SCOPE_OVERRIDE, "");

    let account = TestAccount::from_env(context, None).await?;

    let captured_probe = CapturedScopes::new();
    let probe_cred = Arc::new(RecordingCredential { tag: "probe", captured: captured_probe.clone() });
    let probe_client = account.connect_with_token(probe_cred)?;
    let _ = probe_client.create_database("AAD_Fallback_Probe_DB", None).await;

    let scopes_seen = captured_probe.take();
    let account_scope = scopes_seen.into_iter().find(|s| s.ends_with("/.default")).unwrap();

    //Run with FailOnceThenSucceedCredential to trigger fallback
    let captured = CapturedScopes::new();
    let cred = Arc::new(FailOnceThenSucceedCredential {
        captured: captured.clone(),
        first_call_done: Arc::new(Mutex::new(false)),
        account_scope_prefix: account_scope.clone(),
    });

    let client = account.connect_with_token(cred)?;
    create_db_container_and_item(&client, "AAD_Fallback_DB", "AAD_Fallback_Cont").await?;

    let scopes = captured.take();
    assert!(scopes.contains(&account_scope));
    assert!(scopes.contains(&PUBLIC_COSMOS_SCOPE.to_string()));
    Ok(())
}