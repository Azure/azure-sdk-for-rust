#![cfg(all(test, feature = "test_e2e"))]

use std::assert_eq;

use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;

mod setup;

#[tokio::test]
async fn users() {
    const DATABASE_NAME: &str = "cosmos-test-db-users";
    const USER_NAME: &str = "someone@cool.net";
    const USER_NAME_REPLACED: &str = "someone.else@cool.net";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database(DATABASE_NAME)
        .into_future()
        .await
        .unwrap();

    let databases = Box::pin(client.list_databases().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    assert!(databases.databases.first().unwrap().id == DATABASE_NAME);
    let database_client = client.clone().into_database_client(DATABASE_NAME);
    let user_client = database_client.clone().into_user_client(USER_NAME);

    let _create_user_response = user_client.create_user().into_future().await.unwrap();

    let list_users_response = Box::pin(database_client.list_users().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();

    assert_eq!(list_users_response.users.len(), 1);

    let get_user_response = user_client.get_user().into_future().await;
    assert!(get_user_response.is_ok());
    let retrieved_user = get_user_response.unwrap();
    assert_eq!(retrieved_user.user.id, USER_NAME);

    let _replace_user_response = user_client
        .replace_user(USER_NAME_REPLACED)
        .into_future()
        .await
        .unwrap();

    let list_users_response = Box::pin(database_client.list_users().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(list_users_response.users.len(), 1);

    let user_client = database_client.clone().into_user_client(USER_NAME_REPLACED);

    let _delete_user_response = user_client.delete_user().into_future().await.unwrap();

    let list_users_response = Box::pin(database_client.list_users().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(list_users_response.users.len(), 0);

    // delete the database
    client
        .clone()
        .into_database_client(DATABASE_NAME)
        .delete_database()
        .into_future()
        .await
        .unwrap();

    let _databases = Box::pin(client.list_databases().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
}
