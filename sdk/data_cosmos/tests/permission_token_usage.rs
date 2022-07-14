#![cfg(all(test, feature = "test_e2e"))]
use azure_data_cosmos::prelude::*;
use collection::*;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

mod setup;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct MySampleStruct {
    id: String,
    age: u32,
    phones: Vec<String>,
}

impl azure_data_cosmos::CosmosEntity for MySampleStruct {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.id.clone()
    }
}

#[tokio::test]
async fn permission_token_usage() {
    const DATABASE_NAME: &str = "cosmos-test-db-permusage";
    const COLLECTION_NAME: &str = "cosmos-test-db-permusage";
    const USER_NAME: &str = "someone@cool.net";
    const PERMISSION: &str = "sdktest";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database(DATABASE_NAME)
        .into_future()
        .await
        .unwrap();

    let database = client.database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    let create_collection_response = database
        .create_collection(COLLECTION_NAME, "/id")
        .offer(Offer::Throughput(400))
        .indexing_policy(indexing_policy)
        .into_future()
        .await
        .unwrap();

    let user = database.user_client(USER_NAME);
    user.create_user().into_future().await.unwrap();

    // create the RO permission
    let permission = user.permission_client(PERMISSION);
    let permission_mode = create_collection_response.collection.read_permission();

    let create_permission_response = permission
        .create_permission(permission_mode)
        .expiry_seconds(18000u64) // 5 hours, max!
        .into_future()
        .await
        .unwrap();

    // change the AuthorizationToken using the token
    // of the permission.
    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .into();
    let client = client.auth_token(new_authorization_token);
    let new_database = client.database_client(DATABASE_NAME);

    // let's list the collection content.
    // This must succeed.
    new_database
        .collection_client(COLLECTION_NAME)
        .list_documents()
        .into_stream::<serde_json::Value>()
        .next()
        .await
        .unwrap()
        .unwrap();

    let new_collection = new_database.collection_client(COLLECTION_NAME);

    // Now we try to insert a document with the "read-only"
    // authorization_token just created. It must fail.
    let document = MySampleStruct {
        id: "Gianluigi Bombatomica".into(),
        age: 43,
        phones: vec!["+39 1234567".into(), "+39 2345678".into()],
    };

    new_collection
        .create_document(document.clone())
        .is_upsert(true)
        .into_future()
        .await
        .unwrap_err();

    permission.delete_permission().into_future().await.unwrap();

    // All includes read and write.
    let permission_mode = create_collection_response.collection.all_permission();
    let create_permission_response = permission
        .create_permission(permission_mode)
        .expiry_seconds(18000u64) // 5 hours, max!
        .into_future()
        .await
        .unwrap();

    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .into();
    let client = client.auth_token(new_authorization_token);
    let new_database = client.database_client(DATABASE_NAME);
    let new_collection = new_database.collection_client(COLLECTION_NAME);

    // now we have an "All" authorization_token
    // so the create_document should succeed!
    let create_document_response = new_collection
        .create_document(document)
        .is_upsert(true)
        .into_future()
        .await
        .unwrap();
    println!(
        "create_document_response == {:#?}",
        create_document_response
    );

    // cleanup
    database.delete_database().into_future().await.unwrap();
}
