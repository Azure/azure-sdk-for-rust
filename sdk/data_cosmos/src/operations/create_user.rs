use crate::{prelude::*, resources::user::UserResponse};
use azure_core::Context;

#[derive(Debug, Clone)]
pub struct CreateUserBuilder {
    client: UserClient,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl CreateUserBuilder {
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

    pub fn into_future(self) -> CreateUser {
        Box::pin(async move {
            let mut request = self.client.cosmos_client().prepare_request_pipeline(
                &format!(
                    "dbs/{}/users",
                    self.client.database_client().database_name()
                ),
                http::Method::POST,
            );

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            let body = CreateUserBody {
                id: self.client.user_name(),
            };
            request.set_body(bytes::Bytes::from(serde_json::to_string(&body)?).into());
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Users),
                    &mut request,
                )
                .await?;

            UserResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type CreateUser = futures::future::BoxFuture<'static, crate::Result<UserResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateUserBuilder {
    type IntoFuture = CreateUser;
    type Output = <CreateUser as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Serialize, Debug)]
struct CreateUserBody<'a> {
    id: &'a str,
}
