use futures::stream::StreamExt;

mod setup_mock;

#[tokio::test]
async fn user_operations() {
    const DATABASE_NAME: &str = "cosmos-test-db-users";
    const USER_NAME: &str = "someone@cool.net";
    const USER_NAME_REPLACED: &str = "someone.else@cool.net";

    let client = setup_mock::initialize("user_operations").unwrap();

    // create a temp database
    let _ = client
        .create_database(DATABASE_NAME)
        .into_future()
        .await
        .unwrap();

    let database = client.database_client(DATABASE_NAME);
    let user = database.user_client(USER_NAME);

    let _ = user.create_user().into_future().await.unwrap();

    let list_users_response = database
        .list_users()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap();

    assert_eq!(list_users_response.users.len(), 1);

    let retrieved_user = user.get_user().into_future().await.unwrap();
    assert_eq!(retrieved_user.user.id, USER_NAME);

    let _ = user
        .replace_user(USER_NAME_REPLACED)
        .into_future()
        .await
        .unwrap();

    let list_users_response = database
        .list_users()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(list_users_response.users.len(), 1);

    let user = database.user_client(USER_NAME_REPLACED);

    let _ = user.delete_user().into_future().await.unwrap();

    let list_users_response = database
        .list_users()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(list_users_response.users.len(), 0);

    // delete the database
    client
        .database_client(DATABASE_NAME)
        .delete_database()
        .into_future()
        .await
        .unwrap();
}
