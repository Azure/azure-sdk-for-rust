use crate::{headers, Header};

#[derive(Debug, Clone, Copy)]
pub struct ActivityId<'a>(&'a str);

impl<'a> ActivityId<'a> {
    pub fn new(id: &'a str) -> Self {
        Self(id)
    }
}

impl<'a> Header for ActivityId<'a> {
    fn name(&self) -> &'static str {
        headers::ACTIVITY_ID
    }

    fn value(&self) -> String {
        self.0.to_owned()
    }
}
