//! Utilities for interacting with [`StoredProcedure`]s.

use std::fmt::Debug;

use azure_core::error::ResultExt;
use serde::Serialize;

/// A piece of application logic that is registered and executed against a collection as a single transaction
///
/// You can learn more about stored procedures [here](https://docs.microsoft.com/rest/api/cosmos-db/stored-procedures).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredProcedure {
    /// The procedure id
    pub id: String,
    #[serde(rename = "_rid")]
    /// The resource id
    pub rid: String,
    /// The last updated timestamp
    #[serde(rename = "_ts")]
    pub ts: u64,
    /// The resource's uri
    #[serde(rename = "_self")]
    pub _self: String,
    /// The resource's etag used for concurrency control
    #[serde(rename = "_etag")]
    pub etag: String,
    /// The body
    pub body: String,
}

impl StoredProcedure {
    /// The name of the stored procedure
    pub fn name(&self) -> &str {
        &self.id
    }
}

/// A list of parameters passed to the stored procedure.
#[derive(Clone)]
pub struct Parameters {
    vec: Vec<String>,
}

impl Parameters {
    /// Create a new parameter list.
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    /// Push a parameter on to the list.
    pub fn push<T: Serialize>(&mut self, item: &T) -> azure_core::Result<()> {
        self.vec.push(serde_json::to_string(item).with_context(
            azure_core::error::ErrorKind::DataConversion,
            || {
                let ty = std::any::type_name::<T>();
                format!("failed to convert `{ty}` to StoredProcedure parameter")
            },
        )?);
        Ok(())
    }

    /// Convert the list to json
    pub(crate) fn to_json(&self) -> String {
        let mut result = String::from("[");
        let items = self.vec.join(", ");
        result.push_str(&items);
        result.push(']');
        result
    }
}

impl Debug for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_json())
    }
}

impl<T, U> From<T> for Parameters
where
    T: IntoIterator<Item = U>,
    U: Serialize,
{
    fn from(iter: T) -> Self {
        let mut params = Self::new();
        for item in iter {
            params.push(&item).unwrap();
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let owned = "owned".to_owned();

        let mut serialized = Parameters::new();
        serialized.push(&"aaa").unwrap();
        serialized.push(&owned).unwrap();
        serialized.push(&100u64).unwrap();
        assert_eq!(serialized.to_json(), "[\"aaa\", \"owned\", 100]");

        let vector = vec!["pollo", "arrosto"];
        let parameters: Parameters = vector.into();
        assert_eq!(parameters.to_json(), "[\"pollo\", \"arrosto\"]");

        let array = ["pollo", "arrosto"];
        let parameters: Parameters = array.into();
        assert_eq!(parameters.to_json(), "[\"pollo\", \"arrosto\"]");

        let slice = &["pollo", "arrosto"][..];
        let parameters: Parameters = slice.into();
        assert_eq!(parameters.to_json(), "[\"pollo\", \"arrosto\"]");
    }
}
