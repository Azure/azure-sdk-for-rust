// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! serde helpers

/// serde functions for types that are sent and received as strings but aren't surfaced as strings.
pub mod as_string {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Deserializes a string into a T. T must implement [`std::str::FromStr`].
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        let to_deserialize = <Option<String>>::deserialize(deserializer)?;
        match to_deserialize {
            Some(to_deserialize) => Ok(Some(
                T::from_str(&to_deserialize).map_err(serde::de::Error::custom)?,
            )),
            None => Ok(None),
        }
    }

    /// Serializes T in string format. T must implement [`std::string::ToString`].
    pub fn serialize<S, T>(to_serialize: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: std::string::ToString,
    {
        match to_serialize {
            Some(to_serialize) => String::serialize(&to_serialize.to_string(), serializer),
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::json::{from_json, to_json};
    use serde::{Deserialize, Serialize};
    use serde_json::Number;

    #[derive(Default, Deserialize, Serialize)]
    struct TestType {
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "super::as_string"
        )]
        a_bool: Option<bool>,

        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "super::as_string"
        )]
        json_number: Option<serde_json::Number>,

        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "super::as_string"
        )]
        some_float: Option<f64>,

        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "super::as_string"
        )]
        some_int: Option<i32>,
    }

    #[test]
    fn test_deserialize_none() -> crate::Result<()> {
        let json_body = r#"{}"#;
        let test_type: TestType = from_json(json_body)?;
        assert!(test_type.a_bool.is_none());
        assert!(test_type.json_number.is_none());
        assert!(test_type.some_float.is_none());
        assert!(test_type.some_int.is_none());
        Ok(())
    }

    #[test]
    fn test_deserialize_all() -> crate::Result<()> {
        let json_body = r#"{"a_bool":"true","json_number":"123456789","some_float":"3.1415926","some_int":"42"}"#;
        let test_type: TestType = from_json(json_body)?;
        assert_eq!(test_type.a_bool, Some(true));
        assert_eq!(test_type.json_number, Number::from_i128(123456789));
        assert_eq!(test_type.some_float, Some(3.1415926));
        assert_eq!(test_type.some_int, Some(42));
        Ok(())
    }

    #[test]
    fn test_serialize_none() -> crate::Result<()> {
        let test_type = TestType::default();
        let json_body = to_json(&test_type)?;
        assert_eq!(json_body, r#"{}"#);
        Ok(())
    }

    #[test]
    fn test_serialize_all() -> crate::Result<()> {
        let test_type = TestType {
            a_bool: Some(true),
            json_number: Number::from_i128(123456789),
            some_float: Some(3.1415926),
            some_int: Some(42),
        };
        let json_body = to_json(&test_type)?;
        assert_eq!(
            json_body,
            r#"{"a_bool":"true","json_number":"123456789","some_float":"3.1415926","some_int":"42"}"#
        );
        Ok(())
    }
}
