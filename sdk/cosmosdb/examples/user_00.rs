use azure_core::Context;
use azure_cosmos::prelude::*;
use futures::stream::StreamExt;
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
        .expect("please specify the database name as first command line parameter");
    let user_name = std::env::args()
        .nth(2)
        .expect("please specify the user name as first command line parameter");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let database_client = client.into_database_client(database_name);
    let user_client = database_client.clone().into_user_client(user_name.clone());

    let create_user_response = user_client
        .create_user(Context::new(), CreateUserOptions::new())
        .await?;
    println!("create_user_response == {:#?}", create_user_response);

    let users = Box::pin(database_client.list_users(Context::new(), ListUsersOptions::new()))
        .next()
        .await
        .unwrap()?;

    println!("list_users_response == {:#?}", users);

    let get_user_response = user_client
        .get_user(Context::new(), GetUserOptions::new())
        .await?;
    println!("get_user_response == {:#?}", get_user_response);

    let new_user = format!("{}replaced", user_name);

    let replace_user_response = user_client
        .replace_user(Context::new(), &new_user, ReplaceUserOptions::new())
        .await?;
    println!("replace_user_response == {:#?}", replace_user_response);

    let user_client = database_client.into_user_client(new_user);

    let delete_user_response = user_client
        .delete_user(Context::new(), DeleteUserOptions::new())
        .await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
