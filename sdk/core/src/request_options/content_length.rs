use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct ContentLength(i32);

impl ContentLength {
    pub fn new(count: i32) -> Self {
        Self(count)
    }
}

impl AddAsHeader for ContentLength {
    fn add_as_header(&self, builder: Builder) -> Builder {
        if self.0 <= 0 {
            builder.header(http::header::CONTENT_LENGTH, -1)
        } else {
            builder.header(http::header::CONTENT_LENGTH, self.0)
        }
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
        if self.0 >= 0 {
            let (header_name, header_value) = (http::header::CONTENT_LENGTH, self.0);
            request
                .headers_mut()
                .append(header_name, http::HeaderValue::from(header_value));
        };

        Ok(())
    }
}
