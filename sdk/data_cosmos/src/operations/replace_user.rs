use crate::{prelude::*, resources::user::UserResponse as ReplaceUserResponse};

operation! {
    ReplaceUser,
    client: UserClient,
    user_name: String,
    ?consistency_level: ConsistencyLevel
}

impl ReplaceUserBuilder {
    pub fn into_future(self) -> ReplaceUser {
        Box::pin(async move {
            let mut request = self.client.user_request(azure_core::Method::Put);

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            let body = ReplaceUserBody {
                id: &self.user_name,
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

            ReplaceUserResponse::try_from(response).await
        })
    }
}

#[derive(Serialize)]
struct ReplaceUserBody<'a> {
    id: &'a str,
}
