#![cfg(all(test, feature = "test_e2e"))]
use azure_cosmos::prelude::*;
use collection::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod setup;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct MySampleStruct<'a> {
    id: Cow<'a, str>,
    age: u32,
    phones: Vec<Cow<'a, str>>,
}

impl<'a> azure_cosmos::CosmosEntity<'a, &'a str> for MySampleStruct<'a> {
    fn partition_key(&'a self) -> &'a str {
        self.id.as_ref()
    }
}

#[tokio::test]
async fn permission_token_usage() {
    const DATABASE_NAME: &str = "cosmos-test-db-permusage";
    const COLLECTION_NAME: &str = "cosmos-test-db-permusage";
    const USER_NAME: &str = "someone@cool.net";
    const PERMISSION: &str = "sdktest";

    let mut client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database()
        .execute(DATABASE_NAME)
        .await
        .unwrap();

    let database_client = client.clone().into_database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    let create_collection_response = database_client
        .create_collection("/id")
        .offer(Offer::Throughput(400))
        .indexing_policy(indexing_policy)
        .execute(COLLECTION_NAME)
        .await
        .unwrap();

    let user_client = database_client.clone().into_user_client(USER_NAME);
    user_client.create_user().execute().await.unwrap();

    // create the RO permission
    let permission_client = user_client.into_permission_client(PERMISSION);
    let permission_mode = create_collection_response.collection.read_permission();

    let create_permission_response = permission_client
        .create_permission()
        .expiry_seconds(18000u64) // 5 hours, max!
        .execute(&permission_mode)
        .await
        .unwrap();

    // change the AuthorizationToken using the token
    // of the permission.
    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .into();
    client.auth_token(new_authorization_token);
    let new_database_client = client.clone().into_database_client(DATABASE_NAME);

    // let's list the collection content.
    // This must succeed.
    new_database_client
        .clone()
        .into_collection_client(COLLECTION_NAME)
        .list_documents()
        .execute::<serde_json::Value>()
        .await
        .unwrap();

    let new_collection_client = new_database_client.into_collection_client(COLLECTION_NAME);

    // Now we try to insert a document with the "read-only"
    // authorization_token just created. It must fail.
    let document = MySampleStruct {
        id: Cow::Borrowed("Gianluigi Bombatomica"),
        age: 43,
        phones: vec![Cow::Borrowed("+39 1234567"), Cow::Borrowed("+39 2345678")],
    };

    new_collection_client
        .create_document()
        .is_upsert(true)
        .execute(&document)
        .await
        .unwrap_err();

    permission_client
        .delete_permission()
        .execute()
        .await
        .unwrap();

    // All includes read and write.
    let permission_mode = create_collection_response.collection.all_permission();
    let create_permission_response = permission_client
        .create_permission()
        .expiry_seconds(18000u64) // 5 hours, max!
        .execute(&permission_mode)
        .await
        .unwrap();

    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .into();
    client.auth_token(new_authorization_token);
    let new_database_client = client.into_database_client(DATABASE_NAME);
    let new_collection_client = new_database_client.into_collection_client(COLLECTION_NAME);

    // now we have an "All" authorization_token
    // so the create_document should succeed!
    let create_document_response = new_collection_client
        .create_document()
        .is_upsert(true)
        .execute(&document)
        .await
        .unwrap();
    println!(
        "create_document_response == {:#?}",
        create_document_response
    );

    // cleanup
    database_client.delete_database().execute().await.unwrap();
}
