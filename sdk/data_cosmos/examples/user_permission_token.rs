use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::StreamExt;

mod util;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(flatten)]
    auth: util::Auth,
    /// The name of the database
    database_name: String,
    /// The name of the collection
    collection_name: String,
    /// The name of the user
    user_name: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();

    let client = args.auth.into_client()?;
    let database = client.database_client(args.database_name.clone());
    let collection = database.collection_client(args.collection_name.clone());
    let user = database.user_client(args.user_name);

    let get_collection_response = collection.get_collection().into_future().await?;
    println!("get_collection_response == {:#?}", get_collection_response);

    let create_user_response = user.create_user().into_future().await?;
    println!("create_user_response == {:#?}", create_user_response);

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
        .into_future()
        .await
        .unwrap();
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
        .into_future()
        .await
    {
        Ok(_) => panic!("this should not happen!"),
        Err(error) => println!("Insert failed: {:#?}", error),
    }

    permission.delete_permission().into_future().await?;

    // All includes read and write.
    let permission_mode = get_collection_response.collection.all_permission();
    let create_permission_response = permission
        .create_permission(permission_mode)
        .expiry_seconds(18000u64)
        .into_future()
        .await
        .unwrap();
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
        .database_client(args.database_name)
        .collection_client(args.collection_name)
        .create_document(document)
        .is_upsert(true)
        .partition_key(&"Gianluigi Bombatomica")?
        .into_future()
        .await?;
    println!(
        "create_document_response == {:#?}",
        create_document_response
    );

    println!("Cleaning up user.");
    let delete_user_response = user.delete_user().into_future().await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
