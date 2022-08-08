use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Permission;
use crate::resources::ResourceType;

use azure_core::headers::{continuation_token_from_headers_optional, session_token_from_headers};
use azure_core::prelude::*;
use azure_core::{Pageable, Response as HttpResponse};

operation! {
    #[stream]
    ListPermissions,
    client: UserClient,
    ?max_item_count: MaxItemCount,
    ?consistency_level: ConsistencyLevel
}

impl ListPermissionsBuilder {
    pub fn into_stream(self) -> ListPermissions {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut request = this.client.cosmos_client().request(
                    &format!(
                        "dbs/{}/users/{}/permissions",
                        this.client.database_client().database_name(),
                        this.client.user_name()
                    ),
                    azure_core::Method::Get,
                );

                if let Some(cl) = &this.consistency_level {
                    request.insert_headers(cl);
                }
                request.insert_headers(&this.max_item_count.unwrap_or_default());

                request.insert_headers(&continuation);

                let response = this
                    .client
                    .pipeline()
                    .send(ctx.clone().insert(ResourceType::Permissions), &mut request)
                    .await?;
                ListPermissionsResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

pub type ListPermissions = Pageable<ListPermissionsResponse, azure_core::error::Error>;

#[derive(Debug, Clone)]
pub struct ListPermissionsResponse {
    pub permissions: Vec<Permission>,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
    pub continuation_token: Option<Continuation>,
}

impl ListPermissionsResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        #[derive(Debug, Deserialize)]
        struct Response {
            _rid: String,
            #[serde(rename = "Permissions")]
            permissions: Vec<Permission>,
            _count: u32,
        }

        let response: Response = serde_json::from_slice(&body)?;
        let permissions = response.permissions;

        Ok(Self {
            permissions,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
            continuation_token: continuation_token_from_headers_optional(&headers)?,
        })
    }
}

impl Continuable for ListPermissionsResponse {
    type Continuation = Continuation;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation_token.clone()
    }
}
