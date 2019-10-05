#![cfg(all(test, feature = "test_e2e"))]

mod setup;

#[test]
fn create_and_delete_database() {
    const DATABASE_NAME: &str = "cosmos-test-db-create-and-delete-database";

    let (client, mut core) = setup::initialize().unwrap();

    // list existing databases and remember their number
    let databases = core.run(client.list_databases()).unwrap();
    let database_count_before = databases.len();

    // create a new database and check if the number of DBs increased
    let database = core.run(client.create_database(DATABASE_NAME)).unwrap();
    let databases = core.run(client.list_databases()).unwrap();
    assert!(databases.len() == database_count_before + 1);

    // get the previously created database
    let database_after_get = core.run(client.get_database(DATABASE_NAME)).unwrap();
    assert!(database.rid == database_after_get.rid);

    // delete the database
    core.run(client.delete_database(DATABASE_NAME)).unwrap();
    let databases = core.run(client.list_databases()).unwrap();
    assert!(databases.len() == database_count_before);
}
