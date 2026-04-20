// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! TLS backend selection for the Cosmos DB driver.

use std::fmt;

/// Selects and configures the TLS backend used by the Cosmos DB driver.
///
/// Pass a value of this type to
/// [`CosmosDriverRuntimeBuilder::with_tls_backend()`](crate::driver::CosmosDriverRuntimeBuilder::with_tls_backend)
/// to control which TLS stack is used for all HTTP connections made by the driver.
///
/// The `Rustls` variant holds an `Option` for rustls-specific configuration:
/// - `None` forces the backend (via `use_rustls_tls()`) without any custom configuration.
/// - `Some(config)` passes the pre-configured `rustls::ClientConfig` to the transport via
///   `use_preconfigured_tls()`.
///
/// The `Rustls` variant is available when the `rustls` feature is enabled.
#[derive(Clone, Default)]
#[non_exhaustive]
pub enum TlsBackend {
    /// Use the default backend provided by the HTTP stack (reqwest, usually).
    #[default]
    Default,

    /// Use [rustls](https://docs.rs/rustls) as the TLS implementation.
    ///
    /// Pass `None` to force rustls with default settings, or `Some(config)` to supply a
    /// pre-configured `rustls::ClientConfig`. Use `Some(config)` to specify a custom crypto
    /// provider such as SymCrypt or a FIPS-only aws-lc-rs build.
    ///
    /// Only available when the `rustls` feature is enabled.
    #[cfg(feature = "rustls")]
    Rustls(Option<Box<rustls::ClientConfig>>),

    /// Use the platform's native TLS implementation (e.g., SChannel on Windows,
    /// Secure Transport on macOS, OpenSSL on Linux).
    ///
    /// Only available when the `native_tls` feature is enabled.
    #[cfg(feature = "native_tls")]
    NativeTls,
}

impl fmt::Debug for TlsBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TlsBackend::Default => write!(f, "TlsBackend::Default"),
            #[cfg(feature = "rustls")]
            TlsBackend::Rustls(None) => write!(f, "TlsBackend::Rustls(None)"),
            #[cfg(feature = "rustls")]
            TlsBackend::Rustls(Some(_)) => write!(f, "TlsBackend::Rustls(Some(..))"),
            #[cfg(feature = "native_tls")]
            TlsBackend::NativeTls => write!(f, "TlsBackend::NativeTls"),
        }
    }
}
