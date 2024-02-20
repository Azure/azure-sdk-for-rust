use crate::prelude::*;
use azure_core::{headers::*, AppendToUrlQuery, Context, Etag, Method, Response};
use azure_storage::headers::CommonStorageResponseHeaders;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetEntityBuilder<T> {
    entity_client: EntityClient,
    select: Option<Select>,
    context: Context,
    _entity: PhantomData<T>,
}

impl<T: DeserializeOwned + Send> GetEntityBuilder<T> {
    pub(crate) fn new(entity_client: EntityClient) -> Self {
        Self {
            entity_client,
            select: None,
            context: Context::new(),
            _entity: PhantomData,
        }
    }

    setters! {
        select: Select => Some(select),
        context: Context => context,
    }

    pub fn into_future(mut self) -> GetEntity<T> {
        Box::pin(async move {
            let mut url = self.entity_client.url()?;

            self.select.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.insert(ACCEPT, "application/json;odata=fullmetadata");

            let mut request = EntityClient::finalize_request(url, Method::Get, headers, None)?;

            let response = self
                .entity_client
                .send(&mut self.context, &mut request)
                .await?;

            GetEntityResponse::try_from(response).await
        })
    }
}

azure_core::future!(GetEntity<T>);

impl<T: DeserializeOwned + Send> std::future::IntoFuture for GetEntityBuilder<T> {
    type IntoFuture = GetEntity<T>;
    type Output = <GetEntity<T> as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct GetEntityResponse<T>
where
    T: DeserializeOwned,
{
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: String,
    pub entity: T,
    pub etag: Etag,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetEntityResponseInternal<T> {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(flatten)]
    pub value: T,
}

impl<T> GetEntityResponse<T>
where
    T: DeserializeOwned,
{
    async fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();

        let get_entity_response_internal: GetEntityResponseInternal<T> = body.json().await?;

        Ok(GetEntityResponse {
            common_storage_response_headers: (&headers).try_into()?,
            metadata: get_entity_response_internal.metadata,
            entity: get_entity_response_internal.value,
            etag: etag_from_headers(&headers)?.into(),
        })
    }
}
