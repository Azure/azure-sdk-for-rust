use azure_core::error::Result;
use azure_core::headers::{HeaderName, HeaderValue, Headers};
use azure_core::prelude::*;
use azure_core::CustomHeaders;
use azure_data_cosmos::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
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

    let database = client.database_client(database_name.clone());

    let mut context = Context::new();

    // Next we create a CustomHeaders type and insert it into the context allowing us to insert custom headers.
    let custom_headers: CustomHeaders = {
        let mut custom_headers = std::collections::HashMap::<HeaderName, HeaderValue>::new();
        custom_headers.insert("MyCoolHeader".into(), "CORS maybe?".into());
        let hs: Headers = custom_headers.into();
        hs.into()
    };

    context.insert(custom_headers);

    let response = database
        .get_database()
        .context(context)
        .into_future()
        .await?;
    println!("response == {:?}", response);

    Ok(())
}
