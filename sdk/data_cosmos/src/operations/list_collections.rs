use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Collection;
use crate::ResourceQuota;
use azure_core::headers::{continuation_token_from_headers_optional, session_token_from_headers};
use azure_core::prelude::*;
use azure_core::Pageable;
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

operation! {
    #[stream]
    ListCollections,
    client: DatabaseClient,
    ?max_item_count: MaxItemCount,
    ?consistency_level: ConsistencyLevel
}

impl ListCollectionsBuilder {
    pub fn into_stream(self) -> ListCollections {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut request = this.client.collections_request(azure_core::Method::Get);
                if let Some(cl) = &this.consistency_level {
                    request.insert_headers(cl);
                }
                request.insert_headers(&this.max_item_count.unwrap_or_default());

                request.insert_headers(&continuation);

                let response = this
                    .client
                    .cosmos_client()
                    .send(request, ctx.clone(), ResourceType::Collections)
                    .await?;
                ListCollectionsResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

pub type ListCollections = Pageable<ListCollectionsResponse, azure_core::error::Error>;

#[derive(Debug, Clone)]
pub struct ListCollectionsResponse {
    pub rid: String,
    pub collections: Vec<Collection>,
    pub count: u32,
    pub last_state_change: OffsetDateTime,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub gateway_version: String,
    pub continuation_token: Option<Continuation>,
}

impl ListCollectionsResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        #[derive(Deserialize, Debug)]
        pub struct Response {
            _rid: String,
            #[serde(rename = "DocumentCollections")]
            pub collections: Vec<Collection>,
            #[serde(rename = "_count")]
            pub count: u32,
        }

        let response: Response = serde_json::from_slice(&*body)?;

        Ok(Self {
            rid: response._rid,
            collections: response.collections,
            count: response.count,
            last_state_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            continuation_token: continuation_token_from_headers_optional(&headers)?,
        })
    }
}

impl Continuable for ListCollectionsResponse {
    type Continuation = Continuation;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation_token.clone()
    }
}
