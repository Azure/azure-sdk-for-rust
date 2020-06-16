use crate::rest_client::{perform_request, ServiceType};
use crate::{Client, ClientEndpoint, HyperClientEndpoint};
use azure_sdk_core::errors::AzureError;
use http::request::Builder;
use hyper::{self, Method};
use hyper_rustls::HttpsConnector;
use url::Url;

#[derive(Debug, Clone)]
pub struct KeyClient {
    account: String,
    key: String,
    sas_token: Option<Vec<(String, String)>>,
    hc: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
    blob_uri: String,
    table_uri: String,
}

pub(crate) fn get_sas_token_parms(sas_token: &str) -> Vec<(String, String)> {
    Url::options()
        // Any base url will do: we just need to parse the SAS token
        // to get its query pairs.
        .base_url(Some(&Url::parse("https://blob.core.windows.net").unwrap()))
        .parse(sas_token)
        .unwrap()
        .query_pairs()
        .map(|p| (String::from(p.0), String::from(p.1)))
        .collect()
}

impl KeyClient {
    #[inline]
    pub(crate) fn new(
        account: String,
        key: String,
        sas_token: Option<Vec<(String, String)>>,
        hc: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
        blob_uri: String,
        table_uri: String,
    ) -> Self {
        Self {
            account,
            key,
            sas_token,
            hc,
            blob_uri,
            table_uri,
        }
    }

    fn add_sas_token_to_uri(&self, uri: &str) -> String {
        match &self.sas_token {
            Some(token) => Url::parse_with_params(uri, token).unwrap().to_string(),
            None => String::from(uri),
        }
    }
}

impl Client for KeyClient {
    #[inline]
    fn blob_uri(&self) -> &str {
        &self.blob_uri
    }

    #[inline]
    fn table_uri(&self) -> &str {
        &self.table_uri
    }

    fn perform_request(
        &self,
        uri: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        let uri = self.add_sas_token_to_uri(uri);

        perform_request(
            self,
            &uri,
            method,
            http_header_adder,
            request_body,
            ServiceType::Blob,
        )
    }

    fn perform_table_request(
        &self,
        segment: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_str: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        debug!("segment: {}, method: {:?}", segment, method,);

        let uri =
            self.add_sas_token_to_uri((self.get_uri_prefix(ServiceType::Table) + segment).as_str());

        perform_request(
            self,
            &uri,
            method,
            http_header_adder,
            request_str,
            ServiceType::Table,
        )
    }
}

impl ClientEndpoint for KeyClient {
    fn account(&self) -> &str {
        &self.account
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl HyperClientEndpoint for KeyClient {
    fn hyper_client(&self) -> &hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
        &self.hc
    }
}
