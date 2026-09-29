// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Formatting helpers.

pub use typespec_client_core::fmt::*;

/// Deserializers for `Cow<'_, str>` and `Option<Cow<'_, str>>`.
pub(crate) mod borrowed_str {
    use serde::{de::Visitor, Deserializer};
    use std::{borrow::Cow, fmt};

    // /// Deserialize a `Cow<'_, str>`.
    // pub fn deserialize<'de, D>(deserializer: D) -> Result<Cow<'de, str>, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     deserializer.deserialize_str(CowVisitor)
    // }

    pub struct CowVisitor;

    impl<'de> Visitor<'de> for CowVisitor {
        type Value = Cow<'de, str>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an optionally borrowed string")
        }

        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Cow::Borrowed(v))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Cow::Owned(v.to_owned()))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Cow::Owned(v))
        }
    }

    pub mod option {
        use super::*;

        /// Deserialize an `Option<Cow<'_, str>>`.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Cow<'de, str>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            use serde::de::Error;

            struct OptionVisitor;

            impl<'de> Visitor<'de> for OptionVisitor {
                type Value = Option<Cow<'de, str>>;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str("null or an optionally borrowed string")
                }

                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(None)
                }

                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    Ok(Some(deserializer.deserialize_str(CowVisitor)?))
                }
            }

            deserializer.deserialize_option(OptionVisitor)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        struct M<'a> {
            // `Option<Cow<'a, str>>` always `Cow::Owned`: https://github.com/serde-rs/serde/issues/2016
            #[serde(borrow, default, deserialize_with = "option::deserialize")]
            pub message: Option<Cow<'a, str>>,
        }

        #[test]
        fn borrowed() {
            const JSON: &[u8] = br#"{"message":"foobar"}"#;
            let m: M = serde_json::from_slice(JSON).unwrap();
            assert!(matches!(m.message, Some(Cow::Borrowed(s)) if s == "foobar"));
        }

        #[test]
        fn owned_lf() {
            const JSON: &[u8] = br#"{"message":"foo\nbar"}"#;
            let m: M = serde_json::from_slice(JSON).unwrap();
            assert!(matches!(m.message, Some(Cow::Owned(s)) if s == "foo\nbar"));
        }

        #[test]
        fn owned_crlf() {
            const JSON: &[u8] = br#"{"message":"foo\r\nbar"}"#;
            let m: M = serde_json::from_slice(JSON).unwrap();
            assert!(matches!(m.message, Some(Cow::Owned(s)) if s == "foo\r\nbar"));
        }

        #[test]
        fn missing() {
            const JSON: &[u8] = br#"{}"#;
            let m: M = serde_json::from_slice(JSON).unwrap();
            assert!(matches!(m.message, None));
        }
    }
}
