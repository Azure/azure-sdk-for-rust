use azure_cosmos::prelude::*;
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

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = CosmosClient::new(account, authorization_token);
    let database_client = client.into_database_client(database_name);
    let user_client = database_client.clone().into_user_client(user_name.clone());

    let create_user_response = user_client.create_user().execute().await?;
    println!("create_user_response == {:#?}", create_user_response);

    let list_users_response = database_client.list_users().execute().await?;
    println!("list_users_response == {:#?}", list_users_response);

    let get_user_response = user_client.get_user().execute().await?;
    println!("get_user_response == {:#?}", get_user_response);

    let new_user = format!("{}replaced", user_name);

    let replace_user_response = user_client
        .replace_user()
        .with_user_name(&new_user)
        .execute()
        .await?;
    println!("replace_user_response == {:#?}", replace_user_response);

    let user_client = database_client.into_user_client(new_user);

    let delete_user_response = user_client.delete_user().execute().await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
