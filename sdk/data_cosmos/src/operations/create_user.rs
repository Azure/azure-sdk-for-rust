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

            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;
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

            Ok(UserResponse::try_from(response).await?)
        })
    }
}

type CreateUser = futures::future::BoxFuture<'static, crate::Result<UserResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateUserBuilder {
    type Future = CreateUser;
    type Output = <CreateUser as std::future::Future>::Output;
    fn into_future(self) -> Self::Future {
        Self::into_future(self)
    }
}

#[derive(Serialize, Debug)]
struct CreateUserBody<'a> {
    id: &'a str,
}
