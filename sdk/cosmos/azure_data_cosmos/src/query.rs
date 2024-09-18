use serde::Serialize;

/// Represents a Cosmos DB Query, with optional parameters.
#[derive(Debug, Serialize)]
pub struct Query {
    query: String,
    parameters: Vec<QueryParameter>,
}

impl Query {
    /// Creates a new [`Query`] with the same text, and parameters, but with the specified parameter added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_data_cosmos::Query;
    ///
    /// let query = Query::from("SELECT * FROM c WHERE c.id = @customer_id")
    ///     .with_parameter("customer_id", 42);
    /// ```
    pub fn with_parameter(
        self,
        name: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Self {
        let parameter = QueryParameter {
            name: name.into(),
            value: value.into(),
        };
        let mut parameters = self.parameters;
        parameters.push(parameter);
        Self { parameters, ..self }
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

#[derive(Debug, Serialize)]
pub struct QueryParameter {
    name: String,
    value: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use crate::Query;

    #[test]
    pub fn serialize_query_without_parameters() {
        let query: Query = "SELECT * FROM c".into();
        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"query":"SELECT * FROM c","parameters":[]}"#);
    }

    #[test]
    pub fn serialize_query_with_string_parameters() {
        let query = Query::from("SELECT * FROM c")
            .with_parameter("name1", "value1")
            .with_parameter("name2", "value2");
        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(
            serialized,
            r#"{"query":"SELECT * FROM c","parameters":[{"name":"name1","value":"value1"},{"name":"name2","value":"value2"}]}"#
        );
    }
}
