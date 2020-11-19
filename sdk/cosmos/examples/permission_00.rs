use azure_cosmos::prelude::*;
use azure_cosmos::PermissionMode;
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
    let collection_name2 = std::env::args()
        .nth(3)
        .expect("please specify the collection name as third command line parameter");
    let user_name = std::env::args()
        .nth(4)
        .expect("please specify the user name as fourth command line parameter");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = CosmosClient::new(account, authorization_token);
    let database_client = client.into_database_client(database_name);
    let collection_client = database_client
        .clone()
        .into_collection_client(collection_name);
    let collection2_client = database_client
        .clone()
        .into_collection_client(collection_name2);
    let user_client = database_client.clone().into_user_client(user_name);

    let get_database_response = database_client.get_database().execute().await?;
    println!("get_database_response == {:#?}", get_database_response);

    let get_collection_response = collection_client.get_collection().execute().await?;
    println!("get_collection_response == {:#?}", get_collection_response);

    let get_collection2_response = collection2_client.get_collection().execute().await?;
    println!(
        "get_collection2_response == {:#?}",
        get_collection2_response
    );

    let create_user_response = user_client.create_user().execute().await?;
    println!("create_user_response == {:#?}", create_user_response);

    // create the first permission!
    let permission_client = user_client.clone().into_permission_client("matrix");
    let permission_mode = PermissionMode::Read(get_collection_response.collection);

    let create_permission_response = permission_client
        .create_permission()
        .with_consistency_level((&create_user_response).into())
        .with_expiry_seconds(18000) // 5 hours, max!
        .execute_with_permission(&permission_mode)
        .await?;
    println!(
        "create_permission_response == {:#?}",
        create_permission_response
    );

    // create the second permission!
    let permission_client = user_client.clone().into_permission_client("neo".to_owned());
    let permission_mode = PermissionMode::All(get_collection2_response.collection);

    let create_permission2_response = permission_client
        .create_permission()
        .with_consistency_level((&create_user_response).into())
        .execute_with_permission(&permission_mode)
        .await?;
    println!(
        "create_permission2_response == {:#?}",
        create_permission2_response
    );
    let user_client = user_client.clone();

    let list_permissions_response = user_client
        .list_permissions()
        .with_consistency_level(ConsistencyLevel::from(
            &create_permission2_response.session_token,
        ))
        .execute()
        .await?;
    println!(
        "list_permissions_response == {:#?}",
        list_permissions_response
    );

    let get_permission_response = permission_client
        .get_permission()
        .with_consistency_level(ConsistencyLevel::from(
            &list_permissions_response.session_token,
        ))
        .execute()
        .await?;
    println!("get_permission_response == {:#?}", get_permission_response);

    let get_permission_response = get_permission_response.unwrap();

    // renew permission extending its validity for 60 seconds more.
    let replace_permission_response = permission_client
        .replace_permission()
        .with_expiry_seconds(60)
        .with_consistency_level(ConsistencyLevel::from(
            &get_permission_response.session_token,
        ))
        .execute_with_permission(&get_permission_response.permission.permission_mode)
        .await?;
    println!(
        "replace_permission_response == {:#?}",
        replace_permission_response
    );

    let delete_permission_response = permission_client
        .delete_permission()
        .with_consistency_level(ConsistencyLevel::from(
            &replace_permission_response.session_token,
        ))
        .execute()
        .await?;
    println!(
        "delete_permission_response == {:#?}",
        delete_permission_response
    );

    let delete_user_response = user_client
        .delete_user()
        .with_consistency_level(ConsistencyLevel::from(
            &delete_permission_response.session_token,
        ))
        .execute()
        .await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
