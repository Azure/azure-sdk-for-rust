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

pub fn insert_entity(client: &Client, table_name:& str, partition_key: &str, row_key: &str, key: &str, val: &str) -> Result<(), core::errors::AzureError> {
    let uri = format!("{}://{}.table.core.windows.net/{}",
                            client.auth_scheme(),
                            client.account(),
                            table_name);
    let body = format!( r#"{{"PartitionKey":"{}","RowKey":"{}","{}":"{}"}}"#, partition_key,row_key,key,val);
    let mut resp = try!(client.perform_table_request(&uri, core::HTTPMethod::Post, Some(&body)));
    try!(errors::check_status(&mut resp, StatusCode::Created));
    Ok(())
}

pub fn query_entity(client: &Client, table_name: &str, partition_key: &str, row_key: &str, key: &str) -> Result<String, core::errors::AzureError> {
    let uri = format!("{}://{}.table.core.windows.net/{}(PartitionKey='{}',RowKey='{}')",
                            client.auth_scheme(),
                            client.account(),
                            table_name,
                            partition_key,
                            row_key);
    let mut resp = try!(client.perform_table_request(&uri, core::HTTPMethod::Get, None));
    let mut resp_s = String::new();
    try!(resp.read_to_string(&mut resp_s));
    let parsed = json::parse(&resp_s).unwrap();
    // println!("{:?}", parsed);
    if let json::JsonValue::Object(ref obj) = parsed {
        let kn = obj.get(key).unwrap().as_str().unwrap().to_string();
        // println!("{:?}", kn);
        Ok(kn.to_string())
    }else {
        Err(errors::AzureError::GenericError)
    }
}