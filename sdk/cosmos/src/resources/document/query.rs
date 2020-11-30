use serde_json::Value;

/// A SQL Query
///
/// You can learn more about how SQL queries work in Cosmos [here](https://docs.microsoft.com/en-us/azure/cosmos-db/sql-query-getting-started).
#[derive(Debug, Serialize)]
pub struct Query<'a> {
    query: &'a str,
    parameters: Vec<Param<'a>>,
}

impl<'a> Query<'a> {
    pub fn new(query: &'a str) -> Self {
        Self::with_params(query, vec![])
    }

    pub fn with_params<T: Into<Vec<Param<'a>>>>(query: &'a str, params: T) -> Self {
        Self {
            query,
            parameters: params.into(),
        }
    }

    pub fn query(&self) -> &'a str {
        self.query
    }

    pub fn params(&self) -> &[Param<'a>] {
        &self.parameters
    }
}

impl<'a> From<&'a str> for Query<'a> {
    fn from(query: &'a str) -> Query<'a> {
        Query::new(query)
    }
}

impl<'a> AsRef<Query<'a>> for Query<'a> {
    fn as_ref(&self) -> &Query<'a> {
        &self
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Param<'a> {
    name: &'a str,
    value: Value,
}

impl<'a> Param<'a> {
    pub fn new<T: Into<Value>>(name: &'a str, value: T) -> Self {
        Self {
            name,
            value: value.into(),
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

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
            "SELECT * FROM t",
            vec![
                Param::new("p1", "string"),
                Param::new("p2", 100u64),
                Param::new("p3", v3),
            ],
        );

        let ser = serde_json::to_string(&query).unwrap();

        assert_eq!(
            ser,
            r#"{"query":"SELECT * FROM t","parameters":[{"name":"p1","value":"string"},{"name":"p2","value":100},{"name":"p3","value":[1,2,3]}]}"#
        );
    }
}
