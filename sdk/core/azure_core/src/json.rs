use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

/// Serialize a type to json.
pub fn to_json<T>(value: &T) -> crate::Result<Bytes>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}

/// Reads the XML from bytes.
pub fn from_json<S, T>(body: S) -> crate::Result<T>
where
    S: AsRef<[u8]>,
    T: DeserializeOwned,
{
    serde_json::from_slice(body.as_ref()).map_err(Into::into)
}
