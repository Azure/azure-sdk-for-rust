use azure_core::HttpClient;
use azure_cosmos::clients::DefaultCosmosUri;
use azure_cosmos::prelude::*;
use azure_cosmos::AuthorizationToken;
use std::sync::Arc;

pub fn initialize() -> Result<CosmosStruct<'static, DefaultCosmosUri>, CosmosError> {
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new_master(&key)?;
    let client = {
        let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
        azure_cosmos::client_builder::build_default_client(account, authorization_token)?
            .with_http_client(http_client)
            .build()
    };

    Ok(client)
}
