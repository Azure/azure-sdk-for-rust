use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Collection;
use crate::ResourceQuota;
use azure_core::collect_pinned_stream;
use azure_core::headers::{continuation_token_from_headers_optional, session_token_from_headers};
use azure_core::prelude::*;
use azure_core::Request as HttpRequest;
use azure_core::Response as HttpResponse;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ListCollectionsOptions {
    consistency_level: Option<ConsistencyLevel>,
    max_item_count: MaxItemCount,
}

impl ListCollectionsOptions {
    pub fn new() -> Self {
        Self {
            max_item_count: MaxItemCount::new(-1),
            consistency_level: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
    }

    pub fn decorate_request(&self, request: &mut HttpRequest) -> crate::Result<()> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        azure_core::headers::add_mandatory_header2(&self.max_item_count, request)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListCollectionsResponse {
    pub rid: String,
    pub collections: Vec<Collection>,
    pub count: u32,
    pub last_state_change: DateTime<Utc>,
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
    pub continuation_token: Option<String>,
}

#[async_trait::async_trait]
impl azure_core::util::AsyncTryFrom<HttpResponse> for ListCollectionsResponse {
    type Error = crate::Error;

    async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

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
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            continuation_token: continuation_token_from_headers_optional(&headers)?,
        })
    }
}
