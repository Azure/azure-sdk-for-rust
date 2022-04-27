use crate::headers::{self, Header};

/// The (friendly) name of the user making the request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User(String);

impl User {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl<S> From<S> for User
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Header for User {
    fn name(&self) -> headers::HeaderName {
        headers::USER.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
