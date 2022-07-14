use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::StreamExt;

#[derive(Debug, Parser)]
struct Args {
    /// Cosmos primary key name
    #[clap(env = "COSMOS_PRIMARY_KEY")]
    primary_key: String,
    /// The cosmos account your're using
    #[clap(env = "COSMOS_ACCOUNT")]
    account: String,
    /// The name of the database
    database_name: String,
    /// The name of the user
    user_name: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    // We expect access keys (ie, not resource constrained)
    let args = Args::parse();
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    let client = CosmosClient::new(args.account, authorization_token);
    let database = client.database_client(args.database_name);
    let user = database.user_client(args.user_name.clone());

    let create_user_response = user.create_user().into_future().await?;
    println!("create_user_response == {:#?}", create_user_response);

    let users = database.list_users().into_stream().next().await.unwrap()?;

    println!("list_users_response == {:#?}", users);

    let get_user_response = user.get_user().into_future().await?;
    println!("get_user_response == {:#?}", get_user_response);

    let new_user = format!("{}replaced", args.user_name);

    let replace_user_response = user.replace_user(new_user.clone()).into_future().await?;
    println!("replace_user_response == {:#?}", replace_user_response);

    let user = database.user_client(new_user);

    let delete_user_response = user.delete_user().into_future().await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
