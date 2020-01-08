use azure_sdk_cosmos::prelude::*;
use azure_sdk_cosmos::PermissionMode;
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
        .expect("please specify the user name as second command line parameter");

    let authorization_token =
        AuthorizationToken::new(account.clone(), TokenType::Master, &master_key)?;

    let client = ClientBuilder::new(authorization_token)?;
    let database_client = client.with_database(&database_name);
    let user_client = database_client.with_user(&user_name);

    let get_database_response = database_client.get_database().execute().await?;
    println!("get_database_response == {:#?}", get_database_response);

    let create_user_response = user_client.create_user().execute().await?;
    println!("create_user_response == {:#?}", create_user_response);

    // create the permission!
    let permission_client = user_client.with_permission(&"matrix");
    let permission_mode = PermissionMode::All(get_database_response.database);

    let create_permission_response = permission_client
        .create_permission()
        .with_permission_mode(&permission_mode)
        .execute()
        .await?;
    println!(
        "create_permission_response == {:#?}",
        create_permission_response
    );

    let delete_user_response = user_client.delete_user().execute().await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
