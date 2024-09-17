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

/// Options to be passed to [`DatabaseClientMethods::read()`](crate::clients::DatabaseClientMethods::read()).
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
    use azure_core::builders::ClientOptionsBuilder;

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

    impl ClientOptionsBuilder for CosmosClientOptionsBuilder {
        fn with_per_call_policies<P>(mut self, per_call_policies: P) -> Self
        where
            P: Into<Vec<std::sync::Arc<dyn azure_core::Policy>>>,
            Self: Sized,
        {
            self.0
                .client_options
                .set_per_call_policies(per_call_policies);
            self
        }

        fn with_per_try_policies<P>(mut self, per_try_policies: P) -> Self
        where
            P: Into<Vec<std::sync::Arc<dyn azure_core::Policy>>>,
            Self: Sized,
        {
            self.0.client_options.set_per_try_policies(per_try_policies);
            self
        }

        fn with_retry<P>(mut self, retry: P) -> Self
        where
            P: Into<azure_core::RetryOptions>,
            Self: Sized,
        {
            self.0.client_options.set_retry(retry);
            self
        }

        fn with_transport<P>(mut self, transport: P) -> Self
        where
            P: Into<azure_core::TransportOptions>,
            Self: Sized,
        {
            self.0.client_options.set_transport(transport);
            self
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
