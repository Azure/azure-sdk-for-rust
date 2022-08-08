use crate::prelude::*;
use azure_core::{headers::*, AppendToUrlQuery, Context, Etag, Method, Response};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use serde::de::DeserializeOwned;
use std::{convert::TryInto, marker::PhantomData};

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
            let mut url = self.entity_client.url().to_owned();

            self.select.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.insert(ACCEPT, "application/json;odata=fullmetadata");

            let mut request =
                self.entity_client
                    .finalize_request(url, Method::Get, headers, None)?;

            let response = self
                .entity_client
                .send(&mut self.context, &mut request)
                .await?;

            GetEntityResponse::try_from(response).await
        })
    }
}

azure_core::future!(GetEntity<T>);

#[cfg(feature = "into_future")]
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
        let body = body.collect().await?;

        let get_entity_response_internal: GetEntityResponseInternal<T> =
            serde_json::from_slice(&body)?;

        Ok(GetEntityResponse {
            common_storage_response_headers: (&headers).try_into()?,
            metadata: get_entity_response_internal.metadata,
            entity: get_entity_response_internal.value,
            etag: etag_from_headers(&headers)?.into(),
        })
    }
}
