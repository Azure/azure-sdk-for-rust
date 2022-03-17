use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Permission;
use crate::resources::ResourceType;
use azure_core::collect_pinned_stream;
use azure_core::headers::{continuation_token_from_headers_optional, session_token_from_headers};
use azure_core::prelude::*;
use azure_core::{Pageable, Response as HttpResponse};

#[derive(Debug, Clone)]
pub struct ListPermissionsBuilder {
    client: UserClient,
    consistency_level: Option<ConsistencyLevel>,
    max_item_count: MaxItemCount,
    context: Context,
}

impl ListPermissionsBuilder {
    pub(crate) fn new(client: UserClient) -> Self {
        Self {
            client,
            consistency_level: None,
            max_item_count: MaxItemCount::new(-1),
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
        context: Context => context,
    }

    pub fn into_stream(self) -> ListPermissions {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut request = this.client.cosmos_client().prepare_request_pipeline(
                    &format!(
                        "dbs/{}/users/{}/permissions",
                        this.client.database_client().database_name(),
                        this.client.user_name()
                    ),
                    http::Method::GET,
                );

                azure_core::headers::add_optional_header2(&this.consistency_level, &mut request)?;
                azure_core::headers::add_mandatory_header2(&this.max_item_count, &mut request)?;

                if let Some(ref c) = continuation {
                    request.insert_header(c)?;
                }

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

pub type ListPermissions = Pageable<ListPermissionsResponse, crate::Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct ListPermissionsResponse {
    pub permissions: Vec<Permission>,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
    pub continuation_token: Option<String>,
}

impl ListPermissionsResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

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
            content_path: content_path_from_headers(&headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            continuation_token: continuation_token_from_headers_optional(&headers)?,
        })
    }
}

impl Continuable for ListPermissionsResponse {
    fn continuation(&self) -> Option<String> {
        self.continuation_token.clone()
    }
}
