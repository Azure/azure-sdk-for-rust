use azure_core::{Error, SasError};
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
                table_sas_ip_option::TableSasIpOption,
                table_sas_protocol::TableSasProtocol,
            },
            table_account_sas_builder::TableAccountSasBuilder,
            table_sas_query_parameters::TableSasQueryParameters,
        },
        AccountCredential,
    },
    table::prelude::TableEntity,
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
async fn main() -> Result<(), Error> {
    let sas = generate_sas().unwrap();
    println!("{:#?}", sas);

    Ok(())
}

fn generate_sas() -> Result<TableSasQueryParameters, SasError> {
    let credentials = AccountCredential::new_emulator();

    let builder = TableAccountSasBuilder::new(
        Utc::now() + Duration::hours(1),
        TableAccountSasPermissions::new().add_permission(TableAccountSasPermission::Read),
        TableAccountSasResourceTypes::new().add_resource(TableAccountSasResourceType::Object),
    );

    let optional_options = TableAccountSasOptionalOptions::default()
        .ip(TableSasIpOption::new_single([127, 0, 0, 1]))
        .protocol(TableSasProtocol::Https)
        .start_time(Utc::now() + Duration::minutes(1));

    builder.sign(&credentials, &optional_options)
}
