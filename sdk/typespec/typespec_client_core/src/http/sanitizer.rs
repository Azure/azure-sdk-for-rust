// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore traceparent tracestate

use crate::http::{headers::Headers, Url};
use std::{collections::HashSet, sync::LazyLock};

pub static DEFAULT_ALLOWED_HEADER_NAMES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "accept",
        "cache-control",
        "connection",
        "content-length",
        "content-type",
        "date",
        "etag",
        "expires",
        "if-match",
        "if-modified-since",
        "if-none-match",
        "if-unmodified-since",
        "last-modified",
        "ms-cv",
        "pragma",
        "request-id",
        "retry-after",
        "server",
        "traceparent",
        "tracestate",
        "transfer-encoding",
        "user-agent",
        "www-authenticate",
        "x-ms-request-id",
        "x-ms-client-request-id",
        "x-ms-return-client-request-id",
    ]
    .iter()
    .copied()
    .collect()
});

pub static DEFAULT_ALLOWED_QUERY_PARAMETERS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| ["api-version"].iter().copied().collect());

pub const REDACTED_PATTERN: &str = "REDACTED";

/// A trait that extends a type with sanitization capabilities
pub trait Sanitizer {
    /// Sanitizes a type by removing or masking sensitive information based on a set of allowed patterns
    ///
    /// # Arguments
    ///
    /// * `patterns` - A slice of string slices containing patterns which are allowed in the Url.
    ///
    /// # Returns
    ///
    /// A String containing the sanitized URL
    fn sanitize(&self, allowed_patterns: &HashSet<&'static str>) -> String;
}

impl Sanitizer for Url {
    fn sanitize(&self, allowed_patterns: &HashSet<&'static str>) -> String {
        // Special case the "no query" case to avoid unnecessary allocations
        if self.query().is_none() || cfg!(feature = "debug") {
            return self.as_str().to_string();
        }
        let mut target_url = self.clone();
        target_url.query_pairs_mut().clear();

        // Iterate through the query pairs and only add those query parameters which are in the allowed patterns
        for query in self.query_pairs() {
            if !allowed_patterns.contains(&query.0.as_ref()) {
                target_url
                    .query_pairs_mut()
                    .append_pair(&query.0, REDACTED_PATTERN);
            } else {
                target_url
                    .query_pairs_mut()
                    .append_pair(&query.0, query.1.as_ref());
            }
        }
        let sanitized = target_url.as_str().to_string();
        sanitized
    }
}

impl Sanitizer for Headers {
    fn sanitize(&self, allowed_patterns: &HashSet<&'static str>) -> String {
        let mut sanitized = String::new();
        // If the debug feature is enabled, don't sanitize.
        if cfg!(feature = "debug") {
            for (name, value) in self.iter() {
                sanitized.push_str(&format!("{}: {} ", name.as_str(), value.as_str()));
            }
        } else {
            // Output the headers in a similar format to how they would appear in an HTTP request
            for (name, value) in self.iter() {
                if !allowed_patterns.contains(&name.as_str()) {
                    sanitized.push_str(&format!("{}: {} ", name.as_str(), REDACTED_PATTERN));
                } else {
                    sanitized.push_str(&format!("{}: {} ", name.as_str(), value.as_str()));
                }
            }
        }

        sanitized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_url_with_allowed_query_parameters() {
        let url = Url::parse("https://example.com/api?key=secret123&user=admin").unwrap();
        let patterns = HashSet::from(["key", "user"]);

        let sanitized = url.sanitize(&patterns);
        assert_eq!(
            sanitized,
            "https://example.com/api?key=secret123&user=admin"
        );
    }

    #[test]
    fn sanitize_url_with_redacted_parameters() {
        let url = Url::parse("https://example.com/api?foo=bar").unwrap();
        let patterns = HashSet::from(["key", "admin"]);

        let sanitized = url.sanitize(&patterns);
        assert_eq!(sanitized, "https://example.com/api?foo=REDACTED");
    }

    #[test]
    fn sanitize_url_with_no_query_parameters() {
        let url = Url::parse("https://example.com/api").unwrap();
        let patterns = HashSet::from(["secret123", "admin"]);

        let sanitized = url.sanitize(&patterns);
        assert_eq!(sanitized, "https://example.com/api");
    }
}
