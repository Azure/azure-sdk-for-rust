use crate::account::responses::GetAccountInformationResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use futures::future::done;
use futures::prelude::*;
use hyper::{Method, StatusCode};

#[derive(Debug, Clone)]
pub struct GetAccountInformationBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetAccountInformationBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> GetAccountInformationBuilder<'a> {
        GetAccountInformationBuilder { client }
    }
}

impl<'a> ClientRequired<'a> for GetAccountInformationBuilder<'a> {
    fn client(&self) -> &'a Client {
        self.client
    }
}

// methods callable regardless
impl<'a> GetAccountInformationBuilder<'a> {
    #[inline]
    pub fn finalize(self) -> impl Future<Item = GetAccountInformationResponse, Error = AzureError> {
        let uri = format!("{}/?restype=account&comp=properties", self.client.blob_uri());
        trace!("uri == {:?}", uri);

        let req = self.client().perform_request(&uri, &Method::GET, |ref mut _request| {}, None);

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_headers_and_body(future_response, StatusCode::OK))
            .and_then(move |(headers, _body)| done(GetAccountInformationResponse::from_headers(&headers)))
    }
}
