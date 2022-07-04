use crate::{prelude::*, resources::user::UserResponse as GetUserResponse};

operation! {
    GetUser,
    client: UserClient,
    ?consistency_level: ConsistencyLevel
}

impl GetUserBuilder {
    pub fn into_future(self) -> GetUser {
        Box::pin(async move {
            let mut request = self.client.user_request(azure_core::Method::Get);

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

            GetUserResponse::try_from(response).await
        })
    }
}
