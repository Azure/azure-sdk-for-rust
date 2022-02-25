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
            let mut request = self
                .client
                .prepare_request_with_user_name(http::Method::DELETE);
            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;
            request.set_body(bytes::Bytes::from_static(&[]).into());
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

type DeleteUser = futures::future::BoxFuture<'static, crate::Result<DeleteUserResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteUserBuilder {
    type Future = DeleteUser;
    type Output = <DeleteUser as std::future::Future>::Output;
    fn into_future(self) -> Self::Future {
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
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
        })
    }
}
