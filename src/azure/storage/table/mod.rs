extern crate json;

use azure::core;
use azure::core::errors;
use azure::storage::client::Client;
use hyper::status::StatusCode;
use std::io::Read;

pub fn list_tables(client: &Client) -> Result<Vec<String>, core::errors::AzureError> {
    let uri = format!("{}://{}.table.core.windows.net/Tables",
                            client.auth_scheme(),
                            client.account());
    let mut resp = try!(client.perform_table_request(&uri, core::HTTPMethod::Get, None));
    try!(errors::check_status(&mut resp, StatusCode::Ok));
    let mut resp_s = String::new();
    try!(resp.read_to_string(&mut resp_s));
    let parsed = json::parse(&resp_s).unwrap();
    if let json::JsonValue::Array(ref value) = parsed["value"] {
        let mut ret = Vec::new();
        for item in value {
            if let json::JsonValue::Object(ref obj) = *item {
                ret.push(obj.get("TableName").unwrap().as_str().unwrap().to_string());
            }
        }

        Ok(ret)
    } else {
        Err(errors::AzureError::GenericError)
    }
}

pub fn insert_entity(client: &Client, table_name:& str) -> Result<(), core::errors::AzureError> {
    let uri = format!("{}://{}.table.core.windows.net/{}",
                            client.auth_scheme(),
                            client.account(),
                            table_name);
    let body = r#"{"NumberOfOrders":"251",  
"PartitionKey":"myparti1nkey",  
"RowKey":"myrowkey"}"#;
    let mut resp = try!(client.perform_table_request(&uri, core::HTTPMethod::Post, Some(&body)));
    try!(errors::check_status(&mut resp, StatusCode::Created));
    Ok(())
}