use crate::data_lake::responses::*;
use crate::{data_lake::clients::FileSystemClient, Properties};
use azure_core::prelude::*;
use azure_core::{
    headers::add_optional_header, headers::add_optional_header_ref, AppendToUrlQuery,
};
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateFileSystemBuilder<'a> {
    file_system_client: &'a FileSystemClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    properties: Option<&'a Properties<'a, 'a>>,
}

impl<'a> CreateFileSystemBuilder<'a> {
    pub(crate) fn new(file_system_client: &'a FileSystemClient) -> Self {
        Self {
            file_system_client,
            client_request_id: None,
            timeout: None,
            properties: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        properties: &'a Properties<'a, 'a> => Some(properties),
    }

    pub async fn execute(
        &self,
    ) -> Result<CreateFileSystemResponse, Box<dyn std::error::Error + Sync + Send>> {
        // we clone this so we can add custom
        // query parameters
        let mut url = self.file_system_client.url().clone();

        url.query_pairs_mut().append_pair("resource", "filesystem");
        self.timeout.append_to_url_query(&mut url);

        debug!("url = {}", url);

        let request = self.file_system_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.properties, request);
                request
            },
            None,
        )?;

        debug!("request == {:?}", request);

        let response = self
            .file_system_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::CREATED)
            .await?;

        Ok((&response).try_into()?)
    }
}
