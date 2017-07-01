#![cfg(all(test,feature = "test_e2e"))]

extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod util;
use util::get_from_env;

use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::table::{TableService, BatchItem};
use azure_sdk_for_rust::azure::core::errors::AzureError;

const TEST_TABLE: &'static str = "rtest1";

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Entity {
    PartitionKey: String,
    RowKey: String,
    c: String,
    deleted: Option<String>,
}

#[test]
fn create_table() {
    let client = create_table_service();
    let s = "t123";
    client.create_table(s).unwrap();
}

#[test]
fn insert_get() {
    // env_logger::init().unwrap();
    let client = create_table_service();
    let utc = chrono::UTC::now();
    let ref s = utc.to_string();
    let ref entity1 = &Entity {
        PartitionKey: "e1".to_owned(),
        RowKey: s.to_owned(),
        c: "mot1".to_owned(),
        deleted: Some("DELET".to_owned()),
    };
    client.insert_entity("rtest1", entity1).unwrap();
    let entity: Entity = client.get_entity("rtest1", "e1", s).unwrap().unwrap();
    assert_eq!("mot1", entity.c);
    assert!(entity.deleted.is_some());

    let ref entity2 = &Entity {
        PartitionKey: "e2".to_owned(),
        RowKey: s.to_owned(),
        c: "mot2".to_owned(),
        deleted: None,
    };
    client.insert_entity("rtest1", entity2).unwrap();
    let entity: Entity = client.get_entity("rtest1", "e2", s).unwrap().unwrap();
    assert_eq!("mot2", entity.c);
    assert!(entity.deleted.is_none());
}

#[test]
fn insert_update() {
    let client = create_table_service();
    let utc = chrono::UTC::now();
    let ref s = utc.to_string();
    let mut entity1 = Entity {
        PartitionKey: "e1".to_owned(),
        RowKey: s.to_owned(),
        c: "mot1".to_owned(),
        deleted: Some("DELET".to_owned()),
    };
    client.insert_entity("rtest1", &entity1).unwrap();
    let entity: Entity = client.get_entity("rtest1", "e1", s).unwrap().unwrap();
    assert_eq!("mot1", entity.c);
    assert!(entity.deleted.is_some());

    entity1.c = "mot1edit".to_owned();
    client.update_entity("rtest1", "e1", s, &entity1).unwrap();
    let entity: Entity = client.get_entity("rtest1", "e1", s).unwrap().unwrap();
    assert_eq!("mot1edit", entity.c);
}

#[test]
fn insert_delete() {
    let client = create_table_service();
    let partition_key = "e1";
    let r1 = chrono::UTC::now().to_string();
    let entity1 = Entity {
        PartitionKey: partition_key.to_owned(),
        RowKey: r1.clone(),
        c: "mot1".to_owned(),
        deleted: Some("DELET".to_owned()),
    };
    client.insert_entity("rtest1", &entity1).unwrap();
    let entity: Entity = client
        .get_entity("rtest1", partition_key, r1.as_str())
        .unwrap()
        .unwrap();
    assert_eq!("mot1", entity.c);
    assert!(entity.deleted.is_some());

    client
        .delete_entity(TEST_TABLE, partition_key, r1.as_str())
        .unwrap();
    let result: Option<Entity> = client
        .get_entity(TEST_TABLE, partition_key, r1.as_str())
        .unwrap();
    assert!(result.is_none());
}

#[test]
fn batch() {
    env_logger::init().unwrap();
    let client = create_table_service();
    client.create_table(TEST_TABLE).unwrap();
    let partition_key = "e1";
    let r1 = chrono::UTC::now().to_string();
    let entity1 = Entity {
        PartitionKey: partition_key.to_owned(),
        RowKey: r1.clone(),
        c: "mot1".to_owned(),
        deleted: Some("DELET".to_owned()),
    };
    client.insert_entity(TEST_TABLE, &entity1).unwrap();
    let entity: Entity = client
        .get_entity(TEST_TABLE, partition_key, r1.as_str())
        .unwrap()
        .unwrap();
    assert_eq!("mot1", entity.c);
    assert!(entity.deleted.is_some());

    let r2 = chrono::UTC::now().to_string();
    let entity2 = Entity {
        PartitionKey: partition_key.to_owned(),
        RowKey: r2.clone(),
        c: "mot2".to_owned(),
        deleted: Some("DELET".to_owned()),
    };
    client.insert_entity(TEST_TABLE, &entity2).unwrap();
    let entity: Entity = client
        .get_entity(TEST_TABLE, partition_key, r2.as_str())
        .unwrap()
        .unwrap();
    assert_eq!("mot2", entity.c);
    assert!(entity.deleted.is_some());

    let ref items = [
        BatchItem::new(
            r1.clone(),
            Some(Entity {
                PartitionKey: partition_key.to_owned(),
                RowKey: r1.clone(),
                c: "mot1edit".to_owned(),
                deleted: Some("DELET".to_owned()),
            }),
        ),
        BatchItem::new(r2.clone(), None),
    ];
    client.batch(TEST_TABLE, partition_key, items).unwrap();

    let entity: Entity = client
        .get_entity(TEST_TABLE, partition_key, r1.as_str())
        .unwrap()
        .unwrap();
    assert_eq!("mot1edit", entity.c);
    let result: Option<Entity> = client
        .get_entity(TEST_TABLE, partition_key, r2.as_str())
        .unwrap();
    assert!(result.is_none());
}

#[test]
fn get_non_exist() {
    let client = create_table_service();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    let entity_o: Option<Entity> = client.get_entity("rtest1", "a62", s.as_str()).unwrap();
    assert!(entity_o.is_none());
}

#[test]
fn insert_to_non_exist() {
    let client = create_table_service();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    let ref entity = Entity {
        PartitionKey: "a62".to_owned(),
        RowKey: s.to_owned(),
        c: "c".to_owned(),
        deleted: None,
    };
    assert!(client.insert_entity("nonrtest1", entity).is_err());
}

#[test]
fn query_range() {
    let client = create_table_service();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    for i in 1..5 {
        let key = format!("b{}0", i);
        let tc = Entity {
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
}

fn test_query_range(
    client: &TableService,
    table_name: &str,
    partition_key: &str,
    row_key: &str,
    ge: bool,
    limit: u16,
) -> Result<Vec<Entity>, AzureError> {
    client.query_entities(
        table_name,
        Some(
            format!(
                "$filter=PartitionKey {} '{}' and RowKey le '{}'&$top={}",
                if ge { "ge" } else { "le" },
                partition_key,
                row_key,
                limit
            ).as_str(),
        ),
    )
}

fn create_table_service() -> TableService {
    let azure_storage_account = get_from_env("AZURE_STORAGE_ACCOUNT");
    let azure_storage_key = get_from_env("AZURE_STORAGE_KEY");
    TableService::new(Client::new(
        &azure_storage_account,
        &azure_storage_key,
        true,
    ))
}
