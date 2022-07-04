use crate::headers::from_headers::*;
use crate::prelude::*;
use azure_core::{headers::session_token_from_headers, Response as HttpResponse};

operation! {
    DeleteUser,
    client: UserClient,
    ?consistency_level: ConsistencyLevel
}

impl DeleteUserBuilder {
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
