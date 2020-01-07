use azure_sdk_cosmos::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("please specify the database name as first command line parameter");
    let user_name = std::env::args()
        .nth(2)
        .expect("please specify the user name as first command line parameter");

    let authorization_token =
        AuthorizationToken::new(account.clone(), TokenType::Master, &master_key)?;

    let client = ClientBuilder::new(authorization_token)?;
    let database_client = client.with_database(&database_name);
    let user_client = database_client.with_user(&user_name);

    //let create_user_response = user_client.create_user().execute().await?;
    //println!("{:#?}", create_user_response);

    let list_users_response = database_client.list_users().execute().await?;
    println!("{:#?}", list_users_response);

    Ok(())
}
