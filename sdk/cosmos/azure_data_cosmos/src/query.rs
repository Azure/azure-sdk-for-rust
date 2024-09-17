use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Query {
    query: String,
    parameters: Vec<QueryParameter>,
}

impl Query {
    pub fn with_parameter(self, name: impl Into<String>, value: impl Into<String>) -> Self {
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
    value: String, // TODO: A parameter can be any JSON value. Should we use serde_json::Value or define a custom enum?
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
