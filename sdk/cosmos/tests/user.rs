#![cfg(all(test, feature = "test_e2e"))]

mod setup;

#[tokio::test]
async fn users() {
    const DATABASE_NAME: &str = "cosmos-test-db-users";
    const USER_NAME: &str = "someone@cool.net";
    const USER_NAME_REPLACED: &str = "someone.else@cool.net";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database()
        .execute(DATABASE_NAME)
        .await
        .unwrap();

    let database_client = client.clone().into_database_client(DATABASE_NAME);
    let user_client = database_client.clone().into_user_client(USER_NAME);

    let _create_user_response = user_client.create_user().execute().await.unwrap();

    let list_users_response = database_client.list_users().execute().await.unwrap();
    assert_eq!(list_users_response.users.len(), 1);

    let get_user_response = user_client.get_user().execute().await.unwrap();
    assert!(get_user_response.is_some());
    let retrieved_user = get_user_response.unwrap();
    assert_eq!(retrieved_user.user.id, USER_NAME);

    let _replace_user_response = user_client
        .replace_user()
        .user_name(&USER_NAME_REPLACED)
        .execute()
        .await
        .unwrap();

    let list_users_response = database_client.list_users().execute().await.unwrap();
    assert_eq!(list_users_response.users.len(), 1);

    let user_client = database_client.clone().into_user_client(USER_NAME_REPLACED);

    let _delete_user_response = user_client.delete_user().execute().await.unwrap();

    let list_users_response = database_client.list_users().execute().await.unwrap();
    assert_eq!(list_users_response.users.len(), 0);

    // delete the database
    client
        .clone()
        .into_database_client(DATABASE_NAME)
        .delete_database()
        .execute()
        .await
        .unwrap();
    let _databases = client.list_databases().execute().await.unwrap();
}
