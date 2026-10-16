// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration for session token authentication.

use crate::session::provider::SessionProvider;
use azure_core::fmt::SafeDebug;
use std::sync::Arc;

/// Determines whether blob operations use session token authentication.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SessionMode {
    /// Always use bearer token authentication; never use session tokens.
    #[default]
    Disabled,

    /// Opt in to session token authentication, with one cached session per container.
    Enabled,
}

/// Options for configuring session token authentication for blob operations.
///
/// Session token authentication currently applies only to blob download
/// operations authenticated with a [`TokenCredential`](azure_core::credentials::TokenCredential).
#[derive(Clone, Default, SafeDebug)]
pub struct SessionOptions {
    /// The session authentication mode. Defaults to [`SessionMode::Disabled`].
    #[safe(true)]
    pub mode: SessionMode,

    /// The account name used to sign session requests.
    ///
    /// Optional. When unset, the account name is derived from the client endpoint when
    /// the client is created. Set this explicitly when using a custom endpoint, such as
    /// a custom domain, from which the account name cannot be derived.
    pub account_name: Option<String>,

    /// A shared session provider.
    ///
    /// Wrap [`ContainerSessionProvider::new`](crate::ContainerSessionProvider::new)
    /// in an [`Arc`] to reuse its session cache across clients. When unset, each
    /// client uses its own provider.
    pub session_provider: Option<Arc<dyn SessionProvider>>,
}

impl SessionOptions {
    /// Whether session token authentication is enabled.
    pub(crate) fn is_enabled(&self) -> bool {
        self.mode == SessionMode::Enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_enabled_reflects_mode() {
        assert!(!SessionOptions::default().is_enabled());
        assert!(!SessionOptions {
            mode: SessionMode::Disabled,
            ..Default::default()
        }
        .is_enabled());
        assert!(SessionOptions {
            mode: SessionMode::Enabled,
            ..Default::default()
        }
        .is_enabled());
    }

    #[test]
    fn debug_redacts_account_name() {
        let options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: Some("sensitive-account".into()),
            ..Default::default()
        };

        let debug = format!("{options:?}");
        assert_eq!(debug, "SessionOptions { mode: Enabled, .. }");
        assert!(!debug.contains("sensitive-account"));
    }
}
