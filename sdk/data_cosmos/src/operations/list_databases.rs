use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Database;
use crate::ResourceQuota;

use azure_core::headers::{continuation_token_from_headers_optional, session_token_from_headers};
use azure_core::{prelude::*, Pageable, Response};
use time::OffsetDateTime;

operation! {
    #[stream]
    ListDatabases,
    client: CosmosClient,
    ?max_item_count: MaxItemCount,
    ?consistency_level: ConsistencyLevel
}

impl ListDatabasesBuilder {
    pub fn into_stream(self) -> ListDatabases {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut request = this.client.request("dbs", azure_core::Method::Get);
                if let Some(cl) = &this.consistency_level {
                    request.insert_headers(cl);
                }
                request.insert_headers(&this.max_item_count.unwrap_or_default());
                request.insert_headers(&continuation);

                let response = this
                    .client
                    .pipeline()
                    .send(ctx.clone().insert(ResourceType::Databases), &mut request)
                    .await?;

                ListDatabasesResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

pub type ListDatabases = Pageable<ListDatabasesResponse, azure_core::error::Error>;

#[derive(Clone, Debug)]
pub struct ListDatabasesResponse {
    pub rid: String,
    pub databases: Vec<Database>,
    pub count: u32,
    pub activity_id: uuid::Uuid,
    pub charge: f64,
    pub session_token: String,
    pub last_state_change: OffsetDateTime,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub schema_version: String,
    pub service_version: String,
    pub continuation_token: Option<Continuation>,
    pub gateway_version: String,
}

impl ListDatabasesResponse {
    pub(crate) async fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        #[derive(Deserialize, Debug)]
        pub struct Response {
            #[serde(rename = "_rid")]
            rid: String,
            #[serde(rename = "Databases")]
            pub databases: Vec<Database>,
            #[serde(rename = "_count")]
            pub count: u32,
        }

        let response: Response = serde_json::from_slice(&body)?;

        Ok(Self {
            rid: response.rid,
            databases: response.databases,
            count: response.count,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            continuation_token: continuation_token_from_headers_optional(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
        })
    }
}

impl Continuable for ListDatabasesResponse {
    type Continuation = Continuation;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation_token.clone()
    }
}

impl IntoIterator for ListDatabasesResponse {
    type Item = Database;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.databases.into_iter()
    }
}
