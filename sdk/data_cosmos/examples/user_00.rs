use azure_data_cosmos::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
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

    let database = client.database_client(database_name);
    let user = database.user_client(user_name.clone());

    let create_user_response = user.create_user().into_future().await?;
    println!("create_user_response == {:#?}", create_user_response);

    let users = database.list_users().into_stream().next().await.unwrap()?;

    println!("list_users_response == {:#?}", users);

    let get_user_response = user.get_user().into_future().await?;
    println!("get_user_response == {:#?}", get_user_response);

    let new_user = format!("{}replaced", user_name);

    let replace_user_response = user.replace_user(new_user.clone()).into_future().await?;
    println!("replace_user_response == {:#?}", replace_user_response);

    let user = database.user_client(new_user);

    let delete_user_response = user.delete_user().into_future().await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
