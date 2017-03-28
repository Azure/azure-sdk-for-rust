extern crate json;

use azure::core;
use azure::core::errors::{self, AzureError};
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
            Err(AzureError::GenericError)
        }
    }

    pub fn insert(client: &Client,
                  table_name: &str,
                  partition_key: &str,
                  row_key: &str,
                  body: &str)
                  -> Result<(), core::errors::AzureError> {
        // TODO: more elegant ways for insert keys.
        if !body.starts_with("{") {
            return Err(AzureError::InputParametersError("body not valid.".to_owned()));
        };

        let body = format!(r#"{{"PartitionKey":"{}","RowKey":"{}",{}"#,
                           partition_key,
                           row_key,
                           &body[1..]);

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
                 row_key: &str)
                 -> Result<String, core::errors::AzureError> {
        let segment = format!("{}(PartitionKey='{}',RowKey='{}')",
                              table_name,
                              partition_key,
                              row_key);

        perform_table_request(client,
                              segment.as_str(),
                              core::HTTPMethod::Get,
                              None,
                              StatusCode::Ok)
    }

    pub fn query_range(client: &Client,
                       table_name: &str,
                       partition_key: &str,
                       ge: bool,
                       limit: u16)
                       -> Result<String, core::errors::AzureError> {
        let op = if ge { "ge" } else { "le" };
        let segment = format!("{}?$filter=PartitionKey {} {}&$top={}",
                              table_name,
                              op,
                              partition_key,
                              limit);
        perform_table_request(client,
                              segment.as_str(),
                              core::HTTPMethod::Get,
                              None,
                              StatusCode::Ok)
    }
}


pub fn perform_table_request(client: &Client,
                             segment: &str,
                             method: core::HTTPMethod,
                             request_str: Option<&str>,
                             expected_status_code: StatusCode)
                             -> Result<String, core::errors::AzureError> {

    if let Some(ref body) = request_str {
        trace!("Request: {}", body);
    }

    let uri = format!("{}://{}.{}/{}",
                      client.auth_scheme(),
                      client.account(),
                      TABLE_SUFFIX,
                      segment);
    trace!("uri:{}", uri);
    let mut resp = try!(client.perform_table_request(&uri, method, request_str));
    try!(errors::check_status(&mut resp, expected_status_code));
    let mut resp_s = String::new();
    try!(resp.read_to_string(&mut resp_s));

    trace!("Response: {}", resp_s);
    Ok(resp_s)
}