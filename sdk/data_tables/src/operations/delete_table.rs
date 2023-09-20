use crate::prelude::*;
use azure_core::{
    error::{Error, ErrorKind},
    headers::*,
    Method, Response,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};

operation! {
    DeleteTable,
    client: TableClient,
}

impl DeleteTableBuilder {
    pub fn into_future(mut self) -> DeleteTable {
        Box::pin(async move {
            let mut url = self.client.url()?;
            url.path_segments_mut()
                .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
                .pop()
                .push(&format!("Tables('{}')", self.client.table_name()));

            let mut headers = Headers::new();
            headers.insert(ACCEPT, "application/json");

            let mut request = TableClient::finalize_request(url, Method::Delete, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteTableResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl TryFrom<Response> for DeleteTableResponse {
    type Error = Error;

    fn try_from(response: Response) -> azure_core::Result<Self> {
        Ok(DeleteTableResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
