// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Models and components used to represents and execute queries.

use serde::Serialize;

#[cfg(feature = "preview_query_engine")]
mod engine;

#[cfg(feature = "preview_query_engine")]
pub(crate) mod executor;

#[cfg(feature = "preview_query_engine")]
pub use engine::*;

/// Represents a Cosmos DB Query, with optional parameters.
///
/// # Examples
///
/// Create a query using [`Query::from()`], and use  [`Query::with_parameter()`] to add parameters to it as needed.
///
/// ```rust
/// # use azure_data_cosmos::Query;
/// let query = Query::from("SELECT * FROM c WHERE c.id = @customer_id")
///     .with_parameter("@customer_id", 42).unwrap();
/// # assert_eq!(serde_json::to_string(&query).unwrap(), "{\"query\":\"SELECT * FROM c WHERE c.id = @customer_id\",\"parameters\":[{\"name\":\"@customer_id\",\"value\":42}]}");
/// ```
///
/// # Specifying Parameters
///
/// Any JSON-serializable value, including an empty tuple (`()`), which indicates `null`, can be used as a parameter.
/// The [`Query::with_parameter()`] method accepts any type that implements [`serde::Serialize`] as a value.
/// Because the type needs to be serialized in order to be sent as a query parameter, the [`Query::with_parameter()`] method is fallible and may return [`Result::Err`] if the value cannot be serialized.
///
/// ```rust
/// # use azure_data_cosmos::Query;
/// let query = Query::from("
///     SELECT * FROM c
///     WHERE c.id = @customer_id
///     AND c.name = @customer_name
///     AND c.is_active = @is_active
///     AND c.offer_code = @offer_code")
///     .with_parameter("@customer_id", 42).unwrap()
///     .with_parameter("@customer_name", "Contoso").unwrap()
///     .with_parameter("@is_active", true).unwrap()
///     .with_parameter("@offer_code", ()).unwrap();
/// # assert_eq!(serde_json::to_string(&query).unwrap(), "{\"query\":\"\\n    SELECT * FROM c\\n    WHERE c.id = @customer_id\\n    AND c.name = @customer_name\\n    AND c.is_active = @is_active\\n    AND c.offer_code = @offer_code\",\"parameters\":[{\"name\":\"@customer_id\",\"value\":42},{\"name\":\"@customer_name\",\"value\":\"Contoso\"},{\"name\":\"@is_active\",\"value\":true},{\"name\":\"@offer_code\",\"value\":null}]}");
/// ```
///
/// This includes arrays and objects, if they implement [`serde::Serialize`]:
///
/// ```rust
/// # use azure_data_cosmos::Query;
/// #[derive(serde::Serialize)]
/// struct CustomerInfo {
///     id: u64,
///     name: String
/// }
/// let query = Query::from("
///     SELECT * FROM c
///     WHERE c.id = @customer_info.id
///     AND c.name = @customer_info.name")
///     .with_parameter("@customer_info", CustomerInfo { id: 42, name: "Contoso".into() }).unwrap();
/// # assert_eq!(serde_json::to_string(&query).unwrap(), "{\"query\":\"\\n    SELECT * FROM c\\n    WHERE c.id = @customer_info.id\\n    AND c.name = @customer_info.name\",\"parameters\":[{\"name\":\"@customer_info\",\"value\":{\"id\":42,\"name\":\"Contoso\"}}]}");
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct Query {
    /// The query text itself.
    #[serde(rename = "query")]
    pub(crate) text: String,

    /// A list of parameters used in the query and their associated value.
    #[serde(skip_serializing_if = "Vec::is_empty")] // Don't serialize an empty array.
    parameters: Vec<QueryParameter>,
}

impl Query {
    /// Consumes this [`Query`] instance, adds a new parameter to it, and returns it.
    ///
    /// Returns an error if the value cannot be serialized to JSON.
    pub fn with_parameter(
        mut self,
        name: impl Into<String>,
        value: impl Serialize,
    ) -> azure_core::Result<Self> {
        let parameter = QueryParameter {
            name: name.into(),
            value: serde_json::to_value(value)?,
        };
        self.parameters.push(parameter);

        Ok(self)
    }
}

impl<T: Into<String>> From<T> for Query {
    fn from(value: T) -> Self {
        let query = value.into();
        Self {
            text: query,
            parameters: vec![],
        }
    }
}

/// Represents a single parameter in a Cosmos DB query.
#[derive(Clone, Debug, Serialize)]
struct QueryParameter {
    name: String,
    value: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde::Serialize;

    use crate::Query;

    #[test]
    pub fn serialize_query_without_parameters() -> Result<(), Box<dyn Error>> {
        let query: Query = "SELECT * FROM c".into();
        let serialized = serde_json::to_string(&query)?;
        assert_eq!(serialized, r#"{"query":"SELECT * FROM c"}"#);
        Ok(())
    }

    #[test]
    pub fn serialize_query_with_string_parameters() -> Result<(), Box<dyn Error>> {
        let query = Query::from("SELECT * FROM c")
            .with_parameter("name1", "value1")?
            .with_parameter("name2", "value2")?;
        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(
            serialized,
            r#"{"query":"SELECT * FROM c","parameters":[{"name":"name1","value":"value1"},{"name":"name2","value":"value2"}]}"#
        );
        Ok(())
    }

    #[test]
    pub fn serialize_query_with_various_parameter_types() -> Result<(), Box<dyn Error>> {
        #[derive(Serialize)]
        struct ObjectParameter {
            name: String,
            value: String,
        }
        let obj_param = ObjectParameter {
            name: "foo".into(),
            value: "bar".into(),
        };
        let null_option: Option<&str> = None;

        let query = Query::from("SELECT * FROM c")
            .with_parameter("string_param", "value1")?
            .with_parameter("int_param", 42)?
            .with_parameter("float_param", 4.2)?
            .with_parameter("bool_param", true)?
            .with_parameter("obj_param", obj_param)?
            .with_parameter("arr_param", ["a", "b", "c"])?
            .with_parameter("null_option", null_option)?
            .with_parameter("null_value", ())?;
        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(
            serialized,
            r#"{"query":"SELECT * FROM c","parameters":[{"name":"string_param","value":"value1"},{"name":"int_param","value":42},{"name":"float_param","value":4.2},{"name":"bool_param","value":true},{"name":"obj_param","value":{"name":"foo","value":"bar"}},{"name":"arr_param","value":["a","b","c"]},{"name":"null_option","value":null},{"name":"null_value","value":null}]}"#
        );
        Ok(())
    }
}
