use crate::prelude::*;
use crate::responses::*;
use azure_core::prelude::*;
use azure_core::{headers::add_optional_header, AppendToUrlQuery};
use http::method::Method;
use http::status::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetEntityBuilder<'a> {
    entity_client: &'a EntityClient,
    select: Option<Select<'a>>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> GetEntityBuilder<'a> {
    pub(crate) fn new(entity_client: &'a EntityClient) -> Self {
        Self {
            entity_client,
            select: None,
            client_request_id: None,
        }
    }

    setters! {
        select: Select<'a> => Some(select),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute<E>(
        &self,
    ) -> Result<GetEntityResponse<E>, Box<dyn std::error::Error + Sync + Send>>
    where
        E: DeserializeOwned,
    {
        let mut url = self.entity_client.url().to_owned();

        self.select.append_to_url_query(&mut url);

        debug!("list tables url = {}", url);

        let request = self.entity_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = request.header("Accept", "application/json;odata=fullmetadata");
                request
            },
            None,
        )?;

        debug!("request == {:#?}\n", request);

        let response = self
            .entity_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        Ok((&response).try_into()?)
    }
}
