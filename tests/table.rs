#![cfg(all(test,feature = "e2e_test"))]

extern crate azure_sdk_for_rust;
extern crate chrono;

use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::table::Table;

#[test]
fn insert_to_table() {
    let client = create_storage_client();
    let utc = chrono::UTC::now(); 
    let s = utc.to_string();
    Table::insert(&client, "rtest1", "a62", s.as_str(), "c", "mot").unwrap();
    let result = Table::query(&client, "rtest1", "a62", s.as_str(), "c").unwrap();
    assert_eq!("mot", result );
}

fn create_storage_client() -> Client {
    let azure_storage_account = get_from_env("AZURE_STORAGE_ACCOUNT");
    let azure_storage_key = get_from_env("AZURE_STORAGE_KEY");
    Client::new(&azure_storage_account, &azure_storage_key, true)
}

fn get_from_env(varname: &str) -> String {
    match std::env::var(varname) {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set {} env variable first!", varname);
        }
    }
}
