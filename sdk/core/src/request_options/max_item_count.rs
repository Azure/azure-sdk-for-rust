use crate::headers;
use crate::AddAsHeader;
use http::request::Builder;

/// The max number of items in the collection
#[derive(Debug, Clone, Copy)]
pub struct MaxItemCount(i32);

impl MaxItemCount {
    /// Create a new `MaxItemCount`
    pub fn new(count: i32) -> Self {
        Self(count)
    }
}

impl AddAsHeader for MaxItemCount {
    fn add_as_header(&self, builder: Builder) -> Builder {
        if self.0 <= 0 {
            builder.header(headers::MAX_ITEM_COUNT, -1)
        } else {
            builder.header(headers::MAX_ITEM_COUNT, self.0)
        }
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        let (header_name, header_value) = if self.0 <= 0 {
            (headers::MAX_ITEM_COUNT, -1)
        } else {
            (headers::MAX_ITEM_COUNT, self.0)
        };

        request
            .headers_mut()
            .append(header_name, http::HeaderValue::from(header_value));

        Ok(())
    }
}
