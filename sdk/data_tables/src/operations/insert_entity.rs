use crate::{operations::*, prelude::*, TransactionOperation};
use azure_core::{
    error::{Error, ErrorKind},
    headers::*,
    prelude::*,
    CollectedResponse, Context, Method, Request,
};
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct InsertEntityBuilder {
    table_client: TableClient,
    body: Bytes,
    return_entity: ReturnEntity,
    timeout: Option<Timeout>,
    context: Context,
}

impl InsertEntityBuilder {
    pub(crate) fn new(table_client: TableClient, body: Bytes) -> Self {
        Self {
            table_client,
            body,
            return_entity: false.into(),
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        return_entity: ReturnEntity => return_entity,
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future<E>(mut self) -> FutureResponse<E>
    where
        E: Serialize + DeserializeOwned,
    {
        Box::pin(async move {
            let mut url = self.table_client.url().to_owned();
            url.path_segments_mut()
                .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
                .pop()
                .push(self.table_client.table_name());

            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.return_entity);
            headers.insert(ACCEPT, "application/json;odata=fullmetadata");
            headers.insert(CONTENT_TYPE, "application/json");

            let mut request =
                self.table_client
                    .finalize_request(url, Method::Post, headers, Some(self.body))?;

            let response = self
                .table_client
                .send(&mut self.context, &mut request)
                .await?;

            let collected_response = CollectedResponse::from_response(response).await?;

            collected_response.try_into()
        })
    }

    pub fn to_transaction_operation(self) -> azure_core::Result<TransactionOperation> {
        let mut url = self.table_client.url().to_owned();
        url.path_segments_mut()
            .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
            .pop()
            .push(self.table_client.table_name());

        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json;odata=fullmetadata");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.set_body(self.body);

        Ok(TransactionOperation::new(request))
    }
}

pub type FutureResponse<E> =
    futures::future::BoxFuture<'static, azure_core::Result<InsertEntityResponse<E>>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for InsertEntityBuilder {
    type IntoFuture = FutureResponse;
    type Output = <FutureResponse as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
