use azure_core::{Context, Error, SasError};
use azure_storage::{
    authorization::{
        sas_token::{
            options::{
                table_account_sas_optional_options::TableAccountSasOptionalOptions,
                table_account_sas_permission::{
                    TableAccountSasPermission, TableAccountSasPermissions,
                },
                table_account_sas_resource_type::{
                    TableAccountSasResourceType, TableAccountSasResourceTypes,
                },
            },
            table_account_sas_builder::TableAccountSasBuilder,
            table_sas_query_parameters::TableSasQueryParameters,
        },
        AccountCredential, AuthorizationToken,
    },
    table::{
        clients::{TableClient, TableOptions},
        prelude::{CreateTableOptions, TableEntity},
    },
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    pub id: String,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub city: Option<String>,
}

impl<'a> TableEntity<'a> for UserEntity {
    type Entity = Self;

    fn partition_key(&self) -> &str {
        self.id.as_ref()
    }

    fn row_key(&self) -> &str {
        self.email.as_ref().unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = AccountCredential::new_emulator();

    let sas: String = create_table_sas(&credentials)?.into();
    create_table(credentials.account(), &sas, "users").await?;
    create_table(credentials.account(), &sas, "userss").await?;

    let sas: String = insert_entity_sas(&credentials)?.into();
    create_table(credentials.account(), &sas, "users").await?;
    create_table(credentials.account(), &sas, "userss").await?;

    Ok(())
}

async fn create_table(account: &str, sas: &str, table_name: &str) -> Result<(), Error> {
    let client = TableClient::new(
        account,
        AuthorizationToken::SASToken(sas.to_string()),
        TableOptions::default(),
    );
    let response = client
        .create_table(
            Context::default(),
            table_name,
            CreateTableOptions::default(),
        )
        .await?;
    println!("table {} created using sas token", response.table.name);
    Ok(())
}

fn insert_entity_sas(credentials: &AccountCredential) -> Result<TableSasQueryParameters, SasError> {
    let builder = TableAccountSasBuilder::new(
        Utc::now() + Duration::hours(1),
        TableAccountSasPermissions::new().add_permission(TableAccountSasPermission::Add),
        TableAccountSasResourceTypes::new().add_resource(TableAccountSasResourceType::Object),
    );
    builder.sign(&credentials, &TableAccountSasOptionalOptions::default())
}

fn create_table_sas(credentials: &AccountCredential) -> Result<TableSasQueryParameters, SasError> {
    let builder = TableAccountSasBuilder::new(
        Utc::now() + Duration::hours(1),
        TableAccountSasPermissions::new().add_permission(TableAccountSasPermission::Write),
        TableAccountSasResourceTypes::new().add_resource(TableAccountSasResourceType::Container),
    );
    builder.sign(&credentials, &TableAccountSasOptionalOptions::default())
}
