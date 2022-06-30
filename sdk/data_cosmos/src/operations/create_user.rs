use crate::{prelude::*, resources::user::UserResponse as CreateUserResponse};

operation! {
    CreateUser,
    client: UserClient,
    ?consistency_level: ConsistencyLevel
}

impl CreateUserBuilder {
    pub fn into_future(self) -> CreateUser {
        Box::pin(async move {
            let mut request = self.client.cosmos_client().request(
                &format!(
                    "dbs/{}/users",
                    self.client.database_client().database_name()
                ),
                azure_core::Method::Post,
            );

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            let body = CreateUserBody {
                id: self.client.user_name(),
            };
            request.set_body(serde_json::to_vec(&body)?);
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Users),
                    &mut request,
                )
                .await?;

            CreateUserResponse::try_from(response).await
        })
    }
}

#[derive(Serialize, Debug)]
struct CreateUserBody<'a> {
    id: &'a str,
}
