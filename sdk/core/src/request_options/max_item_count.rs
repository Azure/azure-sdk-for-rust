use crate::headers::{self, Header};

/// The max number of items in the collection
#[derive(Debug, Clone, Copy)]
pub struct MaxItemCount(i32);

impl MaxItemCount {
    /// Create a new `MaxItemCount`
    pub fn new(count: i32) -> Self {
        Self(count)
    }
}

impl Header for MaxItemCount {
    fn name(&self) -> headers::HeaderName {
        headers::MAX_ITEM_COUNT.into()
    }

    fn value(&self) -> headers::HeaderValue {
        let count = if self.0 <= 0 { -1 } else { self.0 };
        format!("{}", count).into()
    }
}
