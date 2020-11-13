use azure_core::errors::CosmosError;
use azure_cosmos::clients::DefaultCosmosUri;
use azure_cosmos::prelude::*;
use azure_cosmos::AuthorizationToken;

pub fn initialize() -> Result<CosmosStruct<'static, DefaultCosmosUri>, CosmosError> {
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new_master(&key)?;
    let client = ClientBuilder::new(account, authorization_token)?;

    Ok(client)
}
