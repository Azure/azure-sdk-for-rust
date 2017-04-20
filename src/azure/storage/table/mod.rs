mod batch;

pub use self::batch::BatchItem;

use self::batch::generate_batch_payload;
use std::io::Read;
use azure::core;
use azure::core::errors::{self, AzureError};
use azure::storage::client::Client;
use azure::storage::rest_client::ServiceType;
use hyper::client::response::Response;
use hyper::header::{Accept, ContentType, Headers, IfMatch, qitem};
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use hyper::status::StatusCode;
use rustc_serialize::{Decodable, Encodable, json};

const TABLE_TABLES: &'static str = "TABLES";

pub struct TableService {
    client: Client,
}

impl TableService {
    pub fn new(client: Client) -> Self {
        TableService { client: client }
    }

    pub fn list_tables(&self) -> Result<Vec<String>, AzureError> {
        Ok(self.query_entities(TABLE_TABLES, None)?
               .into_iter()
               .map(|x: TableEntity| x.TableName)
               .collect())
    }

    // Create table if not exists.
    pub fn create_table<T: Into<String>>(&self, table_name: T) -> Result<(), AzureError> {
        let ref body = json::encode(&TableEntity { TableName: table_name.into() }).unwrap();
        let mut response = try!(self.request_with_default_header(TABLE_TABLES,
                                                                 core::HTTPMethod::Post,
                                                                 Some(body)));
        // TODO: Here treats conflict as existed, but could be reserved name, such as 'Tables',
        // should check table existence directly
        if !(StatusCode::Created == response.status || StatusCode::Conflict == response.status) {
            try!(errors::check_status(&mut response, StatusCode::Created));
        }

        Ok(())
    }

    pub fn get_entity<T: Decodable>(&self,
                                    table_name: &str,
                                    partition_key: &str,
                                    row_key: &str)
                                    -> Result<Option<T>, AzureError> {
        let ref path = entity_path(table_name, partition_key, row_key);
        let mut response =
            try!(self.request_with_default_header(path, core::HTTPMethod::Get, None));
        if StatusCode::NotFound == response.status {
            return Ok(None);
        }
        try!(errors::check_status(&mut response, StatusCode::Ok));
        let ref body = try!(get_response_body(&mut response));
        Ok(json::decode(body).unwrap())
    }

    pub fn query_entities<T: Decodable>(&self,
                                        table_name: &str,
                                        query: Option<&str>)
                                        -> Result<Vec<T>, AzureError> {
        let mut path = table_name.to_owned();
        if let Some(clause) = query {
            path.push_str("?");
            path.push_str(clause);
        }

        let mut response =
            try!(self.request_with_default_header(path.as_str(), core::HTTPMethod::Get, None));
        try!(errors::check_status(&mut response, StatusCode::Ok));
        let ref body = try!(get_response_body(&mut response));
        let ec: EntityCollection<T> = json::decode(body).unwrap();
        Ok(ec.value)
    }

    pub fn insert_entity<T: Encodable>(&self,
                                       table_name: &str,
                                       entity: &T)
                                       -> Result<(), AzureError> {
        let ref body = json::encode(entity).unwrap();
        let mut resp =
            try!(self.request_with_default_header(table_name, core::HTTPMethod::Post, Some(body)));
        try!(errors::check_status(&mut resp, StatusCode::Created));
        Ok(())
    }

    pub fn update_entity<T: Encodable>(&self,
                                       table_name: &str,
                                       partition_key: &str,
                                       row_key: &str,
                                       entity: &T)
                                       -> Result<(), AzureError> {
        let ref body = json::encode(entity).unwrap();
        let ref path = entity_path(table_name, partition_key, row_key);
        let mut resp =
            try!(self.request_with_default_header(path, core::HTTPMethod::Put, Some(body)));
        try!(errors::check_status(&mut resp, StatusCode::NoContent));
        Ok(())
    }

    pub fn delete_entity(&self,
                         table_name: &str,
                         partition_key: &str,
                         row_key: &str)
                         -> Result<(), AzureError> {
        let ref path = entity_path(table_name, partition_key, row_key);
        let mut headers = Headers::new();
        headers.set(Accept(vec![qitem(get_json_mime_nometadata())]));
        headers.set(IfMatch::Any);

        let mut resp = try!(self.request(path, core::HTTPMethod::Delete, None, headers));
        try!(errors::check_status(&mut resp, StatusCode::NoContent));
        Ok(())
    }

    pub fn batch<T: Encodable>(&self,
                               table_name: &str,
                               partition_key: &str,
                               batch_items: &[BatchItem<T>])
                               -> Result<(), AzureError> {
        let ref payload =
            generate_batch_payload(self.client.get_uri_prefix(ServiceType::Table).as_str(),
                                   table_name,
                                   partition_key,
                                   batch_items);
        let mut headers = Headers::new();
        headers.set(ContentType(get_batch_mime()));
        let mut response =
            try!(self.request("$batch", core::HTTPMethod::Post, Some(payload), headers));
        try!(errors::check_status(&mut response, StatusCode::Accepted));
        // TODO deal with body response, handle batch failure.
        // let ref body = try!(get_response_body(&mut response));
        // info!("{}", body);
        Ok(())
    }

    fn request_with_default_header(&self,
                                   segment: &str,
                                   method: core::HTTPMethod,
                                   request_str: Option<&str>)
                                   -> Result<Response, AzureError> {
        let mut headers = Headers::new();
        headers.set(Accept(vec![qitem(get_json_mime_nometadata())]));
        if request_str.is_some() {
            headers.set(ContentType(get_default_json_mime()));
        }
        self.request(segment, method, request_str, headers)
    }

    fn request(&self,
               segment: &str,
               method: core::HTTPMethod,
               request_str: Option<&str>,
               headers: Headers)
               -> Result<Response, AzureError> {
        trace!("{:?} {}", method, segment);
        if let Some(ref body) = request_str {
            trace!("Request: {}", body);
        }

        let resp = try!(self.client.perform_table_request(segment, method, headers, request_str));
        trace!("Response status: {:?}", resp.status);
        Ok(resp)
    }
}


#[allow(non_snake_case)]
#[derive(RustcEncodable, RustcDecodable)]
struct TableEntity {
    TableName: String,
}

#[derive(RustcDecodable)]
struct EntityCollection<T> {
    value: Vec<T>,
}

fn get_response_body(resp: &mut Response) -> Result<String, AzureError> {
    let mut body = String::new();
    try!(resp.read_to_string(&mut body));
    trace!("Response Body:{}", body);
    Ok(body)
}

#[inline]
fn entity_path(table_name: &str, partition_key: &str, row_key: &str) -> String {
    table_name.to_owned() + "(PartitionKey='" + partition_key + "',RowKey='" + row_key + "')"
}

#[inline]
fn get_default_json_mime() -> Mime {
    return Mime(TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, Value::Utf8)]);
}

#[inline]
fn get_json_mime_nometadata() -> Mime {
    return Mime(TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Ext("odata".to_owned()), Value::Ext("nometadata".to_owned()))]);
}

#[inline]
fn get_batch_mime() -> Mime {
    return Mime(TopLevel::Multipart,
                SubLevel::Ext("Mixed".to_owned()),
                vec![(Attr::Ext("boundary".to_owned()),
                      Value::Ext("batch_a1e9d677-b28b-435e-a89e-87e6a768a431".to_owned()))]);
}