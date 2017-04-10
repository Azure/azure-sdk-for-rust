use std::io::Read;
use azure::core;
use azure::core::errors::{self, AzureError};
use azure::storage::client::Client;
use hyper::client::response::Response;
use hyper::status::StatusCode;
use rustc_serialize::{Decodable, Encodable, json};

const TABLE_SUFFIX: &'static str = "table.core.windows.net";
const TABLE_TABLES: &'static str = "TABLES";

pub struct TableClient {
    client: Client,
}

impl TableClient {
    pub fn new(client: Client) -> Self {
        TableClient { client: client }
    }

    pub fn list(&self) -> Result<Vec<String>, core::errors::AzureError> {
        Ok(self.query_range_entity(TABLE_TABLES, None)?
               .into_iter()
               .map(|x: TableEntry| x.TableName)
               .collect())
    }

    pub fn create_if_not_exists<T: Into<String>>(&self,
                                                 table_name: T)
                                                 -> Result<(), core::errors::AzureError> {
        let ref body = json::encode(&TableEntry { TableName: table_name.into() }).unwrap();
        let mut response = try!(self.do_request(TABLE_TABLES, core::HTTPMethod::Post, Some(body)));
        // TODO: Here treats conflict as existed, but could be reserved name, such as 'Tables',
        // should check table existence directly
        if !(StatusCode::Created == response.status || StatusCode::Conflict == response.status) {
            try!(errors::check_status(&mut response, StatusCode::Created));
        }

        Ok(())
    }

    pub fn insert_entity<T: Encodable>(&self,
                                       table_name: &str,
                                       entity: &T)
                                       -> Result<(), AzureError> {
        let ref body = json::encode(entity).unwrap();
        let mut resp = try!(self.do_request(table_name, core::HTTPMethod::Post, Some(body)));
        try!(errors::check_status(&mut resp, StatusCode::Created));
        Ok(())
    }

    pub fn update_entity<T: Encodable>(&self,
                                       table_name: &str,
                                       partition_key: &str,
                                       row_key: &str,
                                       entity: &T)
                                       -> Result<(), core::errors::AzureError> {
        let ref body = json::encode(entity).unwrap();
        let ref path = format!("{}(PartitionKey='{}',RowKey='{}')",
                               table_name,
                               partition_key,
                               row_key);
        let mut resp = try!(self.do_request(path, core::HTTPMethod::Put, Some(body)));
        try!(errors::check_status(&mut resp, StatusCode::NoContent));
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
        let mut response = try!(self.do_request(path, core::HTTPMethod::Get, None));
        if StatusCode::NotFound == response.status {
            return Ok(None);
        }
        try!(errors::check_status(&mut response, StatusCode::Ok));
        let ref body = try!(get_response_body(&mut response));
        Ok(json::decode(body).unwrap())
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
        let mut response = try!(self.do_request(path, core::HTTPMethod::Get, None));
        try!(errors::check_status(&mut response, StatusCode::Ok));
        let ref body = try!(get_response_body(&mut response));
        let ec: EntryCollection<T> = json::decode(body).unwrap();
        Ok(ec.value)
    }

    pub fn do_request(&self,
                      segment: &str,
                      method: core::HTTPMethod,
                      request_str: Option<&str>)
                      -> Result<Response, core::errors::AzureError> {
        let client = &self.client;
        let uri = format!("{}://{}.{}/{}",
                          client.auth_scheme(),
                          client.account(),
                          TABLE_SUFFIX,
                          segment);
        trace!("{:?} {}", method, uri);
        if let Some(ref body) = request_str {
            trace!("Request: {}", body);
        }

        let resp = try!(client.perform_table_request(&uri, method, request_str));
        trace!("Response status: {:?}", resp.status);
        Ok(resp)
    }
}

fn get_response_body(resp: &mut Response) -> Result<String, core::errors::AzureError> {
    let mut body = String::new();
    try!(resp.read_to_string(&mut body));
    trace!("Response Body:{}", body);
    Ok(body)
}

#[allow(non_snake_case)]
#[derive(RustcEncodable, RustcDecodable)]
struct TableEntry {
    TableName: String,
}

#[derive(RustcDecodable)]
struct EntryCollection<T> {
    value: Vec<T>,
}
