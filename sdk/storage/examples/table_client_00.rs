use azure_core::{Context, Error};
use azure_storage::{
    operations::{
        create_table::{CreateTableOptions, CreateTableResponse},
        delete_entity::DeleteEntityOptions,
        delete_table::DeleteTableOptions,
        get_entity::QueryEntitiesOptions,
        insert_entity::InsertEntityOptions,
        query_tables::QueryTablesOptions,
        update_entity::UpdateEntityOptions,
        EchoContent, OdataMetadataLevel, TableEntity,
    },
    table::clients::{EntityClient, TableClient, TableOptions},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserEntity {
    #[serde(rename = "PartitionKey")]
    pub city: Option<String>,
    #[serde(rename = "RowKey")]
    pub surname: Option<String>,
    pub name: Option<String>,
}

impl UserEntity {
    pub fn new(city: &str, surname: &str, name: &str) -> Self {
        UserEntity {
            name: Some(name.into()),
            city: Some(city.into()),
            surname: Some(surname.into()),
        }
    }
}

impl<'a> TableEntity<'a> for UserEntity {
    type Entity = Self;

    fn partition_key(&self) -> &str {
        self.city.as_ref().unwrap().as_str()
    }

    fn row_key(&self) -> &str {
        self.surname.as_ref().unwrap().as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserEntityExtended {
    #[serde(rename = "PartitionKey")]
    pub city: String,
    #[serde(rename = "RowKey")]
    pub surname: String,
    pub age: u8,
}

impl<'a> TableEntity<'a> for UserEntityExtended {
    type Entity = Self;

    fn partition_key(&self) -> &str {
        self.city.as_str()
    }

    fn row_key(&self) -> &str {
        self.surname.as_str()
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let table_name = "users";

    let table_client = TableClient::emulator(TableOptions::default());

    //create user table if not exists;
    let _ = create_if_not_exist(&table_client, table_name).await?;

    let entity_client = table_client.into_entity_client(table_name);

    let users = vec![
        UserEntity::new("beit dagan", "shem tov", "or"),
        UserEntity::new("rishon lezion", "gerbil", "yaron"),
        UserEntity::new("poria neve oved", "sachanov", "shay"),
    ];

    // insert users into users table;
    for user in users.iter() {
        let _ = entity_client
            .insert_entity(
                Context::new(),
                user,
                InsertEntityOptions::default()
                    .echo_content(EchoContent::ReturnContent)
                    .odata_metadata_level(OdataMetadataLevel::NoMetadata),
            )
            .await?;
    }

    // print users from the table using partition_key and row_key;
    for user in users.iter() {
        let user = entity_client
            .query_entities::<UserEntity>(
                Context::new(),
                user.partition_key(),
                user.row_key(),
                QueryEntitiesOptions::default(),
            )
            .await?
            .model;

        // update entity by adding new column
        entity_client
            .update_entity::<UserEntityExtended>(
                Context::new(),
                &UserEntityExtended {
                    city: user.city.unwrap(),
                    surname: user.surname.unwrap(),
                    //name: user.name,
                    age: 30,
                },
                UpdateEntityOptions::default(),
            )
            .await?;
    }

    // delete the users table content;
    for user in users.iter() {
        let user = entity_client
            .delete_entity(
                Context::new(),
                user.partition_key(),
                user.row_key(),
                DeleteEntityOptions::default(),
            )
            .await?;
        println!("{:#?}", user);
    }

    Ok(())
}

async fn create_if_not_exist(
    table_client: &TableClient,
    table_name: &str,
) -> Result<Option<CreateTableResponse>, Error> {
    let exists = table_client
        .query_tables(Context::new(), QueryTablesOptions::default())
        .await?
        .tables
        .iter()
        .find(|&t| t.table_name == table_name)
        .is_some();

    if !exists {
        Ok(Some(
            table_client
                .create_table(Context::new(), table_name, CreateTableOptions::default())
                .await?,
        ))
    } else {
        Ok(None)
    }
}
