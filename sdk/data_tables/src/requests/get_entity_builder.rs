use crate::{prelude::*, responses::*};
use azure_core::{error::Result, headers::*, prelude::*, AppendToUrlQuery, Method};
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

    pub async fn execute<E>(&self) -> Result<GetEntityResponse<E>>
    where
        E: DeserializeOwned,
    {
        let mut url = self.entity_client.url().to_owned();

        self.select.append_to_url_query(&mut url);

        let mut headers = Headers::new();
        headers.add(self.client_request_id.clone());
        headers.insert(ACCEPT, "application/json;odata=fullmetadata");

        let request = self
            .entity_client
            .finalize_request(url, Method::Get, headers, None)?;

        let response = self
            .entity_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}
