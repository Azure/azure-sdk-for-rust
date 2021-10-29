use azure_core::{Context, Error};

use azure_storage::table::{
    clients::{TableClient, TableOptions},
    prelude::{
        entity::{
            delete_entity::DeleteEntityOptions, get_entity::QueryEntityOptions,
            insert_entity::InsertEntityOptions,
            insert_or_replace_entity::InsertOrReplaceEntityOptions, TableEntity,
        },
        table::{
            create_table::{CreateTableOptions, CreateTableResponse},
            delete_table::DeleteTableOptions,
            query_tables::QueryTablesOptions,
        },
        EchoContent, OdataMetadataLevel,
    },
};
use serde::{Deserialize, Serialize};

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
    // in the follwing example we will interact with the user table. first let's create the table if it doesn't exists;
    let users = vec![
        UserEntity::new("beit dagan", "shem tov", "or"),
        UserEntity::new("rishon lezion", "gerbil", "yaron"),
        UserEntity::new("poria neve oved", "sachanov", "shay"),
    ];

    let table_name = "users";
    let table_client = TableClient::emulator(TableOptions::default());

    //create user table if not exists;
    if let Some(table_created) = create_if_not_exist(&table_client, table_name).await? {
        println!("create table response body: {:#?}", table_created);
    }

    let entity_client = table_client.into_entity_client(table_name);

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
        let entity = entity_client
            .query_entity::<UserEntity>(
                Context::new(),
                user.partition_key(),
                user.row_key(),
                QueryEntityOptions::default(),
            )
            .await?;
        //println!("{:#?}", entity);

        // update entity by adding new column
        let _ = entity_client
            .insert_or_replace_entity::<UserEntityExtended>(
                Context::new(),
                &UserEntityExtended {
                    city: entity.entity.city.as_ref().unwrap().to_string(),
                    surname: user.surname.as_ref().unwrap().to_string(),
                    //name: user.name,
                    age: 30,
                },
                InsertOrReplaceEntityOptions::default(),
            )
            .await
            .map_err(|err| println!("error in update entity. error details: {:#?}", err));
    }

    // delete the users table content;
    for user in users.iter() {
        entity_client
            .delete_entity(
                Context::new(),
                user.partition_key(),
                user.row_key(),
                DeleteEntityOptions::default(),
            )
            .await?;
        //println!("{:#?}", user);
    }

    TableClient::emulator(TableOptions::default())
        .delete_table(Context::new(), table_name, DeleteTableOptions::default())
        .await?;

    Ok(())
}

async fn create_if_not_exist(
    table_client: &TableClient,
    table_name: &str,
) -> Result<Option<CreateTableResponse>, Error> {
    let exists = table_client
        .query_tables(
            Context::new(),
            QueryTablesOptions::default().odata_metadata_level(OdataMetadataLevel::NoMetadata),
        )
        .await?
        .tables
        .iter()
        //.inspect(|t| println!("{:#?}", t))
        .find(|&t| t.name == table_name)
        .is_some();

    if !exists {
        Ok(Some(
            table_client
                .create_table(
                    Context::new(),
                    table_name,
                    CreateTableOptions::default()
                        .odata_metadata_level(OdataMetadataLevel::NoMetadata),
                )
                .await?,
        ))
    } else {
        Ok(None)
    }
}
