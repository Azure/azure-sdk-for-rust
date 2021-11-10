use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use http::{HeaderMap, HeaderValue};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("Please provide the database name as first parameter");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let database_client = client.into_database_client(database_name.clone());

    let mut context = Context::new();

    // Next we create a CustomHeaders type and insert it into the context allowing us to insert custom headers.
    let custom_headers: CustomHeaders = {
        let mut custom_headers = HeaderMap::new();
        custom_headers.insert("MyCoolHeader", HeaderValue::from_static("CORS maybe?"));
        custom_headers.into()
    };

    context.insert(custom_headers);

    let response = database_client
        .get_database(context, GetDatabaseOptions::new())
        .await?;
    println!("response == {:?}", response);

    Ok(())
}
