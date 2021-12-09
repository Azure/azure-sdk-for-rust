use azure_core::{Context, Error};
use azure_storage::table::prelude::*;
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
struct UserEntityExtended<'a> {
    #[serde(rename = "PartitionKey")]
    pub city: &'a str,
    #[serde(rename = "RowKey")]
    pub surname: &'a str,
    pub age: u8,
}

impl<'a> TableEntity<'a> for UserEntityExtended<'a> {
    type Entity = Self;

    fn partition_key(&self) -> &str {
        self.city
    }

    fn row_key(&self) -> &str {
        self.surname
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // in the following example we will interact with the user table. first let's create the table if it doesn't exists;
    let table_name = "users";
    let users = vec![
        UserEntity::new("beit dagan", "shem tov", "or"),
        UserEntity::new("rishon lezion", "gerbil", "yaron"),
        UserEntity::new("poria neve oved", "sachanov", "shay"),
    ];

    let table_client = create_client();

    //create user table if not exists;
    if let Some(_) = create_if_not_exist(&table_client, table_name).await? {
        println!("{} table created successfully.", table_name);
    } else {
        println!("{} already exists.", table_name);
    }

    let entity_client = table_client.into_entity_client(table_name);

    // insert users into users table;
    for user in users.iter() {
        let entity = entity_client
            .insert_entity(
                Context::new(),
                user,
                InsertEntityOptions::default()
                    .echo_content(EchoContent::ReturnContent)
                    .odata_metadata_level(OdataMetadataLevel::NoMetadata),
            )
            .await?;
        println!("insert operation completed successfully - {:?}", entity);
    }

    // print users from the table using partition_key and row_key;
    for user in users.iter() {
        println!(
            "trying to get user entity with partition_key: {} and row_key: {}",
            user.partition_key(),
            user.row_key()
        );

        let entity = entity_client
            .query_entity::<UserEntity>(
                Context::new(),
                user.partition_key(),
                user.row_key(),
                QueryEntityOptions::default()
                    .odata_metadata_level(OdataMetadataLevel::NoMetadata)
                    .timeout(12),
            )
            .await?;
        println!("Entity found - {:#?}", entity);

        // update entity by adding new column
        let _ = entity_client
            .insert_or_replace_entity::<UserEntityExtended>(
                Context::new(),
                &UserEntityExtended {
                    city: entity.entity.city.as_ref().unwrap(),
                    surname: user.surname.as_ref().unwrap(),
                    age: 30,
                },
                InsertOrReplaceEntityOptions::default(),
            )
            .await
            .map_err(|err| println!("error in update entity. error details: {:#?}", err));
        println!("Entity updated successfully");
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
        println!("Entity deleted successfully - {:?}", user);
    }

    TableClient::emulator(TableOptions::default())
        .delete_table(Context::new(), table_name, DeleteTableOptions::default())
        .await?;
    println!("Table {:#?} deleted successfully", table_name);

    Ok(())
}

fn create_client() -> TableClient {
    let account = std::env::vars()
        .find(|i| i.0 == "STORAGE_ACCOUNT")
        .map(|i| i.1);
    let key = std::env::vars()
        .find(|i| i.0 == "STORAGE_MASTER_KEY")
        .map(|i| i.1);

    let auth_token = match (account.as_ref(), key) {
        (Some(account), Some(key)) => Some(AuthorizationToken::SharedKeyToken {
            account: account.to_owned(),
            key,
        }),
        _ => None,
    };

    if let Some(auth_token) = auth_token {
        TableClient::new(
            account.clone().unwrap(),
            auth_token.clone(),
            TableOptions::default(),
        )
    } else {
        TableClient::emulator(TableOptions::default())
    }
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
