use crate::headers::{self, Header};

#[derive(Debug, Clone, Copy)]
pub struct ActivityId<'a>(&'a str);

impl<'a> ActivityId<'a> {
    pub fn new(id: &'a str) -> Self {
        Self(id)
    }
}

impl<'a> Header for ActivityId<'a> {
    fn name(&self) -> headers::HeaderName {
        headers::ACTIVITY_ID.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
