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
    /// The name of the collection
    collection_name: String,
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
    let database = client.database_client(args.database_name.clone());
    let collection = database.collection_client(args.collection_name.clone());
    let user = database.user_client(args.user_name);

    let get_collection_response = collection.get_collection().await?;
    println!("get_collection_response == {get_collection_response:#?}");

    let create_user_response = user.create_user().await?;
    println!("create_user_response == {create_user_response:#?}");

    // test list documents
    let list_documents_response = collection
        .list_documents()
        .into_stream::<serde_json::Value>()
        .next()
        .await
        .unwrap()?;
    println!(
        "list_documents_response got {} document(s).",
        list_documents_response.documents.len()
    );

    // create the first permission!
    let permission = user.permission_client("matrix");

    let permission_mode = get_collection_response.collection.read_permission();

    let create_permission_response = permission
        .create_permission(permission_mode)
        .expiry_seconds(18000u64) // 5 hours, max!
        .await
        .unwrap();
    println!("create_permission_response == {create_permission_response:#?}");

    // change the AuthorizationToken using the token
    // of the permission.
    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .into();

    println!("Replacing authorization_token with {new_authorization_token:?}.");
    let client = client.clone().auth_token(new_authorization_token);

    // let's list the documents with the new auth token
    let list_documents_response = client
        .database_client(args.database_name.clone())
        .collection_client(args.collection_name.clone())
        .list_documents()
        .into_stream::<serde_json::Value>()
        .next()
        .await
        .unwrap()?;
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
        .database_client(args.database_name.clone())
        .collection_client(args.collection_name.clone())
        .create_document(document.clone())
        .is_upsert(true)
        .partition_key(&"Gianluigi Bombatomica")?
        .await
    {
        Ok(_) => panic!("this should not happen!"),
        Err(error) => println!("Insert failed: {error:#?}"),
    }

    permission.delete_permission().await?;

    // All includes read and write.
    let permission_mode = get_collection_response.collection.all_permission();
    let create_permission_response = permission
        .create_permission(permission_mode)
        .expiry_seconds(18000u64)
        .await
        .unwrap();
    println!("create_permission_response == {create_permission_response:#?}");

    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .into();

    println!("Replacing authorization_token with {new_authorization_token:?}.");
    let client = client.auth_token(new_authorization_token);

    // now we have an "All" authorization_token
    // so the create_document should succeed!
    let create_document_response = client
        .database_client(args.database_name)
        .collection_client(args.collection_name)
        .create_document(document)
        .is_upsert(true)
        .partition_key(&"Gianluigi Bombatomica")?
        .await?;
    println!("create_document_response == {create_document_response:#?}");

    println!("Cleaning up user.");
    let delete_user_response = user.delete_user().await?;
    println!("delete_user_response == {delete_user_response:#?}");

    Ok(())
}
