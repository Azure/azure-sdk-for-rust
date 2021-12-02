pub mod options;
pub mod table_account_sas_builder;
pub mod table_sas_query_parameters;

#[cfg(test)]
mod test {
    // use std::{
    //     net::{IpAddr, Ipv4Addr},
    //     str::FromStr,
    // };

    // use crate::authorization::AccountCredential;

    // use super::{
    //     options::{
    //         table_account_sas_options::TableAccountSasOptions,
    //         table_sas_options::TableSasQueryOptions, TableAccountSasPermission,
    //         TableAccountSasPermissions, TableAccountSasResourceType, TableAccountSasResourceTypes,
    //         TableSasIpRange, TableSasProtocol,
    //     },
    //     TableAccountSasBuilder,
    // };
    // use chrono::{Duration, Utc};

    // #[test]
    // fn try_create_table_account_sas() {
    //     let account_credential = AccountCredential::new("", "");

    //     let expiry_time = Utc::now() + Duration::days(1);
    //     let permissions_builder = TableAccountSasPermissions::default()
    //         .add_permission(TableAccountSasPermission::Read)
    //         .add_permission(TableAccountSasPermission::List);
    //     let resource_types_builder = TableAccountSasResourceTypes::default()
    //         .add_resource(TableAccountSasResourceType::Container);

    //     let sas_options =
    //         TableAccountSasOptions::new(expiry_time, permissions_builder, resource_types_builder)
    //             .protocol(TableSasProtocol::Https)
    //             .identifier("identifier")
    //             .start_time(Utc::now() + Duration::hours(1))
    //             .ip_range(TableSasIpRange::new(
    //                 IpAddr::V4(Ipv4Addr::from_str("127.0.0.1").unwrap()),
    //                 IpAddr::V4(Ipv4Addr::from_str("127.0.0.2").unwrap()),
    //             ));

    //     match TableAccountSasBuilder::new(sas_options).sign(&account_credential) {
    //         Ok(sas) => println!("sas: {:#?}", sas),
    //         Err(err) => eprintln!("error: {:#?}", err),
    //     };
    // }

    // #[test]
    // fn try_create_table_account_sas_all_all() {
    //     // let credential = AccountCredential::new("", "");
    //     // let sas_options = TableAccountSasOptions::new(
    //     //     Utc::now() + Duration::days(1),
    //     //     TableAccountSasPermissions::all(),
    //     //     TableAccountSasResourceTypes::all(),
    //     // );
    //     // let sas = TableAccountSasBuilder::new(sas_options)
    //     //     .sign(&credential)
    //     //     .unwrap();
    // }

    // #[test]
    // fn try_create_table_sas() {
    //     let sas_options = TableSasQueryOptions::new("some_table_name")
    //         .start_partition_key("partition_key_from")
    //         .end_partition_key("partition_key_to")
    //         .start_row_key("row_key_from")
    //         .end_row_key("row_key_from");
    // }
}
