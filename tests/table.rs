#![cfg(all(test,feature = "e2e_test"))]

extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
extern crate rustc_serialize;

use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::table::{TableClient, BatchItem};
use azure_sdk_for_rust::azure::core::errors::AzureError;

const TEST_TABLE: &'static str = "rtest1";

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Entry {
    PartitionKey: String,
    RowKey: String,
    c: String,
    deleted: Option<String>,
}

#[test]
fn insert_get() {
    // env_logger::init().unwrap();
    let client = create_table_client();
    let utc = chrono::UTC::now();
    let ref s = utc.to_string();
    let ref entity1 = &Entry {
                           PartitionKey: "e1".to_owned(),
                           RowKey: s.to_owned(),
                           c: "mot1".to_owned(),
                           deleted: Some("DELET".to_owned()),
                       };
    client.insert_entity("rtest1", entity1).unwrap();
    let entry: Entry = client.get_entity("rtest1", "e1", s).unwrap().unwrap();
    assert_eq!("mot1", entry.c);
    assert!(entry.deleted.is_some());

    let ref entry2 = &Entry {
                          PartitionKey: "e2".to_owned(),
                          RowKey: s.to_owned(),
                          c: "mot2".to_owned(),
                          deleted: None,
                      };
    client.insert_entity("rtest1", entry2).unwrap();
    let entry: Entry = client.get_entity("rtest1", "e2", s).unwrap().unwrap();
    assert_eq!("mot2", entry.c);
    assert!(entry.deleted.is_none());
}

#[test]
fn insert_update() {
    // env_logger::init().unwrap();
    let client = create_table_client();
    let utc = chrono::UTC::now();
    let ref s = utc.to_string();
    let mut entity1 = Entry {
        PartitionKey: "e1".to_owned(),
        RowKey: s.to_owned(),
        c: "mot1".to_owned(),
        deleted: Some("DELET".to_owned()),
    };
    client.insert_entity("rtest1", &entity1).unwrap();
    let entry: Entry = client.get_entity("rtest1", "e1", s).unwrap().unwrap();
    assert_eq!("mot1", entry.c);
    assert!(entry.deleted.is_some());

    entity1.c = "mot1edit".to_owned();
    client.update_entity("rtest1", "e1", s, &entity1).unwrap();
    let entry: Entry = client.get_entity("rtest1", "e1", s).unwrap().unwrap();
    assert_eq!("mot1edit", entry.c);
}

#[test]
fn insert_delete() {
    // env_logger::init().unwrap();
    let client = create_table_client();
    let partition_key = "e1";
    let r1 = chrono::UTC::now().to_string();
    let entity1 = Entry {
        PartitionKey: partition_key.to_owned(),
        RowKey: r1.clone(),
        c: "mot1".to_owned(),
        deleted: Some("DELET".to_owned()),
    };
    client.insert_entity("rtest1", &entity1).unwrap();
    let entry: Entry = client.get_entity("rtest1", partition_key, r1.as_str()).unwrap().unwrap();
    assert_eq!("mot1", entry.c);
    assert!(entry.deleted.is_some());

    client.delete_entity(TEST_TABLE, partition_key, r1.as_str()).unwrap();
    let result: Option<Entry> = client.get_entity(TEST_TABLE, partition_key, r1.as_str()).unwrap();
    assert!(result.is_none());
}

#[test]
fn batch() {
    env_logger::init().unwrap();
    let client = create_table_client();
    client.create_if_not_exists(TEST_TABLE).unwrap();
    let partition_key = "e1";
    let r1 = chrono::UTC::now().to_string();
    let entity1 = Entry {
        PartitionKey: partition_key.to_owned(),
        RowKey: r1.clone(),
        c: "mot1".to_owned(),
        deleted: Some("DELET".to_owned()),
    };
    client.insert_entity(TEST_TABLE, &entity1).unwrap();
    let entry: Entry = client.get_entity(TEST_TABLE, partition_key, r1.as_str()).unwrap().unwrap();
    assert_eq!("mot1", entry.c);
    assert!(entry.deleted.is_some());

    let r2 = chrono::UTC::now().to_string();
    let entity2 = Entry {
        PartitionKey: partition_key.to_owned(),
        RowKey: r2.clone(),
        c: "mot2".to_owned(),
        deleted: Some("DELET".to_owned()),
    };
    client.insert_entity(TEST_TABLE, &entity2).unwrap();
    let entry: Entry = client.get_entity(TEST_TABLE, partition_key, r2.as_str()).unwrap().unwrap();
    assert_eq!("mot2", entry.c);
    assert!(entry.deleted.is_some());

    let ref items = [BatchItem::new(r1.clone(),
                                    Some(Entry {
                                             PartitionKey: partition_key.to_owned(),
                                             RowKey: r1.clone(),
                                             c: "mot1edit".to_owned(),
                                             deleted: Some("DELET".to_owned()),
                                         })),
                     BatchItem::new(r2.clone(), None)];
    client.batch(TEST_TABLE, partition_key, items).unwrap();

    let entry: Entry = client.get_entity(TEST_TABLE, partition_key, r1.as_str()).unwrap().unwrap();
    assert_eq!("mot1edit", entry.c);
    let result: Option<Entry> = client.get_entity(TEST_TABLE, partition_key, r2.as_str()).unwrap();
    assert!(result.is_none());
}

#[test]
fn get_non_exist() {
    let client = create_table_client();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    let entry_o: Option<Entry> = client.get_entity("rtest1", "a62", s.as_str()).unwrap();
    assert!(entry_o.is_none());
}

#[test]
fn insert_to_non_exist() {
    let client = create_table_client();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    let ref entity = Entry {
        PartitionKey: "a62".to_owned(),
        RowKey: s.to_owned(),
        c: "c".to_owned(),
        deleted: None,
    };
    assert!(client.insert_entity("nonrtest1", entity).is_err());
}

#[test]
fn create_table() {
    let client = create_table_client();
    let s = "t123";
    client.create_if_not_exists(s).unwrap();
}

#[test]
fn query_range() {
    // env_logger::init().unwrap();
    let client = create_table_client();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    for i in 1..5 {
        let key = format!("b{}0", i);
        let tc = Entry {
            PartitionKey: key.clone(),
            RowKey: s.to_owned(),
            c: format!("val{}", i),
            deleted: None,
        };

        client.insert_entity("rtest1", &tc).unwrap();
    }

    let ec = test_query_range(&client, "rtest1", "b20", s.as_str(), false, 3).unwrap();
    for item in ec {
        println!("{:?}", item);
    }
    // assert_eq!("mot1", entry.c);
}

fn test_query_range(client: &TableClient,
                    table_name: &str,
                    partition_key: &str,
                    row_key: &str,
                    ge: bool,
                    limit: u16)
                    -> Result<Vec<Entry>, AzureError> {
    client.query_range_entity(
                             table_name,
                             Some(format!("$filter=PartitionKey {} '{}' and RowKey le '{}'&$top={}",
                                          if ge { "ge" } else { "le" },
                                          partition_key,
                                          row_key,
                                          limit)
                                          .as_str()))
}

fn create_table_client() -> TableClient {
    let azure_storage_account = get_from_env("AZURE_STORAGE_ACCOUNT");
    let azure_storage_key = get_from_env("AZURE_STORAGE_KEY");
    TableClient::new(Client::new(&azure_storage_account, &azure_storage_key, true))
}

fn get_from_env(varname: &str) -> String {
    match std::env::var(varname) {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set {} env variable first!", varname);
        }
    }
}
