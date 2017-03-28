extern crate json;

use azure::core;
use azure::core::errors;
use azure::storage::client::Client;
use hyper::status::StatusCode;
use std::io::Read;

const TABLE_SUFFIX: &'static str = "table.core.windows.net";

pub struct Table;
impl Table {
    pub fn list(client: &Client) -> Result<Vec<String>, core::errors::AzureError> {
        let resp_s = try!(perform_table_request(client,
                                                "Tables",
                                                core::HTTPMethod::Get,
                                                None,
                                                StatusCode::Ok));
        let parsed = json::parse(&resp_s).unwrap();
        if let json::JsonValue::Array(ref value) = parsed["value"] {
            let mut ret = Vec::new();
            for item in value {
                if let json::JsonValue::Object(ref obj) = *item {
                    ret.push(obj.get("TableName")
                                 .unwrap()
                                 .as_str()
                                 .unwrap()
                                 .to_string());
                }
            }

            Ok(ret)
        } else {
            Err(errors::AzureError::GenericError)
        }
    }

    pub fn insert(client: &Client,
                  table_name: &str,
                  partition_key: &str,
                  row_key: &str,
                  key: &str,
                  val: &str)
                  -> Result<(), core::errors::AzureError> {
        let body = format!(r#"{{"PartitionKey":"{}","RowKey":"{}","{}":"{}"}}"#,
                           partition_key,
                           row_key,
                           key,
                           val);
        try!(perform_table_request(client,
                                   table_name,
                                   core::HTTPMethod::Post,
                                   Some(&body),
                                   StatusCode::Created));
        Ok(())
    }


    pub fn query(client: &Client,
                 table_name: &str,
                 partition_key: &str,
                 row_key: &str,
                 key: &str)
                 -> Result<String, core::errors::AzureError> {
        let segment = format!("{}(PartitionKey='{}',RowKey='{}')",
                              table_name,
                              partition_key,
                              row_key);

        let resp_s = try!(perform_table_request(client,
                                                segment.as_str(),
                                                core::HTTPMethod::Get,
                                                None,
                                                StatusCode::Ok));

        let parsed = json::parse(&resp_s).unwrap();
        // println!("{:?}", parsed);
        if let json::JsonValue::Object(ref obj) = parsed {
            let kn = obj.get(key)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            // println!("{:?}", kn);
            Ok(kn.to_string())
        } else {
            Err(errors::AzureError::GenericError)
        }
    }
}


pub fn perform_table_request(client: &Client,
                             segment: &str,
                             method: core::HTTPMethod,
                             request_str: Option<&str>,
                             expected_status_code: StatusCode)
                             -> Result<String, core::errors::AzureError> {
    let uri = format!("{}://{}.{}/{}",
                      client.auth_scheme(),
                      client.account(),
                      TABLE_SUFFIX,
                      segment);
    let mut resp = try!(client.perform_table_request(&uri, method, request_str));
    try!(errors::check_status(&mut resp, expected_status_code));
    let mut resp_s = String::new();
    try!(resp.read_to_string(&mut resp_s));

    Ok(resp_s)
}