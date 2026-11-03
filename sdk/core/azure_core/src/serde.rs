// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Serialization and deserialization helpers.

/// Skips serializing a read-only field unless the `test` and `unstable_fake` features are enabled.
///
/// Use [`option::skip_serializing_read_only()`] for optional fields to also omit absent values.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use azure_core::serde::skip_serializing_read_only;
///
/// #[derive(Debug, Deserialize, Serialize)]
/// struct M {
///     #[serde(skip_serializing_if = "skip_serializing_read_only")]
///     pub id: String,
///     pub name: Option<String>,
/// }
///
/// let m = M {
///     id: "abc123".into(),
///     name: Some("Contoso".into()),
/// };
/// let s = serde_json::to_string(&m).unwrap();
/// if cfg!(all(feature = "test", feature = "unstable_fake")) {
///     assert_eq!(s, r#"{"id":"abc123","name":"Contoso"}"#);
/// } else {
///     assert_eq!(s, r#"{"name":"Contoso"}"#);
/// }
/// ```
pub fn skip_serializing_read_only<T: ?Sized>(_value: &T) -> bool {
    !cfg!(all(feature = "test", feature = "unstable_fake"))
}

/// Serialization and deserialization helpers for `Option<T>` members.
pub mod option {
    /// Returns whether to omit an optional read-only field during serialization.
    ///
    /// Populated fields serialize when `azure_core`'s `test` and `unstable_fake` features
    /// are enabled. Absent fields are always omitted.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use azure_core::serde::option;
    ///
    /// #[derive(Debug, Deserialize, Serialize)]
    /// struct M {
    ///     #[serde(skip_serializing_if = "option::skip_serializing_read_only")]
    ///     pub id: Option<String>,
    ///     pub name: Option<String>,
    /// }
    ///
    /// let m = M {
    ///     id: Some("abc123".into()),
    ///     name: Some("Contoso".into()),
    /// };
    /// let s = serde_json::to_string(&m).unwrap();
    /// if cfg!(all(feature = "test", feature = "unstable_fake")) {
    ///     assert_eq!(s, r#"{"id":"abc123","name":"Contoso"}"#);
    /// } else {
    ///     assert_eq!(s, r#"{"name":"Contoso"}"#);
    /// }
    ///
    /// let m = M { id: None, name: m.name };
    /// assert_eq!(serde_json::to_string(&m).unwrap(), r#"{"name":"Contoso"}"#);
    /// ```
    pub fn skip_serializing_read_only<T>(value: &Option<T>) -> bool {
        super::skip_serializing_read_only(value) || value.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::{option, skip_serializing_read_only};

    #[test]
    fn read_only_fields_follow_fake_features() {
        let skip = !cfg!(all(feature = "test", feature = "unstable_fake"));
        assert_eq!(skip_serializing_read_only(&42), skip);
        assert_eq!(skip_serializing_read_only(&Vec::<u8>::new()), skip);
        assert_eq!(skip_serializing_read_only(&vec![1, 2]), skip);
        assert_eq!(option::skip_serializing_read_only(&Some(42)), skip);
        assert!(option::skip_serializing_read_only::<u8>(&None));
    }
}
