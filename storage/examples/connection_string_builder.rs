use azure_sdk_storage::core::{ConnectionStringBuilder, EndpointProtocol};

pub fn main() {
    let account_name =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let account_key = std::env::var("ACCOUNT_KEY").expect("Set env variable ACCOUNT_KEY first!");
    let default_endpoints_protocol = std::env::var("DEFAULT_ENDPOINTS_PROTOCOL")
        .expect("Set env variable DEFAULT_ENDPOINTS_PROTOCOL first!");
    let default_endpoints_protocol = match &default_endpoints_protocol[..] {
        "https" => EndpointProtocol::Https,
        "http" => EndpointProtocol::Http,
        _ => panic!("Invalid default endpoints protocol"),
    };

    let connection_string = ConnectionStringBuilder::new()
        .with_account_name(&account_name)
        .with_account_key(&account_key)
        .with_default_endpoints_protocol(default_endpoints_protocol)
        .build();

    println!("The connection string is: '{}'", connection_string);
}
