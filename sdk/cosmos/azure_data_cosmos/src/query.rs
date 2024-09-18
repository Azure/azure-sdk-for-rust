use serde::Serialize;

/// Represents a Cosmos DB Query, with optional parameters.
///
/// # Examples
///
/// Create a query using [`Query::from()`], and use  [`Query::with_parameter()`] to add parameters to it as needed.
///
/// ```rust
/// # use azure_data_cosmos::{Query,NullValue};
/// let query = Query::from("SELECT * FROM c WHERE c.id = @customer_id")
///     .with_parameter("@customer_id", 42).unwrap();
/// ```
///
/// # Specifying Parameters
///
/// Any JSON-serializable value, including the [`NullValue`](crate::NullValue) marker value, can be used as a parameter.
/// The [`Query::with_parameter()`] method accepts any type that implements [`serde::Serialize`] as a value.
/// Because the type needs to be serialized in order to be sent as a query parameter, the [`Query::with_parameter()`] method is fallible and may return [`Result::Err`].
///
/// ```rust
/// # use azure_data_cosmos::{Query, NullValue};
/// let query = Query::from("SELECT * FROM c WHERE ...")
///     .with_parameter("@customer_id", 42).unwrap()
///     .with_parameter("@customer_name", "Contoso").unwrap()
///     .with_parameter("@is_active", true).unwrap()
///     .with_parameter("@offer_code", NullValue).unwrap();
/// ```
///
/// This includes arrays and objects, if they implement [`serde::Serialize`]:
///
/// ```rust
/// # use azure_data_cosmos::{Query, NullValue};
/// #[derive(serde::Serialize)]
/// struct CustomerInfo {
///     id: u64,
///     name: String
/// }
/// let query = Query::from("SELECT * FROM c WHERE ...")
///     .with_parameter("@customer_info", CustomerInfo { id: 42, name: "Contoso".into() }).unwrap();
/// ```
#[derive(Debug, Serialize)]
pub struct Query {
    query: String,

    #[serde(skip_serializing_if = "Vec::is_empty")] // Don't serialize an empty array.
    parameters: Vec<QueryParameter>,
}

impl Query {
    /// Creates a new [`Query`] with the same text and the same parameters, with one additional parameter added.
    ///
    /// Returns an error if the value cannot be serialized to JSON.
    pub fn with_parameter(
        mut self,
        name: impl Into<String>,
        value: impl Serialize,
    ) -> azure_core::Result<Self> {
        let parameter = QueryParameter {
            name: name.into(),
            value: QueryParameterValue(serde_json::to_value(value)?),
        };
        self.parameters.push(parameter);

        Ok(self)
    }
}

impl<T: Into<String>> From<T> for Query {
    fn from(value: T) -> Self {
        let query = value.into();
        Self {
            query,
            parameters: vec![],
        }
    }
}

/// Represents a single parameter in a Cosmos DB query.
#[derive(Debug, Serialize)]
struct QueryParameter {
    name: String,
    value: QueryParameterValue,
}

/// Represents a value that can be provided to a query parameter.
#[derive(Debug, Serialize)]
#[serde(transparent)]
struct QueryParameterValue(serde_json::Value);

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::{NullValue, Query};

    #[test]
    pub fn serialize_query_without_parameters() {
        let query: Query = "SELECT * FROM c".into();
        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"query":"SELECT * FROM c"}"#);
    }

    #[test]
    pub fn serialize_query_with_string_parameters() {
        let query = Query::from("SELECT * FROM c")
            .with_parameter("name1", "value1")
            .unwrap()
            .with_parameter("name2", "value2")
            .unwrap();
        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(
            serialized,
            r#"{"query":"SELECT * FROM c","parameters":[{"name":"name1","value":"value1"},{"name":"name2","value":"value2"}]}"#
        );
    }

    #[test]
    pub fn serialize_query_with_various_parameter_types() {
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
            .with_parameter("string_param", "value1")
            .unwrap()
            .with_parameter("int_param", 42)
            .unwrap()
            .with_parameter("float_param", 4.2)
            .unwrap()
            .with_parameter("bool_param", true)
            .unwrap()
            .with_parameter("obj_param", obj_param)
            .unwrap()
            .with_parameter("arr_param", &["a", "b", "c"])
            .unwrap()
            .with_parameter("null_option", null_option)
            .unwrap()
            .with_parameter("null_value", NullValue)
            .unwrap();
        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(
            serialized,
            r#"{"query":"SELECT * FROM c","parameters":[{"name":"string_param","value":"value1"},{"name":"int_param","value":42},{"name":"float_param","value":4.2},{"name":"bool_param","value":true},{"name":"obj_param","value":{"name":"foo","value":"bar"}},{"name":"arr_param","value":["a","b","c"]},{"name":"null_option","value":null},{"name":"null_value","value":null}]}"#
        );
    }
}
