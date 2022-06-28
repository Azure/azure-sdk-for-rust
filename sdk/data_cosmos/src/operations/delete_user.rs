use crate::headers::from_headers::*;
use crate::prelude::*;
use azure_core::{headers::session_token_from_headers, Context, Response as HttpResponse};

#[derive(Debug, Clone)]
pub struct DeleteUserBuilder {
    client: UserClient,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl DeleteUserBuilder {
    pub(crate) fn new(client: UserClient) -> Self {
        Self {
            client,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> DeleteUser {
        Box::pin(async move {
            let mut request = self.client.user_request(azure_core::Method::Delete);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            request.set_body(bytes::Bytes::new());
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Users),
                    &mut request,
                )
                .await?;

            DeleteUserResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type DeleteUser = futures::future::BoxFuture<'static, azure_core::Result<DeleteUserResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteUserBuilder {
    type IntoFuture = DeleteUser;
    type Output = <DeleteUser as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct DeleteUserResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl DeleteUserResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
        })
    }
}
