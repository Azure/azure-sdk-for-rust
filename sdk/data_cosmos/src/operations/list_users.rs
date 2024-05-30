use crate::{
    headers::from_headers::{activity_id_from_headers, request_charge_from_headers},
    prelude::*,
};
use azure_core::{
    headers::{continuation_token_from_headers_optional, session_token_from_headers},
    prelude::Continuation,
    prelude::MaxItemCount,
    Continuable, Pageable, Response as HttpResponse, SessionToken,
};

operation! {
    #[stream]
    ListUsers,
    client: DatabaseClient,
    ?max_item_count: MaxItemCount,
    ?consistency_level: ConsistencyLevel,
    ?continuation: Continuation,
}

impl ListUsersBuilder {
    pub fn into_stream(self) -> ListUsers {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut request = this.client.cosmos_client().request(
                    &format!("dbs/{}/users", this.client.database_name()),
                    azure_core::Method::GET,
                );

                if let Some(cl) = &this.consistency_level {
                    request.insert_headers(cl);
                }
                request.insert_headers(&this.max_item_count.unwrap_or_default());

                let continuation = continuation.or(this.continuation);
                request.insert_headers(&continuation);

                let response = this
                    .client
                    .cosmos_client()
                    .send(request, ctx.clone(), ResourceType::Users)
                    .await?;
                ListUsersResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

pub type ListUsers = Pageable<ListUsersResponse, azure_core::error::Error>;

#[derive(Debug, Clone)]
pub struct ListUsersResponse {
    pub users: Vec<User>,
    pub rid: String,
    pub count: u32,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: SessionToken,
    pub continuation_token: Option<Continuation>,
}

impl ListUsersResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();

        #[derive(Deserialize, Debug)]
        pub struct Response {
            #[serde(rename = "_rid")]
            rid: String,
            #[serde(rename = "Users")]
            pub users: Vec<User>,
            #[serde(rename = "_count")]
            pub count: u32,
        }

        let response: Response = body.json().await?;

        Ok(Self {
            users: response.users,
            rid: response.rid,
            count: response.count,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            continuation_token: continuation_token_from_headers_optional(&headers)?,
        })
    }
}

impl IntoIterator for ListUsersResponse {
    type Item = User;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.users.into_iter()
    }
}

impl Continuable for ListUsersResponse {
    type Continuation = Continuation;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation_token.clone()
    }
}
