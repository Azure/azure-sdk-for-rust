use crate::rest_client::{perform_request, ServiceType};
use azure_sdk_core::errors::AzureError;
use hyper::{self, Method};
use hyper_rustls::HttpsConnector;
use url::Url;

#[derive(Debug, Clone)]
pub struct Client {
    account: String,
    key: String,
    sas_token: Option<Vec<(String, String)>>,
    hc: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
    blob_uri: String,
    table_uri: String,
}

impl Client {
    pub fn new(account: &str, key: &str) -> Result<Client, AzureError> {
        Client::azure(account, key)
    }

    pub fn azure_sas(account: &str, sas_token: &str) -> Result<Client, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());
        let params: Vec<(String, String)> = Url::options()
            // Any base url will do: we just need to parse the SAS token
            // to get its query pairs.
            .base_url(Some(&Url::parse("https://blob.core.windows.net").unwrap()))
            .parse(sas_token)
            .unwrap()
            .query_pairs()
            .map(|p| (String::from(p.0), String::from(p.1)))
            .collect();

        Ok(Client {
            account: account.to_owned(),
            key: String::new(),
            sas_token: Some(params),
            hc: client,
            blob_uri: format!("https://{}.blob.core.windows.net", account),
            table_uri: format!("https://{}.table.core.windows.net", account),
        })
    }

    pub fn azure(account: &str, key: &str) -> Result<Client, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        Ok(Client {
            account: account.to_owned(),
            key: key.to_owned(),
            sas_token: None,
            hc: client,
            blob_uri: format!("https://{}.blob.core.windows.net", account),
            table_uri: format!("https://{}.table.core.windows.net", account),
        })
    }

    pub fn emulator(blob_storage_url: &Url, table_storage_url: &Url) -> Result<Client, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        let blob_uri = format!("{}devstoreaccount1", blob_storage_url.as_str());
        debug!("blob_uri == {}", blob_uri);
        let table_uri = format!("{}devstoreaccount1", table_storage_url.as_str());
        debug!("table_uri == {}", table_uri);

        Ok(Client {
            account: "devstoreaccount1".to_owned(),
            key: "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==".to_owned(),
            sas_token: None,
            hc: client,
            blob_uri,
            table_uri,
        })
    }

    pub fn account(&self) -> &str {
        &self.account
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    #[inline]
    pub fn blob_uri(&self) -> &str {
        &self.blob_uri
    }

    #[inline]
    pub fn table_uri(&self) -> &str {
        &self.table_uri
    }

    fn add_sas_token_to_uri(&self, uri: &str) -> String {
        match &self.sas_token {
            Some(token) => Url::parse_with_params(uri, token).unwrap().to_string(),
            None => String::from(uri),
        }
    }

    pub fn perform_request<F>(
        &self,
        uri: &str,
        method: &Method,
        headers_func: F,
        request_body: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError>
    where
        F: FnOnce(::http::request::Builder) -> ::http::request::Builder,
    {
        let uri = self.add_sas_token_to_uri(uri);

        perform_request(
            &self.hc,
            &uri,
            method,
            &self.key,
            headers_func,
            request_body,
            ServiceType::Blob,
        )
    }

    pub fn perform_table_request<F>(
        &self,
        segment: &str,
        method: &Method,
        headers_func: F,
        request_str: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError>
    where
        F: FnOnce(::http::request::Builder) -> ::http::request::Builder,
    {
        debug!("segment: {}, method: {:?}", segment, method,);

        let uri =
            self.add_sas_token_to_uri((self.get_uri_prefix(ServiceType::Table) + segment).as_str());

        perform_request(
            &self.hc,
            &uri,
            method,
            &self.key,
            headers_func,
            request_str,
            ServiceType::Table,
        )
    }

    /// Uri scheme + authority e.g. http://myaccount.table.core.windows.net/
    pub fn get_uri_prefix(&self, service_type: ServiceType) -> String {
        match service_type {
            ServiceType::Blob => format!("{}/", self.blob_uri()),
            ServiceType::Table => format!("{}/", self.table_uri()),
        }
    }
}
