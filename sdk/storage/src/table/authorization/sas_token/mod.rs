pub mod options;
pub mod table_account_sas_builder;
pub mod table_sas_query_parameters;

#[cfg(test)]
mod test {

    use chrono::{DateTime, Utc};

    use crate::authorization::{
        sas_token::options::{
            table_account_sas_permission::TableAccountSasPermission,
            table_account_sas_resource_type::TableAccountSasResourceType,
        },
        AccountCredential,
    };

    use super::{
        options::{
            table_account_sas_optional_options::TableAccountSasOptionalOptions,
            table_account_sas_permission::TableAccountSasPermissions,
            table_account_sas_resource_type::TableAccountSasResourceTypes,
        },
        table_account_sas_builder::TableAccountSasBuilder,
    };

    #[test]
    fn try_create_table_account_sas() {
        let credentials = AccountCredential::new_emulator();

        let expires_on = DateTime::<Utc>::from_utc(
            DateTime::parse_from_rfc3339("2021-12-05T17:44:05Z")
                .unwrap()
                .naive_utc(),
            Utc,
        );

        let builder = TableAccountSasBuilder::new(
            expires_on,
            TableAccountSasPermissions::new().add_permission(TableAccountSasPermission::Write),
            TableAccountSasResourceTypes::new()
                .add_resource(TableAccountSasResourceType::Container),
        );

        let optional_options = TableAccountSasOptionalOptions::default();

        let sas = builder
            .sign(&credentials, &optional_options)
            .unwrap()
            .token()
            .unwrap();
        println!("{}", sas);
    }
}
