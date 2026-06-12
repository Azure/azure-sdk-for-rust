// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Conditional request types built on top of [`Etag`].

use azure_core::http::Etag;

/// Conditional request options based on ETag values.
///
/// Used for optimistic concurrency control on write operations.
/// Exactly one condition must be specified - either `IfMatch` or `IfNoneMatch`.
///
/// # Variants
///
/// - [`IfMatch`](Self::IfMatch): Operation succeeds only if the resource's current ETag matches.
///   Used for "update if unchanged" semantics (optimistic concurrency).
/// - [`IfNoneMatch`](Self::IfNoneMatch): Operation succeeds only if the resource's current ETag
///   does NOT match. Use `Etag::from("*")` for "create if not exists" semantics.
///
/// # Example
///
/// ```
/// use azure_core::http::Etag;
/// use azure_data_cosmos_driver::models::Precondition;
///
/// // Update only if the resource hasn't changed (optimistic concurrency)
/// let condition = Precondition::if_match(Etag::from("\"abc123\""));
///
/// // Create only if the resource doesn't exist
/// let condition = Precondition::if_none_match(Etag::from("*"));
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Precondition {
    /// Operation succeeds only if the resource's current ETag matches.
    ///
    /// Used for "update if unchanged" semantics (optimistic concurrency).
    IfMatch(Etag),

    /// Operation succeeds only if the resource's current ETag does NOT match.
    ///
    /// Use `Etag::from("*")` for "create if not exists" semantics.
    IfNoneMatch(Etag),
}

impl Precondition {
    /// Creates an If-Match condition.
    ///
    /// The operation succeeds only if the resource's current ETag matches the given value.
    /// Used for "update if unchanged" semantics (optimistic concurrency).
    pub fn if_match(etag: impl Into<Etag>) -> Self {
        Self::IfMatch(etag.into())
    }

    /// Creates an If-None-Match condition.
    ///
    /// The operation succeeds only if the resource's current ETag does NOT match the given value.
    /// Use `"*"` for "create if not exists" semantics.
    pub fn if_none_match(etag: impl Into<Etag>) -> Self {
        Self::IfNoneMatch(etag.into())
    }

    /// Returns the ETag if this is an If-Match condition.
    pub fn as_if_match(&self) -> Option<&Etag> {
        match self {
            Self::IfMatch(etag) => Some(etag),
            Self::IfNoneMatch(_) => None,
        }
    }

    /// Returns the ETag if this is an If-None-Match condition.
    pub fn as_if_none_match(&self) -> Option<&Etag> {
        match self {
            Self::IfNoneMatch(etag) => Some(etag),
            Self::IfMatch(_) => None,
        }
    }

    /// Returns `true` if this is an If-Match condition.
    pub fn is_if_match(&self) -> bool {
        matches!(self, Self::IfMatch(_))
    }

    /// Returns `true` if this is an If-None-Match condition.
    pub fn is_if_none_match(&self) -> bool {
        matches!(self, Self::IfNoneMatch(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_match_accessors() {
        let etag = Etag::from("abc123");
        let condition = Precondition::if_match(etag.clone());

        assert!(condition.is_if_match());
        assert!(!condition.is_if_none_match());
        assert_eq!(condition.as_if_match(), Some(&etag));
        assert_eq!(condition.as_if_none_match(), None);
    }

    #[test]
    fn if_none_match_accessors() {
        let etag = Etag::from("*");
        let condition = Precondition::if_none_match(etag.clone());

        assert!(!condition.is_if_match());
        assert!(condition.is_if_none_match());
        assert_eq!(condition.as_if_match(), None);
        assert_eq!(condition.as_if_none_match(), Some(&etag));
    }
}
