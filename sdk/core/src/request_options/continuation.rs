use crate::{headers, AddAsHeader};
use http::request::Builder;

#[derive(Debug, Clone)]
pub struct Continuation(String);

impl Continuation {
    pub fn new(c: String) -> Self {
        Self(c)
    }

    pub fn into_raw(self) -> String {
        self.0
    }
}

impl AddAsHeader for Continuation {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::CONTINUATION, &self.0)
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HttpHeaderError> {
        request
            .headers_mut()
            .append(headers::CONTINUATION, http::HeaderValue::from_str(&self.0)?);

        Ok(())
    }
}
