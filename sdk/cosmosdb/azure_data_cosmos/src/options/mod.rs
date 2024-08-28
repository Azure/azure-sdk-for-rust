use azure_core::ClientOptions;

#[derive(Clone, Debug, Default)]
pub struct CosmosClientOptions {
    pub(crate) client_options: ClientOptions,
}

impl CosmosClientOptions {
    pub fn builder() -> builders::CosmosClientOptionsBuilder {
        builders::CosmosClientOptionsBuilder::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ReadDatabaseOptions {}

impl ReadDatabaseOptions {}

pub mod builders {
    use crate::{CosmosClientOptions, ReadDatabaseOptions};

    #[derive(Default)]
    pub struct CosmosClientOptionsBuilder(CosmosClientOptions);

    impl CosmosClientOptionsBuilder {
        pub fn build(&self) -> CosmosClientOptions {
            self.0.clone()
        }
    }

    #[derive(Default)]
    pub struct ReadDatabaseOptionsBuilder(ReadDatabaseOptions);

    impl ReadDatabaseOptionsBuilder {
        pub fn build(&self) -> ReadDatabaseOptions {
            self.0.clone()
        }
    }
}
