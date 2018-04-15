use std::marker::PhantomData;

pub trait Complete {}

pub struct True;
pub struct False;

impl Complete for True {}
impl Complete for False {}

#[derive(Debug, Serialize, Clone)]
pub struct Parameter<'a> {
    name: &'a str,
    value: &'a str,
}

#[derive(Debug, Serialize, Clone)]
pub struct IncompleteParameter<'a, N = False, V = False>
where
    N: Complete,
    V: Complete,
{
    name_completed: PhantomData<N>,
    value_completed: PhantomData<V>,

    parameter: Parameter<'a>,
}

impl<'a> Parameter<'a> {
    pub fn new() -> IncompleteParameter<'a, False, False> {
        IncompleteParameter {
            name_completed: PhantomData,
            value_completed: PhantomData,
            parameter: Parameter {
                name: "",
                value: "",
            },
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn value(&self) -> &'a str {
        self.value
    }
}

impl<'a, N, V> IncompleteParameter<'a, N, V>
where
    N: Complete,
    V: Complete,
{
    pub fn name(self, name: &'a str) -> IncompleteParameter<'a, True, V> {
        IncompleteParameter {
            name_completed: PhantomData,
            value_completed: PhantomData,
            parameter: Parameter {
                name: name,
                value: self.parameter.value,
            },
        }
    }

    pub fn value(self, value: &'a str) -> IncompleteParameter<'a, N, True> {
        IncompleteParameter {
            name_completed: PhantomData,
            value_completed: PhantomData,
            parameter: Parameter {
                name: self.parameter.name,
                value: value,
            },
        }
    }
}

impl<'a> IncompleteParameter<'a, True, True> {
    pub fn build(self) -> Parameter<'a> {
        self.parameter
    }
}

#[derive(Debug, Serialize)]
pub struct Query<'a> {
    query: &'a str,
    parameters: Vec<Parameter<'a>>,
}

impl<'a> Query<'a> {
    pub fn new(query: &'a str) -> Query<'a> {
        Query {
            query: query,
            parameters: Vec::new(),
        }
    }

    pub fn set_query(&mut self, query: &'a str) {
        self.query = query;
    }

    pub fn parameters_mut(&mut self) -> &mut Vec<Parameter<'a>> {
        &mut self.parameters
    }

    pub fn query(&self) -> &'a str {
        self.query
    }

    pub fn parameters(&self) -> &[Parameter<'a>] {
        &self.parameters
    }
}

impl<'a> From<&'a str> for Query<'a> {
    fn from(query: &'a str) -> Query<'a> {
        Query::new(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[derive(Debug)]
    struct Test {}

    impl<'a> Into<&'a str> for Test {
        fn into(self) -> &'a str {
            "palazzo"
        }
    }

    #[test]
    fn tst_query() {
        let p1 = Parameter::new().name("p1").value("string").build();

        let val = &100u64.to_string() as &str;
        let p2 = Parameter::new().name("p2").value(val).build();

        let t = Test {};

        let p3 = Parameter::new().name("p3").value(t.into()).build();

        let mut query = Query::new("SELECT * FROM Table");

        query.parameters_mut().push(p1);
        query.parameters_mut().push(p2);
        query.parameters_mut().push(p3);

        let ser = serde_json::to_string(&query).unwrap();

        assert_eq!(
            ser,
            r#"{"query":"SELECT * FROM Table","parameters":[{"name":"p1","value":"string"},{"name":"p2","value":"100"},{"name":"p3","value":"palazzo"}]}"#
        );
    }
}
