use crate::{operations::*, prelude::*};
use azure_core::{
    error::{Error, ErrorKind},
    headers::*,
    prelude::*,
    CollectedResponse, Context, Method,
};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use std::{convert::TryInto, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct InsertEntityBuilder<T> {
    table_client: TableClient,
    body: Bytes,
    return_entity: ReturnEntity,
    timeout: Option<Timeout>,
    context: Context,
    _entity: PhantomData<T>,
}

impl<T> InsertEntityBuilder<T>
where
    T: DeserializeOwned + Send,
{
    pub(crate) fn new(table_client: TableClient, body: Bytes) -> Self {
        Self {
            table_client,
            body,
            return_entity: false.into(),
            timeout: None,
            context: Context::new(),
            _entity: PhantomData,
        }
    }

    setters! {
        return_entity: ReturnEntity => return_entity,
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(mut self) -> FutureResponse<T> {
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
}

pub type FutureResponse<T> =
    futures::future::BoxFuture<'static, azure_core::Result<InsertEntityResponse<T>>>;

#[cfg(feature = "into_future")]
impl<T: DeserializeOwned + Send> std::future::IntoFuture for InsertEntityBuilder<T> {
    type IntoFuture = FutureResponse<T>;
    type Output = <FutureResponse<T> as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
