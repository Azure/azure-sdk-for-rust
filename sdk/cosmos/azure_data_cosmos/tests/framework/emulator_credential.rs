// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore enableaadauthentication appidacr

//! A [`TokenCredential`] that mints the fake JWT the Azure Cosmos DB emulator
//! accepts when started with `/enableaadauthentication`.
//!
//! The emulator does not validate tokens against Entra ID. Instead it accepts a
//! self-signed JWT whose signature segment is the emulator's well-known master
//! key (encoded as base64url, no padding). This mirrors the
//! `CosmosEmulatorCredential` used by the Python SDK's AAD emulator tests.
//!
//! This credential is for **tests only**. Never use it against a real account.

use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
use azure_core::time::OffsetDateTime;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// The well-known master key the Cosmos DB emulator ships with. The emulator
/// validates the JWT signature segment against this exact string.
pub const EMULATOR_MASTER_KEY: &str =
    "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";

/// Records how a [`CosmosEmulatorCredential`] was exercised so tests can assert
/// the AAD path (and not key auth) was actually used.
#[derive(Clone, Default)]
pub struct CredentialRecorder {
    call_count: Arc<AtomicUsize>,
    scopes: Arc<Mutex<Vec<String>>>,
}

impl CredentialRecorder {
    /// Number of times `get_token` was called.
    pub fn call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }

    /// All scopes that were requested across every `get_token` call.
    pub fn requested_scopes(&self) -> Vec<String> {
        self.scopes.lock().unwrap().clone()
    }

    /// Returns `true` if any `get_token` call requested `scope`.
    pub fn requested_scope(&self, scope: &str) -> bool {
        self.scopes.lock().unwrap().iter().any(|s| s == scope)
    }
}

/// A [`TokenCredential`] that produces the emulator's fake AAD token.
#[derive(Clone)]
pub struct CosmosEmulatorCredential {
    master_key: String,
    call_count: Arc<AtomicUsize>,
    scopes: Arc<Mutex<Vec<String>>>,
}

impl std::fmt::Debug for CosmosEmulatorCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Redact `master_key`: it is signing-key material and must never land in
        // a `{:?}` test log, even though today it is the emulator's well-known key.
        f.debug_struct("CosmosEmulatorCredential")
            .finish_non_exhaustive()
    }
}

impl std::fmt::Debug for CredentialRecorder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CredentialRecorder")
            .field("call_count", &self.call_count())
            .finish_non_exhaustive()
    }
}

impl CosmosEmulatorCredential {
    /// Creates a credential using the emulator's well-known master key.
    pub fn new() -> Self {
        Self::with_master_key(EMULATOR_MASTER_KEY)
    }

    /// Creates a credential signing tokens with a specific master key string.
    ///
    /// The `master_key` must be the same key the emulator was started with; the
    /// emulator validates the JWT signature segment against it byte-for-byte.
    pub fn with_master_key(master_key: impl Into<String>) -> Self {
        Self {
            master_key: master_key.into(),
            call_count: Arc::new(AtomicUsize::new(0)),
            scopes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns a [`CredentialRecorder`] that observes how this credential is used.
    pub fn recorder(&self) -> CredentialRecorder {
        CredentialRecorder {
            call_count: self.call_count.clone(),
            scopes: self.scopes.clone(),
        }
    }

    /// Builds the emulator fake JWT: `b64url(header).b64url(claims).b64url(key)`.
    fn build_token(&self, expires_on: OffsetDateTime) -> String {
        // Header advertises the emulator's well-known signing key identifier.
        let header = r#"{"typ":"JWT","alg":"RS256","x5t":"CosmosEmulatorPrimaryMaster","kid":"CosmosEmulatorPrimaryMaster"}"#;

        let now = OffsetDateTime::now_utc().unix_timestamp();
        // Allow for some clock skew between the test host and the emulator.
        let not_before = now - 300;
        let issued_at = now - 300;
        let expiry = expires_on.unix_timestamp();

        let claims = format!(
            concat!(
                "{{",
                r#""aud":"https://localhost.localhost","#,
                r#""iss":"https://sts.fake-issuer.net/7b1999a1-dfd7-440e-8204-00170979b984","#,
                r#""iat":{iat},"nbf":{nbf},"exp":{exp},"#,
                r#""aio":"","appid":"localhost","appidacr":"1","#,
                r#""idp":"https://localhost.localhost","oid":"96313034-4739-43cb-93cd-74193adbe5b6","#,
                r#""rh":"","sub":"localhost","#,
                r#""tid":"EmulatorFederation","#,
                r#""unique_name":"localhost","uti":"","#,
                r#""ver":"1.0","#,
                r#""scp":"user_impersonation","#,
                r#""groups":["7ce1d3a1-6df6-4e88-bb0d-1c5e1c6c2b21","e99e9b3e-9c4f-4e5e-9e0a-2b9a4b2c3d4e","b2b3b4b5-c6c7-48d9-9e0a-2b9a4b2c3d4e","c3c4c5c6-d7d8-49e0-9f0b-3c0b5c3d4e5f","d4d5d6d7-e8e9-4af1-a01c-4d1c6d4e5f60"]"#,
                "}}",
            ),
            iat = issued_at,
            nbf = not_before,
            exp = expiry,
        );

        let header_segment = URL_SAFE_NO_PAD.encode(header.as_bytes());
        let claims_segment = URL_SAFE_NO_PAD.encode(claims.as_bytes());
        // The emulator validates this segment against its master key string bytes.
        let signature_segment = URL_SAFE_NO_PAD.encode(self.master_key.as_bytes());

        format!("{header_segment}.{claims_segment}.{signature_segment}")
    }
}

impl Default for CosmosEmulatorCredential {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for CosmosEmulatorCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        _options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        {
            let mut recorded = self.scopes.lock().unwrap();
            recorded.extend(scopes.iter().map(|s| s.to_string()));
        }

        let expires_on = OffsetDateTime::now_utc() + azure_core::time::Duration::hours(2);
        let token = self.build_token(expires_on);
        Ok(AccessToken::new(token, expires_on))
    }
}
