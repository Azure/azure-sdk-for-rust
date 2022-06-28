use crate::prelude::*;
use azure_core::{
    error::{Error, ErrorKind},
    Context, Method, Response,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct DeleteTableBuilder {
    table_client: TableClient,
    context: Context,
}

impl DeleteTableBuilder {
    pub(crate) fn new(table_client: TableClient) -> Self {
        Self {
            table_client,
            context: Context::new(),
        }
    }

    setters! {
        context: Context => context,
    }

    pub fn into_future(mut self) -> FutureResponse {
        Box::pin(async move {
            let mut url = self.table_client.url().to_owned();
            url.path_segments_mut()
                .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
                .pop()
                .push(&format!("Tables('{}')", self.table_client.table_name()));

            let mut request = self
                .table_client
                .prepare_request(url, Method::DELETE, None)?;
            request.insert_header("Accept", "application/json");

            let response = self
                .table_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }
}

pub type FutureResponse =
    futures::future::BoxFuture<'static, azure_core::Result<DeleteTableResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteTableBuilder {
    type IntoFuture = FutureResponse;
    type Output = <FutureResponse as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
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
