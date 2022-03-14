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
    let _create_database_response = client.create_database(DATABASE_NAME).await.unwrap();

    let databases = Box::pin(client.list_databases().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    assert!(databases.databases.first().unwrap().id == DATABASE_NAME);
    let database = client.database_client(DATABASE_NAME);
    let user = database.user_client(USER_NAME);

    let _create_user_response = user.create_user().await.unwrap();

    let list_users_response = Box::pin(database.list_users().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();

    assert_eq!(list_users_response.users.len(), 1);

    let get_user_response = user.get_user().await;
    assert!(get_user_response.is_ok());
    let retrieved_user = get_user_response.unwrap();
    assert_eq!(retrieved_user.user.id, USER_NAME);

    let _replace_user_response = user.replace_user(USER_NAME_REPLACED).await.unwrap();

    let list_users_response = Box::pin(database.list_users().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(list_users_response.users.len(), 1);

    let user = database.user_client(USER_NAME_REPLACED);

    let _delete_user_response = user.delete_user().await.unwrap();

    let list_users_response = Box::pin(database.list_users().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(list_users_response.users.len(), 0);

    // delete the database
    client
        .database_client(DATABASE_NAME)
        .delete_database()
        .await
        .unwrap();

    let _databases = Box::pin(client.list_databases().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
}
