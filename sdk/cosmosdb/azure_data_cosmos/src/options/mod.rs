use azure_core::ClientOptions;

#[derive(Clone, Debug, Default)]
pub struct CosmosClientOptions {
    pub(crate) client_options: ClientOptions
}

impl CosmosClientOptions {
    pub fn builder() -> builders::CosmosClientOptionsBuilder {
        builders::CosmosClientOptionsBuilder::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ReadDatabaseOptions {
    pub(crate) if_match: Option<azure_core::Etag>,
    pub(crate) if_none_match: Option<azure_core::Etag>,
}

impl ReadDatabaseOptions {
    pub fn builder() -> builders::ReadDatabaseOptionsBuilder {
        builders::ReadDatabaseOptionsBuilder::default()
    }
}

pub mod builders {
    use crate::{ReadDatabaseOptions, CosmosClientOptions};

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

        pub fn if_match(&mut self, if_match: azure_core::Etag) -> &mut Self {
            self.0.if_match = Some(if_match);
            self
        }

        pub fn if_none_match(&mut self, if_none_match: azure_core::Etag) -> &mut Self {
            self.0.if_none_match = Some(if_none_match);
            self
        }
    }
}
