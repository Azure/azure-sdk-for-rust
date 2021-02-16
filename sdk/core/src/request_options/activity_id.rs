use crate::{headers, AddAsHeader};
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct ActivityId<'a>(&'a str);

impl<'a> ActivityId<'a> {
    pub fn new(id: &'a str) -> Self {
        Self(id)
    }
}

impl<'a> AddAsHeader for ActivityId<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::ACTIVITY_ID, self.0)
    }
}
