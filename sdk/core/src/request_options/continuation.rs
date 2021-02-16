use crate::{headers, AddAsHeader};
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct Continuation<'a>(&'a str);

impl<'a> Continuation<'a> {
    pub fn new(c: &'a str) -> Self {
        Self(c)
    }
}

impl AddAsHeader for Continuation<'_> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::CONTINUATION, self.0)
    }
}
