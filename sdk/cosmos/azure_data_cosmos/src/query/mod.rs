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
/// You can also modify the query text using [`Query::with_text()`] to replace it entirely
/// or [`Query::append_text()`] to add to the existing text:
///
/// ```rust
/// # use azure_data_cosmos::Query;
/// let query = Query::from("SELECT * FROM c")
///     .append_text(" WHERE c.time >= @low_time")
///     .with_parameter("@low_time", "2023-01-01").unwrap()
///     .append_text(" AND c.time <= @high_time")
///     .with_parameter("@high_time", "2023-12-31").unwrap();
/// # // We can't directly access the text field as it's private, but we can serialize to verify
/// # let serialized = serde_json::to_string(&query).unwrap();
/// # assert!(serialized.contains("WHERE c.time >= @low_time AND c.time <= @high_time"));
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

    /// Replaces all parameters in this [`Query`] instance with the parameters from another [`Query`] instance, and returns it.
    ///
    /// Since the parameters in the other query are already serialized, this method cannot fail.
    #[cfg(feature = "preview_query_engine")] // Crate-private for now, and thus only in the preview_query_engine feature (which is the only place it's used).
    pub(crate) fn with_parameters_from(mut self, other: &Query) -> Self {
        self.parameters = other.parameters.clone();
        self
    }

    /// Consumes this [`Query`] instance, replaces its text with the provided value, and returns it.
    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    /// Consumes this [`Query`] instance, appends the provided text to its current text, and returns it.
    pub fn append_text(mut self, text: &str) -> Self {
        self.text.push_str(text);
        self
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

    #[test]
    pub fn with_text_replaces_query_text() {
        let query = Query::from("SELECT * FROM c").with_text("SELECT c.id FROM c".to_string());
        assert_eq!(query.text, "SELECT c.id FROM c");
    }

    #[test]
    pub fn with_text_preserves_parameters() -> Result<(), Box<dyn Error>> {
        let query = Query::from("SELECT * FROM c")
            .with_parameter("@id", 42)?
            .with_text("SELECT c.name FROM c WHERE c.id = @id".to_string());

        assert_eq!(query.text, "SELECT c.name FROM c WHERE c.id = @id");
        assert_eq!(query.parameters.len(), 1);
        assert_eq!(query.parameters[0].name, "@id");
        Ok(())
    }

    #[test]
    pub fn append_text_adds_to_existing_text() {
        let query = Query::from("SELECT * FROM c").append_text(" WHERE c.id = @id");
        assert_eq!(query.text, "SELECT * FROM c WHERE c.id = @id");
    }

    #[test]
    pub fn append_text_preserves_parameters() -> Result<(), Box<dyn Error>> {
        let query = Query::from("SELECT * FROM c")
            .with_parameter("@id", 42)?
            .append_text(" WHERE c.id = @id");

        assert_eq!(query.text, "SELECT * FROM c WHERE c.id = @id");
        assert_eq!(query.parameters.len(), 1);
        assert_eq!(query.parameters[0].name, "@id");
        Ok(())
    }

    #[test]
    pub fn method_chaining_works_with_new_methods() -> Result<(), Box<dyn Error>> {
        let query = Query::from("SELECT * FROM c")
            .append_text(" WHERE c.time >= @low_time")
            .with_parameter("@low_time", "2023-01-01")?
            .append_text(" AND c.time <= @high_time")
            .with_parameter("@high_time", "2023-12-31")?;

        assert_eq!(
            query.text,
            "SELECT * FROM c WHERE c.time >= @low_time AND c.time <= @high_time"
        );
        assert_eq!(query.parameters.len(), 2);
        Ok(())
    }

    #[test]
    #[cfg(feature = "preview_query_engine")]
    pub fn with_parameters_from_replaces_all_parameters() -> Result<(), Box<dyn Error>> {
        let source_query = Query::from("SELECT * FROM c")
            .with_parameter("@id", 42)?
            .with_parameter("@name", "Contoso")?;

        let target_query = Query::from("SELECT c.value FROM c WHERE c.id = @id AND c.name = @name")
            .with_parameter("@old_param", "old_value")?
            .with_parameters_from(&source_query);

        // Check that the text is preserved from the target query
        assert_eq!(
            target_query.text,
            "SELECT c.value FROM c WHERE c.id = @id AND c.name = @name"
        );

        // Check that parameters are replaced with those from source query
        assert_eq!(target_query.parameters.len(), 2);
        assert_eq!(target_query.parameters[0].name, "@id");
        assert_eq!(
            target_query.parameters[0].value,
            serde_json::Value::Number(serde_json::Number::from(42))
        );
        assert_eq!(target_query.parameters[1].name, "@name");
        assert_eq!(
            target_query.parameters[1].value,
            serde_json::Value::String("Contoso".to_string())
        );

        Ok(())
    }
}
