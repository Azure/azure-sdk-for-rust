use std::io::Read;
use azure::core;
use azure::core::errors::{self, AzureError};
use azure::storage::client::Client;
use hyper::status::StatusCode;
use rustc_serialize::{Decodable, json};

const TABLE_SUFFIX: &'static str = "table.core.windows.net";

pub struct Table;

impl Table {
    pub fn list(client: &Client) -> Result<Vec<String>, core::errors::AzureError> {
        let v1: Vec<TableEntry> = try!(Self::query_range_entry(client, "Tables", None));
        Ok(v1.into_iter().map(|x| x.TableName).collect())
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


    pub fn get(client: &Client,
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
                       path: &str,
                       query: Option<&str>)
                       -> Result<String, core::errors::AzureError> {
        perform_table_request(client,
                              format!("{}?{}",
                                      path,
                                      match query {
                                          Some(clause) => clause,
                                          None => "",
                                      })
                                      .as_str(),
                              core::HTTPMethod::Get,
                              None,
                              StatusCode::Ok)
    }

    pub fn query_range_entry<T: Decodable>(client: &Client,
                                           path: &str,
                                           query: Option<&str>)
                                           -> Result<Vec<T>, core::errors::AzureError> {
        let result = Self::query_range(client, path, query);
        let ec: EntryCollection<T> = json::decode(result?.as_str()).unwrap();
        Ok(ec.value)
    }
}

#[allow(non_snake_case)]
#[derive(RustcDecodable)]
struct TableEntry {
    TableName: String,
}

#[derive(RustcDecodable)]
struct EntryCollection<T> {
    value: Vec<T>,
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