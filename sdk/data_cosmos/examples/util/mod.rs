use azure_data_cosmos::{
    clients::{CosmosClient, CosmosOptions},
    prelude::AuthorizationToken,
};

/// Help for getting authorization information for use in examples.
///
/// This gathers the primary key and cosmos account from environment variables.
/// It can then create a Cosmos client from that. To see how this is done manually,
/// see the `readme` example.
#[derive(clap::Parser, Debug)]
pub struct Auth {
    /// Cosmos primary key
    #[clap(env = "COSMOS_PRIMARY_KEY")]
    primary_key: String,
    /// The cosmos account your're using
    #[clap(env = "COSMOS_ACCOUNT")]
    account: String,
}

impl Auth {
    pub fn into_client(self) -> azure_core::Result<CosmosClient> {
        let token = AuthorizationToken::primary_from_base64(&self.primary_key)?;
        Ok(CosmosClient::new(
            self.account,
            token,
            CosmosOptions::default(),
        ))
    }

    #[allow(unused)]
    pub fn account(&self) -> &String {
        &self.account
    }
}
