use azure_core::ClientOptions;

/// Options used when creating a [`CosmosClient`](crate::CosmosClient).
///
/// NOTE: There are currently no options to set on this type.
/// It exists to enable future extensibility.
#[derive(Clone, Debug, Default)]
pub struct CosmosClientOptions {
    pub(crate) client_options: ClientOptions,
}

impl CosmosClientOptions {
    /// Creates a new [`CosmosClientOptionsBuilder`](builders::CosmosClientOptionsBuilder) that can be used to construct a [`CosmosClientOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::ReadDatabaseOptions::builder().build();
    /// ```
    pub fn builder() -> builders::CosmosClientOptionsBuilder {
        builders::CosmosClientOptionsBuilder::default()
    }
}

/// Options to be passed to [`DatabaseClientMethods::read()`](crate::DatabaseClientMethods::read()).
///
/// NOTE: There are currently no options to set on this type.
/// It exists to enable future extensibility.
#[derive(Clone, Debug, Default)]
pub struct ReadDatabaseOptions {}

impl ReadDatabaseOptions {
    /// Creates a new [`ReadDatabaseOptionsBuilder`](builders::ReadDatabaseOptionsBuilder) that can be used to construct a [`ReadDatabaseOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::ReadDatabaseOptions::builder().build();
    /// ```
    pub fn builder() -> builders::ReadDatabaseOptionsBuilder {
        builders::ReadDatabaseOptionsBuilder::default()
    }
}

/// Builders for Cosmos-related options structs.
pub mod builders {
    use crate::{CosmosClientOptions, ReadDatabaseOptions};

    /// Builder used to construct a [`CosmosClientOptions`].
    #[derive(Default)]
    pub struct CosmosClientOptionsBuilder(CosmosClientOptions);

    impl CosmosClientOptionsBuilder {
        /// Builds a [`CosmosClientOptions`] object from the builder.
        ///
        /// This does not consume the builder, and can be called multiple times.
        pub fn build(&self) -> CosmosClientOptions {
            self.0.clone()
        }
    }

    /// Builder used to construct a [`ReadDatabaseOptions`].
    #[derive(Default)]
    pub struct ReadDatabaseOptionsBuilder(ReadDatabaseOptions);

    impl ReadDatabaseOptionsBuilder {
        /// Builds a [`CosmosClientOptions`] object from the builder.
        ///
        /// This does not consume the builder, and can be called multiple times.
        pub fn build(&self) -> ReadDatabaseOptions {
            self.0.clone()
        }
    }
}
