use serde_json::Value;

/// A SQL Query
///
/// You can learn more about how SQL queries work in Cosmos [here](https://docs.microsoft.com/azure/cosmos-db/sql-query-getting-started).
#[derive(Debug, Serialize, Clone)]
pub struct Query {
    query: String,
    parameters: Vec<Param>,
}

impl Query {
    /// A new SQL query with no parameters
    #[must_use]
    pub fn new(query: String) -> Self {
        Self::with_params(query, vec![])
    }

    /// A new SQL query with the supplied parameters
    pub fn with_params<T: Into<Vec<Param>>>(query: String, params: T) -> Self {
        Self {
            query,
            parameters: params.into(),
        }
    }

    /// The query as a `&str`
    #[must_use]
    pub fn query(&self) -> &str {
        &self.query
    }

    /// The supplied params
    #[must_use]
    pub fn params(&self) -> &[Param] {
        &self.parameters
    }
}

impl From<&'static str> for Query {
    fn from(query: &'static str) -> Query {
        Query::new(query.into())
    }
}

/// A SQL query parameter
#[derive(Debug, Serialize, Clone)]
pub struct Param {
    name: String,
    value: Value,
}

impl Param {
    /// Create a new `Param` with the supplied name and the JSON value
    pub fn new<T: Into<Value>>(name: String, value: T) -> Self {
        Self {
            name,
            value: value.into(),
        }
    }

    /// The param name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The param value
    #[must_use]
    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn tst_query() {
        let v3 = Value::from(vec![1, 2, 3]);
        let query = Query::with_params(
            "SELECT * FROM t".into(),
            vec![
                Param::new("p1".into(), "string"),
                Param::new("p2".into(), 100u64),
                Param::new("p3".into(), v3),
            ],
        );

        let ser = serde_json::to_string(&query).unwrap();

        assert_eq!(
            ser,
            r#"{"query":"SELECT * FROM t","parameters":[{"name":"p1","value":"string"},{"name":"p2","value":100},{"name":"p3","value":[1,2,3]}]}"#
        );
    }
}
