use crate::headers::{self, Header};

#[derive(Debug, Clone)]
pub struct ActivityId(String);

impl ActivityId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl<S> From<S> for ActivityId
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Header for ActivityId {
    fn name(&self) -> headers::HeaderName {
        headers::ACTIVITY_ID.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
