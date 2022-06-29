use crate::prelude::*;
use azure_core::{
    collect_pinned_stream, headers::*, AppendToUrlQuery, Context, Etag, Method, Response,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetEntityBuilder {
    entity_client: EntityClient,
    select: Option<Select>,
    context: Context,
}

impl GetEntityBuilder {
    pub(crate) fn new(entity_client: EntityClient) -> Self {
        Self {
            entity_client,
            select: None,
            context: Context::new(),
        }
    }

    setters! {
        select: Select => Some(select),
        context: Context => context,
    }

    pub fn into_future<E>(mut self) -> FutureResponse<E>
    where
        E: DeserializeOwned,
    {
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

pub type FutureResponse<E> =
    futures::future::BoxFuture<'static, azure_core::Result<GetEntityResponse<E>>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetEntityBuilder {
    type IntoFuture = FutureResponse<E>;
    type Output = <FutureResponse<E> as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct GetEntityResponse<E>
where
    E: DeserializeOwned,
{
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: String,
    pub entity: E,
    pub etag: Etag,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetEntityResponseInternal<E> {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(flatten)]
    pub value: E,
}

impl<E> GetEntityResponse<E>
where
    E: DeserializeOwned,
{
    async fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let get_entity_response_internal: GetEntityResponseInternal<E> =
            serde_json::from_slice(&body)?;

        Ok(GetEntityResponse {
            common_storage_response_headers: (&headers).try_into()?,
            metadata: get_entity_response_internal.metadata,
            entity: get_entity_response_internal.value,
            etag: etag_from_headers(&headers)?.into(),
        })
    }
}
