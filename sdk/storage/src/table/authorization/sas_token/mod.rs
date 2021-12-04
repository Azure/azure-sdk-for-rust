pub mod options;
pub mod table_account_sas_builder;
pub mod table_sas_query_parameters;

#[cfg(test)]
mod test {
    use std::ops::Add;

    use chrono::{DateTime, Duration, Utc};

    use crate::authorization::{
        sas_token::options::{
            table_account_sas_permission::TableAccountSasPermission,
            table_account_sas_resource_type::TableAccountSasResourceType,
            table_sas_ip_option::TableSasIpOption, table_sas_protocol::TableSasProtocol,
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

        let builder = TableAccountSasBuilder::new(
            Utc::now() + Duration::hours(1),
            TableAccountSasPermissions::new().add_permission(TableAccountSasPermission::Read),
            TableAccountSasResourceTypes::new().add_resource(TableAccountSasResourceType::Object),
        );

        let optional_options = TableAccountSasOptionalOptions::default();
        // .protocol(TableSasProtocol::Https)
        // .start_time(Utc::now() + Duration::minutes(1))
        // .ip(TableSasIpOption::new_single([127, 0, 0, 1]));

        let sas = builder.sign(&credentials, &optional_options).unwrap();
        println!("{:#?}", sas);

        let sas_string: String = sas.into();
        println!("{}", sas_string);
    }
}
