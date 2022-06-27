use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::ResourceType;
use crate::resources::StoredProcedure;
use crate::ResourceQuota;
use azure_core::collect_pinned_stream;
use azure_core::headers::{continuation_token_from_headers_optional, session_token_from_headers};
use azure_core::prelude::*;
use azure_core::{Pageable, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ListStoredProceduresBuilder {
    client: CollectionClient,
    consistency_level: Option<ConsistencyLevel>,
    max_item_count: MaxItemCount,
    context: Context,
}

impl ListStoredProceduresBuilder {
    pub(crate) fn new(client: CollectionClient) -> Self {
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

    pub fn into_stream(self) -> ListStoredProcedures {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut request = this.client.cosmos_client().request(
                    &format!(
                        "dbs/{}/colls/{}/sprocs",
                        this.client.database_client().database_name(),
                        this.client.collection_name(),
                    ),
                    azure_core::Method::Get,
                );

                if let Some(cl) = &this.consistency_level {
                    request.insert_headers(cl);
                }
                request.insert_headers(&this.max_item_count);

                request.insert_headers(&continuation);

                let response = this
                    .client
                    .pipeline()
                    .send(
                        ctx.clone().insert(ResourceType::StoredProcedures),
                        &mut request,
                    )
                    .await?;
                ListStoredProceduresResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

pub type ListStoredProcedures = Pageable<ListStoredProceduresResponse, azure_core::error::Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct ListStoredProceduresResponse {
    pub stored_procedures: Vec<StoredProcedure>,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub last_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub gateway_version: String,
    pub continuation_token: Option<String>,
}

impl ListStoredProceduresResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Response {
            pub _rid: String,
            #[serde(rename = "StoredProcedures")]
            pub stored_procedures: Vec<StoredProcedure>,
            pub _count: u64,
        }

        Ok(Self {
            stored_procedures: serde_json::from_slice::<Response>(&body)?.stored_procedures,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            last_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            continuation_token: continuation_token_from_headers_optional(&headers)?,
        })
    }
}

impl Continuable for ListStoredProceduresResponse {
    fn continuation(&self) -> Option<Continuation> {
        self.continuation_token.clone().map(Continuation::from)
    }
}
