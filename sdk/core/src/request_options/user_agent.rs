use crate::Header;
use http::header;

#[derive(Debug, Clone, Copy)]
pub struct UserAgent<'a>(&'a str);

impl<'a> UserAgent<'a> {
    pub fn new(agent: &'a str) -> Self {
        Self(agent)
    }
}

impl<'a> Header for UserAgent<'a> {
    fn name(&self) -> &'static str {
        header::USER_AGENT.as_str()
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}
