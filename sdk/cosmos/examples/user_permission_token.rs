use azure_core::HttpClient;
use azure_cosmos::prelude::*;
use std::error::Error;
use std::sync::Arc;

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
    let user_name = std::env::args()
        .nth(3)
        .expect("please specify the user name as third command line parameter");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let client = CosmosClient::new(http_client, account.clone(), authorization_token);

    let database_client = client.clone().into_database_client(database_name.clone());
    let collection_client = database_client
        .clone()
        .into_collection_client(collection_name.clone());
    let user_client = database_client.into_user_client(user_name);

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
    let permission_client = user_client.clone().into_permission_client("matrix");

    let permission_mode = get_collection_response.collection.read_permission();

    let create_permission_response = permission_client
        .create_permission()
        .expiry_seconds(18000u64) // 5 hours, max!
        .execute(&permission_mode)
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
        .into();

    println!(
        "Replacing authorization_token with {:?}.",
        new_authorization_token
    );
    let mut client = client.clone();
    client.auth_token(new_authorization_token);

    // let's list the documents with the new auth token
    let list_documents_response = client
        .clone()
        .into_database_client(database_name.clone())
        .into_collection_client(collection_name.clone())
        .list_documents()
        .execute::<serde_json::Value>()
        .await
        .unwrap();
    println!(
        "second list_documents_response got {} document(s).",
        list_documents_response.documents.len()
    );

    // Now we try to insert a document with the "read-only"
    // authorization_token just created. It will fail.
    // The collection should have /id as partition key
    // for this example to work.
    let data = r#"
        {
            "id": "Gianluigi Bombatomica",
            "age": 43,
            "phones": [
                "+39 1234567",
                "+39 2345678"
            ]
        }"#;
    let document = serde_json::from_str::<serde_json::Value>(data)?;

    match client
        .clone()
        .into_database_client(database_name.clone())
        .into_collection_client(collection_name.clone())
        .create_document()
        .is_upsert(true)
        .execute_with_partition_key(&document, &"Gianluigi Bombatomica")
        .await
    {
        Ok(_) => panic!("this should not happen!"),
        Err(error) => println!("Insert failed: {:#?}", error),
    }

    permission_client.delete_permission().execute().await?;

    // All includes read and write.
    let permission_mode = get_collection_response.collection.all_permission();
    let create_permission_response = permission_client
        .create_permission()
        .expiry_seconds(18000u64) // 5 hours, max!
        .execute(&permission_mode)
        .await?;
    println!(
        "create_permission_response == {:#?}",
        create_permission_response
    );

    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .into();

    println!(
        "Replacing authorization_token with {:?}.",
        new_authorization_token
    );
    client.auth_token(new_authorization_token);

    // now we have an "All" authorization_token
    // so the create_document should succeed!
    let create_document_response = client
        .into_database_client(database_name)
        .into_collection_client(collection_name)
        .create_document()
        .is_upsert(true)
        .execute_with_partition_key(&document, &"Gianluigi Bombatomica")
        .await?;
    println!(
        "create_document_response == {:#?}",
        create_document_response
    );

    println!("Cleaning up user.");
    let delete_user_response = user_client.delete_user().execute().await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
