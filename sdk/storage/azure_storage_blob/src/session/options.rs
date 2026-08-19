// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration for session token authentication.

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
#[derive(Clone, Debug, Default)]
pub struct SessionOptions {
    /// The session authentication mode. Defaults to [`SessionMode::Auto`].
    pub mode: SessionMode,

    /// The account name used to sign session requests.
    ///
    /// Optional. When unset, the account name is derived from the request URL at
    /// signing time. Set this explicitly when using a custom endpoint from which
    /// the account name cannot be derived.
    pub account_name: Option<String>,
}

impl SessionOptions {
    /// Whether session token authentication is enabled after resolving
    /// [`SessionMode::Auto`].
    pub(crate) fn is_enabled(&self) -> bool {
        self.mode.resolve() == SessionMode::Enabled
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
}
