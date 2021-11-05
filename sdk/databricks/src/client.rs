use crate::prelude::*;
use reqwest::{self, header, Client, Url};
use std::sync::Arc;

#[derive(Debug)]
pub struct DatabricksClient {
    token: String,
    pub http_client: Client,
    pub host: Url,
}

impl DatabricksClient {
    pub fn new<'a>(token: &'a str, host: &'a str) -> DatabricksResult<Arc<Self>> {
        let mut headers = header::HeaderMap::new();

        let bearer = "Bearer ".to_owned() + &token.to_string();

        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&bearer).unwrap(),
        );

        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str("application/json").unwrap(),
        );

        let http_client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Unable to build http_client");

        let url_parsed: Url = Url::parse(host).expect("host not in correct format");

        Ok(Arc::new(Self {
            token: String::from(token),
            host: url_parsed,
            http_client: http_client,
        }))
    }
}
