use std::io::Read;
use azure::core;
use azure::core::errors::{self, AzureError};
use azure::storage::client::Client;
use hyper::status::StatusCode;
use rustc_serialize::{Decodable, Encodable, json};

const TABLE_SUFFIX: &'static str = "table.core.windows.net";

pub struct TableClient {
    client: Client,
}

impl TableClient {
    pub fn new(client: Client) -> Self {
        TableClient { client: client }
    }

    pub fn list(&self) -> Result<Vec<String>, core::errors::AzureError> {
        Ok(self.query_range_entity("Tables", None)?
               .into_iter()
               .map(|x: TableEntry| x.TableName)
               .collect())
    }

    pub fn insert_entity<T: Encodable>(&self,
                                       table_name: &str,
                                       partition_key: &str,
                                       row_key: &str,
                                       entity: &T)
                                       -> Result<(), core::errors::AzureError> {
        let ref body = json::encode(entity).unwrap();
        if !body.starts_with("{") {
            return Err(AzureError::InputParametersError("body not valid.".to_owned()));
        };

        let ref post_body = format!(r#"{{"PartitionKey":"{}","RowKey":"{}",{}"#,
                                    partition_key,
                                    row_key,
                                    &body[1..]);

        try!(self.do_request(table_name,
                             core::HTTPMethod::Post,
                             Some(post_body),
                             StatusCode::Created));
        Ok(())
    }

    pub fn get_entity<T: Decodable>(&self,
                                    table_name: &str,
                                    partition_key: &str,
                                    row_key: &str)
                                    -> Result<Option<T>, core::errors::AzureError> {
        let ref path = format!("{}(PartitionKey='{}',RowKey='{}')",
                               table_name,
                               partition_key,
                               row_key);
        Ok(self.do_request(path,
                           core::HTTPMethod::Get,
                           None,
                           StatusCode::Ok)?
               .map(|x| json::decode(x.as_str()).unwrap()))
    }

    pub fn query_range_entity<T: Decodable>(&self,
                                           path: &str,
                                           query: Option<&str>)
                                           -> Result<Vec<T>, core::errors::AzureError> {
        let ref path = format!("{}?{}",
                               path,
                               match query {
                                   Some(clause) => clause,
                                   None => "",
                               });
        let result =
            self.do_request(path, core::HTTPMethod::Get, None, StatusCode::Ok).map(|x| x.unwrap());
        let ec: EntryCollection<T> = json::decode(result?.as_str()).unwrap();
        Ok(ec.value)
    }

    pub fn do_request(&self,
                      segment: &str,
                      method: core::HTTPMethod,
                      request_str: Option<&str>,
                      expected_status_code: StatusCode)
                      -> Result<Option<String>, core::errors::AzureError> {
        let client = &self.client;
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
        if StatusCode::NotFound == resp.status {
            return Ok(None);
        }

        try!(errors::check_status(&mut resp, expected_status_code));
        let mut resp_s = String::new();
        try!(resp.read_to_string(&mut resp_s));

        trace!("Response: {}", resp_s);
        Ok(Some(resp_s))
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
