// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration for session token authentication.

use crate::session::provider::SessionProvider;
use azure_core::fmt::SafeDebug;
use std::sync::Arc;

/// Determines whether blob operations use session token authentication.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SessionMode {
    /// The client library decides the behavior; may change in future releases.
    /// Currently resolves to [`SessionMode::Disabled`].
    #[default]
    Auto,

    /// Always use bearer token authentication; never use session tokens.
    Disabled,

    /// Opt in to session token authentication, with one cached session per container.
    Enabled,
}

impl SessionMode {
    /// Resolves [`SessionMode::Auto`] to the current default behavior.
    fn resolve(self) -> SessionMode {
        match self {
            // Auto maps to Disabled for now; this may change in the future.
            SessionMode::Auto => SessionMode::Disabled,
            other => other,
        }
    }
}

/// Options for configuring session token authentication for blob operations.
///
/// Session token authentication currently applies only to blob download
/// operations authenticated with a [`TokenCredential`](azure_core::credentials::TokenCredential).
#[derive(Clone, Default, SafeDebug)]
pub struct SessionOptions {
    /// The session authentication mode. Defaults to [`SessionMode::Auto`].
    #[safe(true)]
    pub mode: SessionMode,

    /// The account name used to sign session requests.
    ///
    /// Optional. When unset, the account name is derived from the request URL at
    /// signing time. Set this explicitly when using a custom endpoint from which
    /// the account name cannot be derived.
    pub account_name: Option<String>,

    /// An explicit session provider to share across clients.
    ///
    /// Construct one with
    /// [`ContainerSessionProvider::new`](crate::ContainerSessionProvider::new) to
    /// reuse a single session cache across multiple clients. When unset, each
    /// client creates its own per-container provider.
    pub session_provider: Option<Arc<dyn SessionProvider>>,
}

impl SessionOptions {
    /// Whether session token authentication is enabled after resolving
    /// [`SessionMode::Auto`].
    pub(crate) fn is_enabled(&self) -> bool {
        self.mode.resolve() == SessionMode::Enabled
    }

    /// Whether session authentication was explicitly requested via
    /// [`SessionMode::Enabled`], as opposed to being enabled by default.
    pub(crate) fn is_explicitly_enabled(&self) -> bool {
        self.mode == SessionMode::Enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_resolves_to_disabled() {
        assert_eq!(SessionMode::Auto.resolve(), SessionMode::Disabled);
        assert_eq!(SessionMode::Enabled.resolve(), SessionMode::Enabled);
        assert_eq!(SessionMode::Disabled.resolve(), SessionMode::Disabled);
    }

    #[test]
    fn is_enabled_reflects_resolved_mode() {
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
