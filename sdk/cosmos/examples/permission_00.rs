use azure_core::prelude::*;
use azure_cosmos::prelude::*;
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
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify the collection name as second command line parameter");
    let collection_name2 = std::env::args()
        .nth(3)
        .expect("please specify the collection name as third command line parameter");
    let user_name = std::env::args()
        .nth(4)
        .expect("please specify the user name as fourth command line parameter");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let database_client = client.into_database_client(database_name);
    let collection_client = database_client
        .clone()
        .into_collection_client(collection_name);
    let collection2_client = database_client
        .clone()
        .into_collection_client(collection_name2);
    let user_client = database_client.clone().into_user_client(user_name);

    let get_database_response = database_client
        .get_database(&mut Context::new(), GetDatabaseOptions::new())
        .await?;
    println!("get_database_response == {:#?}", get_database_response);

    let get_collection_response = collection_client.get_collection().execute().await?;
    println!("get_collection_response == {:#?}", get_collection_response);

    let get_collection2_response = collection2_client.get_collection().execute().await?;
    println!(
        "get_collection2_response == {:#?}",
        get_collection2_response
    );

    let create_user_response = user_client
        .create_user(&mut Context::new(), CreateUserOptions::default())
        .await?;
    println!("create_user_response == {:#?}", create_user_response);

    // create the first permission!
    let permission_client = user_client.clone().into_permission_client("matrix");
    let permission_mode = get_collection_response.collection.read_permission();

    let create_permission_response = permission_client
        .create_permission(
            &mut Context::new(),
            CreatePermissionOptions::new()
                .consistency_level(&create_user_response)
                .expiry_seconds(18000u64),
            &permission_mode,
        )
        .await
        .unwrap();
    println!(
        "create_permission_response == {:#?}",
        create_permission_response
    );

    // create the second permission!
    let permission_client = user_client.clone().into_permission_client("neo".to_owned());
    let permission_mode = get_collection2_response.collection.all_permission();

    let create_permission2_response = permission_client
        .create_permission(
            &mut Context::new(),
            CreatePermissionOptions::new().consistency_level(&create_user_response),
            &permission_mode,
        )
        .await
        .unwrap();

    println!(
        "create_permission2_response == {:#?}",
        create_permission2_response
    );
    let user_client = user_client.clone();

    let list_permissions_response = user_client
        .list_permissions()
        .consistency_level(ConsistencyLevel::Session(
            create_permission2_response.session_token,
        ))
        .execute()
        .await?;
    println!(
        "list_permissions_response == {:#?}",
        list_permissions_response
    );

    let get_permission_response = permission_client
        .get_permission(
            &mut Context::new(),
            GetPermissionOptions::new().consistency_level(ConsistencyLevel::Session(
                list_permissions_response.session_token,
            )),
        )
        .await
        .unwrap();
    println!("get_permission_response == {:#?}", get_permission_response);

    let permission_mode = &get_permission_response.permission.permission_mode;

    // renew permission extending its validity for 60 seconds more.
    let replace_permission_response = permission_client
        .replace_permission(
            &mut Context::new(),
            ReplacePermissionOptions::new()
                .expiry_seconds(600u64)
                .consistency_level(ConsistencyLevel::Session(
                    get_permission_response.session_token,
                )),
            permission_mode,
        )
        .await
        .unwrap();
    println!(
        "replace_permission_response == {:#?}",
        replace_permission_response
    );

    let delete_permission_response = permission_client
        .delete_permission(
            &mut Context::new(),
            DeletePermissionOptions::new().consistency_level(ConsistencyLevel::Session(
                replace_permission_response.session_token,
            )),
        )
        .await
        .unwrap();

    println!(
        "delete_permission_response == {:#?}",
        delete_permission_response
    );

    let delete_user_response = user_client
        .delete_user()
        .consistency_level(ConsistencyLevel::Session(
            delete_permission_response.session_token,
        ))
        .execute()
        .await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
