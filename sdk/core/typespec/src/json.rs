// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! JSON serialization functions.
use crate::error::Result;
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

/// Serialize a type to JSON.
pub fn to_json<T>(value: &T) -> Result<Bytes>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}

/// Reads the JSON from bytes.
pub fn from_json<S, T>(body: S) -> Result<T>
where
    S: AsRef<[u8]>,
    T: DeserializeOwned,
{
    serde_json::from_slice(body.as_ref()).map_err(Into::into)
}

/// Reads the JSON from bytes while allowing the result to borrow from the input.
pub fn from_json_ref<'de, S, T>(body: &'de S) -> Result<T>
where
    S: AsRef<[u8]> + ?Sized,
    T: serde::Deserialize<'de>,
{
    serde_json::from_slice(body.as_ref()).map_err(Into::into)
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct Test<'a> {
        value: &'a str,
    }

    #[test]
    fn reading_json_ref() -> Result<()> {
        let json = br#"{"value":"Hello, world!"}"#;
        let test: Test<'_> = from_json_ref(&json)?;
        assert_eq!(test.value, "Hello, world!");
        Ok(())
    }
}
