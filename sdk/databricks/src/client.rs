use http::{HeaderMap, Method, Uri, self};
use std::sync::Arc;
use azure_core::{Body, prelude::*};
use crate::core::*;
use bytes::Bytes;

#[derive(Debug)]
pub enum DatabricksCredentials{
    BearerToken(String),
    Username(String)
}

#[derive(Debug)]
pub struct DatabricksClient{
    credentials: DatabricksCredentials,
    host: Uri,
    http_client: Arc<dyn HttpClient>,
    port: u16
}

impl DatabricksClient{
    pub fn new<'a>(credentials: DatabricksCredentials, host: &'a str, port: Option<u16>) -> Arc<Self>{
        let http_client = new_http_client();

        let port = port.unwrap_or_else(|| {DATABRICKS_DEFAULT_PORT});


        let url_parsed = host.parse::<Uri>().expect("Unable to parse url");
        Arc::new(Self{
            credentials: credentials,
            host: url_parsed,
            port: port,
            http_client: http_client
        })
    }

    pub fn http_client(&self) -> &dyn HttpClient{
        self.http_client.as_ref()
    }

    pub fn credentials(&self) -> &DatabricksCredentials{
        &self.credentials
    }


    // so far prepare only supports personal access token
    pub fn prepare_request(&self, method: &Method, body: Body) -> http::Result<http::Request<Bytes>>{
        let mut auth_token: String = String::from("Bearer: ");

        let token = match self.credentials(){
            DatabricksCredentials::BearerToken(token) => {
                token
            }
            _ => {
                panic!("nope!");
            }
        };

        auth_token.push_str(token);
        let request = http::Request::builder().uri(&self.host).method(method).header("Authorization", auth_token);

        if let Body::Bytes(body) = body {
            request.body(body)
        }
        else{
            request.body(Bytes::new())
        }
    }
}

