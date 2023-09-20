use crate::{prelude::*, IfMatchCondition};
use azure_core::{error::Error, headers::Headers, Method, Response};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};

operation! {
    DeleteEntity,
    client: EntityClient,
    ?if_match: IfMatchCondition
}

impl DeleteEntityBuilder {
    pub fn into_future(mut self) -> DeleteEntity {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            headers.add(self.if_match.unwrap_or(IfMatchCondition::Any));

            let mut request = EntityClient::finalize_request(url, Method::Delete, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteEntityResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl TryFrom<Response> for DeleteEntityResponse {
    type Error = Error;

    fn try_from(response: Response) -> azure_core::Result<Self> {
        Ok(DeleteEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
