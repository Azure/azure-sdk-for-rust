// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ETag types for optimistic concurrency control.

use std::borrow::Cow;

/// An ETag value used for optimistic concurrency control.
///
/// ETags are opaque identifiers representing a specific version of a resource.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ETag(pub Cow<'static, str>);

impl ETag {
    /// Creates a new ETag with the given value.
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self(value.into())
    }

    /// Returns the ETag value as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for ETag {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ETag {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<Cow<'static, str>> for ETag {
    fn from(value: Cow<'static, str>) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for ETag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

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
///   does NOT match. Use `ETag::new("*")` for "create if not exists" semantics.
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::models::{ETag, ETagCondition};
///
/// // Update only if the resource hasn't changed (optimistic concurrency)
/// let condition = ETagCondition::if_match("\"abc123\"");
///
/// // Create only if the resource doesn't exist
/// let condition = ETagCondition::if_none_match("*");
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ETagCondition {
    /// Operation succeeds only if the resource's current ETag matches.
    ///
    /// Used for "update if unchanged" semantics (optimistic concurrency).
    IfMatch(ETag),

    /// Operation succeeds only if the resource's current ETag does NOT match.
    ///
    /// Use `ETag::new("*")` for "create if not exists" semantics.
    IfNoneMatch(ETag),
}

impl ETagCondition {
    /// Creates an If-Match condition.
    ///
    /// The operation succeeds only if the resource's current ETag matches the given value.
    /// Used for "update if unchanged" semantics (optimistic concurrency).
    pub fn if_match(etag: impl Into<ETag>) -> Self {
        Self::IfMatch(etag.into())
    }

    /// Creates an If-None-Match condition.
    ///
    /// The operation succeeds only if the resource's current ETag does NOT match the given value.
    /// Use `"*"` for "create if not exists" semantics.
    pub fn if_none_match(etag: impl Into<ETag>) -> Self {
        Self::IfNoneMatch(etag.into())
    }

    /// Returns the ETag if this is an If-Match condition.
    pub fn as_if_match(&self) -> Option<&ETag> {
        match self {
            Self::IfMatch(etag) => Some(etag),
            Self::IfNoneMatch(_) => None,
        }
    }

    /// Returns the ETag if this is an If-None-Match condition.
    pub fn as_if_none_match(&self) -> Option<&ETag> {
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
        let etag = ETag::new("abc123");
        let condition = ETagCondition::if_match(etag.clone());

        assert!(condition.is_if_match());
        assert!(!condition.is_if_none_match());
        assert_eq!(condition.as_if_match(), Some(&etag));
        assert_eq!(condition.as_if_none_match(), None);
    }

    #[test]
    fn if_none_match_accessors() {
        let etag = ETag::new("*");
        let condition = ETagCondition::if_none_match(etag.clone());

        assert!(!condition.is_if_match());
        assert!(condition.is_if_none_match());
        assert_eq!(condition.as_if_match(), None);
        assert_eq!(condition.as_if_none_match(), Some(&etag));
    }
}
