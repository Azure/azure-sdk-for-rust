// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Formatting helpers.

use std::borrow::Cow;

/// Converts ASCII characters in `value` to lowercase if required; otherwise, returns the original slice.
///
/// # Examples
///
/// Returns the original slice:
///
/// ```
/// # use std::borrow::Cow;
/// # use typespec_client_core::fmt::to_ascii_lowercase;
/// let actual = to_ascii_lowercase("hello, world!");
/// assert!(matches!(actual, Cow::Borrowed("hello, world!")));
/// ```
///
/// Returns a clone converted to lowercase ASCII character.
///
/// ```
/// # use std::borrow::Cow;
/// # use typespec_client_core::fmt::to_ascii_lowercase;
/// let actual = to_ascii_lowercase("hello, World!");
/// assert!(matches!(
///     actual,
///     Cow::Owned(expected) if expected == "hello, world!"
/// ));
/// ```
pub fn to_ascii_lowercase(value: &str) -> Cow<'_, str> {
    for (i, c) in value.chars().enumerate() {
        if c.is_ascii_uppercase() {
            let mut s = value.to_owned();
            s[i..].make_ascii_lowercase();

            return Cow::Owned(s);
        }
    }

    Cow::Borrowed(value)
}

#[test]
fn test_to_ascii_lowercase() {
    let actual = to_ascii_lowercase("hello, 🌎!");
    assert!(matches!(actual, Cow::Borrowed("hello, 🌎!")));

    let actual = to_ascii_lowercase("Hello, 🌎!");
    assert!(matches!(
        actual,
        Cow::Owned(expected) if expected == "hello, 🌎!"
    ));
}
