// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Proxy configuration diagnostics.

/// Captures proxy configuration for diagnostic purposes.
///
/// Built once at client creation time and inserted into the request
/// [`Context`](azure_core::http::Context) so that policies and diagnostic
/// consumers can identify when a proxy is in use.
#[derive(Clone, Debug)]
pub struct ProxyConfiguration {
    /// Whether proxy usage is allowed.
    pub proxy_allowed: bool,
    /// Whether `HTTPS_PROXY` (or `https_proxy`) was set at client creation time.
    pub https_proxy_set: bool,
    /// Whether `HTTP_PROXY` (or `http_proxy`) was set at client creation time.
    pub http_proxy_set: bool,
}

impl ProxyConfiguration {
    /// Snapshots the current proxy environment variables.
    pub fn from_env(proxy_allowed: bool) -> Self {
        let (https_proxy_set, http_proxy_set) = if proxy_allowed {
            (
                std::env::var("HTTPS_PROXY")
                    .or_else(|_| std::env::var("https_proxy"))
                    .is_ok(),
                std::env::var("HTTP_PROXY")
                    .or_else(|_| std::env::var("http_proxy"))
                    .is_ok(),
            )
        } else {
            (false, false)
        };

        Self {
            proxy_allowed,
            https_proxy_set,
            http_proxy_set,
        }
    }
}
