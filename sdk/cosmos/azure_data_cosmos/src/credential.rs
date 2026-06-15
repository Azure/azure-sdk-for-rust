// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Credential types for authenticating with Azure Cosmos DB.

use azure_core::credentials::TokenCredential;
use azure_core::fmt::SafeDebug;
use std::sync::Arc;

#[cfg(feature = "key_auth")]
use azure_core::credentials::Secret;

/// Authentication credential for connecting to a Cosmos DB account.
///
/// Either key-based authentication using a master key, or token-based
/// authentication using an Azure credential (e.g., managed identity, service principal).
///
/// # Examples
///
/// Using Entra ID (Azure AD) authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::CosmosCredential;
/// use std::sync::Arc;
///
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let credential: CosmosCredential = credential.into();
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::CosmosCredential;
/// use azure_core::credentials::Secret;
///
/// let credential: CosmosCredential = Secret::from("my_account_key").into();
/// ```
#[derive(Clone, SafeDebug)]
#[non_exhaustive]
pub enum CosmosCredential {
    /// Entra ID (Azure AD) token credential.
    TokenCredential(Arc<dyn TokenCredential>),
    /// Primary or secondary account key.
    #[cfg(feature = "key_auth")]
    MasterKey(Secret),
}

impl From<Arc<dyn TokenCredential>> for CosmosCredential {
    fn from(credential: Arc<dyn TokenCredential>) -> Self {
        Self::TokenCredential(credential)
    }
}

#[cfg(feature = "key_auth")]
impl From<Secret> for CosmosCredential {
    fn from(key: Secret) -> Self {
        Self::MasterKey(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Locks down the exact `Debug` rendering of every [`CosmosCredential`]
    /// variant. Because we derive [`SafeDebug`] (without `#[safe(true)]`),
    /// the inner credential payload — the `Arc<dyn TokenCredential>` for
    /// `TokenCredential`, and the `Secret` for `MasterKey` — is replaced
    /// with a redaction marker (`..`) at format time, so neither token-
    /// fetching state nor master-key bytes can leak through `{:?}`.
    ///
    /// The expected rendering shape (`Variant(..)`) is the `SafeDebug`
    /// behavior on rustc ≥ 1.82; older toolchains emit just `Variant`.
    /// Both shapes redact the payload — the test accepts either so it
    /// keeps passing as the MSRV moves.
    #[derive(Debug)]
    struct TestCredential;

    #[async_trait::async_trait]
    impl azure_core::credentials::TokenCredential for TestCredential {
        async fn get_token(
            &self,
            _scopes: &[&str],
            _options: Option<azure_core::credentials::TokenRequestOptions<'_>>,
        ) -> azure_core::Result<azure_core::credentials::AccessToken> {
            unimplemented!("test credential is for Debug-formatting tests only")
        }
    }

    fn assert_safe_debug_render(rendered: &str, variant: &str) {
        let payload_redacted = format!("{variant}(..)");
        let payload_elided = variant.to_string();
        assert!(
            rendered == payload_redacted || rendered == payload_elided,
            "expected {payload_redacted:?} or {payload_elided:?}, got {rendered:?}"
        );
    }

    #[test]
    fn debug_token_credential_redacts_inner_credential() {
        let credential: Arc<dyn TokenCredential> = Arc::new(TestCredential);
        let cosmos = CosmosCredential::TokenCredential(credential);
        assert_safe_debug_render(&format!("{cosmos:?}"), "TokenCredential");
    }

    #[cfg(feature = "key_auth")]
    #[test]
    fn debug_master_key_redacts_secret() {
        let cosmos = CosmosCredential::MasterKey(Secret::from("super-secret-key"));
        let rendered = format!("{cosmos:?}");
        assert_safe_debug_render(&rendered, "MasterKey");
        assert!(
            !rendered.contains("super-secret-key"),
            "MasterKey Debug output must not contain the raw key bytes: {rendered:?}"
        );
    }
}
