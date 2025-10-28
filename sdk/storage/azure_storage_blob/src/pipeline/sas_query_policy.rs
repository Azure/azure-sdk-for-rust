/* Copyright (c) Microsoft Corporation.
   Licensed under the MIT License. */

use async_trait::async_trait;
use azure_core::http::{
    policies::{Policy, PolicyResult},
    Context, Request,
};
use std::collections::HashSet;
use std::sync::Arc;

/// A policy that ensures SAS query parameters from the client endpoint
/// are present on every outgoing request.
///
/// This is intended for use in SAS-only client constructors. The SAS query
/// string from the endpoint is parsed at construction time and cached in the
/// policy. For each request, any missing SAS parameters are appended to the
/// request URL, preserving existing query parameters added by the operation.
///
/// Notes:
/// - Existing query parameters are preserved and take precedence; SAS params
///   that are already present on the request will NOT be duplicated or
///   overwritten.
/// - Parameter name comparison is case-sensitive (Azure SAS parameters are
///   conventionally lower-case, e.g. `sv`, `ss`, `srt`, `sp`, `se`, `st`,
///   `spr`, `sig`).
#[derive(Debug, Clone)]
pub struct SasQueryPolicy {
    pairs: Vec<(String, String)>,
}

impl SasQueryPolicy {
    /// Create a policy from a raw query string (e.g., everything after the `?`).
    ///
    /// If `query` is empty, the resulting policy will be a no-op.
    pub fn from_query_str(query: &str) -> Self {
        // Parse the query into key/value pairs, percent-decoding as necessary.
        // The `append_pair` API will handle re-encoding on write.
        let pairs = if query.is_empty() {
            Vec::new()
        } else {
            url::form_urlencoded::parse(query.as_bytes())
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect()
        };
        Self { pairs }
    }

    /// Returns true if the policy has no SAS parameters and will be a no-op.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for SasQueryPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // If we have nothing to append, skip quickly.
        if self.pairs.is_empty() {
            return next[0].send(ctx, request, &next[1..]).await;
        }

        // Snapshot existing query parameter names to avoid duplicates.
        // We only check for the presence of the key (name); if present, we
        // do not attempt to overwrite the value. This preserves any operation-
        // specific overrides (e.g., pre-existing SAS on the request).
        let existing: HashSet<String> = request
            .url()
            .query_pairs()
            .map(|(k, _)| k.to_string())
            .collect();

        // Append missing SAS pairs.
        {
            let mut qp = request.url_mut().query_pairs_mut();
            for (k, v) in &self.pairs {
                if !existing.contains(k) {
                    qp.append_pair(k, v);
                }
            }
        }

        next[0].send(ctx, request, &next[1..]).await
    }
}
