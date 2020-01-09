use azure_sdk_cosmos::prelude::*;
use azure_sdk_cosmos::PermissionMode;
use std::convert::TryInto;
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
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify the collection name as second command line parameter");
    let user_name = std::env::args()
        .nth(3)
        .expect("please specify the user name as third command line parameter");

    let authorization_token = AuthorizationToken::new(TokenType::Master, &master_key)?;

    let client = ClientBuilder::new(account, authorization_token)?;
    let database_client = client.with_database(&database_name);
    let collection_client = database_client.with_collection(&collection_name);
    let user_client = database_client.with_user(&user_name);

    let get_collection_response = collection_client.get_collection().execute().await?;
    println!("get_collection_response == {:#?}", get_collection_response);

    let create_user_response = user_client.create_user().execute().await?;
    println!("create_user_response == {:#?}", create_user_response);

    // test list documents
    let list_documents_response = collection_client
        .list_documents()
        .execute::<serde_json::Value>()
        .await
        .unwrap();
    println!(
        "list_documents_response got {} document(s).",
        list_documents_response.documents.len()
    );

    // create the first permission!
    let permission_client = user_client.with_permission(&"matrix");

    let permission_mode = PermissionMode::Read(get_collection_response.collection);

    let create_permission_response = permission_client
        .create_permission()
        .with_permission_mode(&permission_mode)
        .with_expiry_seconds(18000) // 5 hours, max!
        .execute()
        .await?;
    println!(
        "create_permission_response == {:#?}",
        create_permission_response
    );

    // change the AuthorizationToken using the token
    // of the permission.
    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .try_into()
        .unwrap();
    client.set_auth_token(new_authorization_token);

    let delete_user_response = user_client.delete_user().execute().await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
