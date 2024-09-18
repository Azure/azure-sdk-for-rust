#[cfg(doc)]
use crate::clients::ContainerClientMethods;

/// Options to be passed to [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
#[derive(Clone, Debug, Default)]
pub struct QueryOptions {}

impl QueryOptions {
    /// Creates a new [`QueryOptionsBuilder`](QueryOptionsBuilder) that can be used to construct a [`QueryOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::QueryOptions::builder().build();
    /// ```
    pub fn builder() -> QueryOptionsBuilder {
        QueryOptionsBuilder::default()
    }
}

/// Builder used to construct a [`QueryOptions`].
///
/// Obtain a [`QueryOptionsBuilder`] by calling [`QueryOptions::builder()`]
#[derive(Default)]
pub struct QueryOptionsBuilder(QueryOptions);

impl QueryOptionsBuilder {
    /// Builds a [`QueryOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> QueryOptions {
        self.0.clone()
    }
}
