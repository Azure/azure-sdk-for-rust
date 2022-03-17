use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::ResourceType;
use crate::resources::UserDefinedFunction;
use crate::ResourceQuota;
use azure_core::collect_pinned_stream;
use azure_core::headers::{
    continuation_token_from_headers_optional, item_count_from_headers, session_token_from_headers,
};
use azure_core::{prelude::*, Pageable, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ListUserDefinedFunctionsBuilder {
    client: CollectionClient,
    if_match_condition: Option<IfMatchCondition>,
    consistency_level: Option<ConsistencyLevel>,
    max_item_count: MaxItemCount,
    context: Context,
}

impl ListUserDefinedFunctionsBuilder {
    pub(crate) fn new(client: CollectionClient) -> Self {
        Self {
            client,
            if_match_condition: None,
            consistency_level: None,
            max_item_count: MaxItemCount::new(-1),
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        context: Context => context,
    }

    pub fn into_stream(self) -> ListUserDefinedFunctions {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut request = this.client.cosmos_client().prepare_request_pipeline(
                    &format!(
                        "dbs/{}/colls/{}/udfs",
                        this.client.database_client().database_name(),
                        this.client.collection_name()
                    ),
                    http::Method::GET,
                );

                azure_core::headers::add_optional_header2(&this.if_match_condition, &mut request)?;
                azure_core::headers::add_optional_header2(&this.consistency_level, &mut request)?;
                azure_core::headers::add_mandatory_header2(&this.max_item_count, &mut request)?;

                if let Some(ref c) = continuation {
                    request.insert_header(c)?;
                }

                let response = this
                    .client
                    .pipeline()
                    .send(
                        ctx.clone().insert(ResourceType::UserDefinedFunctions),
                        &mut request,
                    )
                    .await?;
                ListUserDefinedFunctionsResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

pub type ListUserDefinedFunctions = Pageable<ListUserDefinedFunctionsResponse, crate::Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct ListUserDefinedFunctionsResponse {
    pub rid: String,
    pub user_defined_functions: Vec<UserDefinedFunction>,
    pub content_location: String,
    pub server: String,
    pub last_state_change: DateTime<Utc>,
    pub continuation_token: Option<String>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub item_count: u32,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub session_token: String,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl ListUserDefinedFunctionsResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        #[derive(Debug, Deserialize)]
        struct Response<'a> {
            #[serde(rename = "_rid")]
            rid: &'a str,
            #[serde(rename = "UserDefinedFunctions")]
            user_defined_functions: Vec<UserDefinedFunction>,
            #[serde(rename = "_count")]
            #[allow(unused)]
            count: u32,
        }
        let response: Response = serde_json::from_slice(&body)?;

        Ok(Self {
            rid: response.rid.to_owned(),
            user_defined_functions: response.user_defined_functions,
            content_location: content_location_from_headers(&headers)?.to_owned(),
            server: server_from_headers(&headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(&headers)?,
            continuation_token: continuation_token_from_headers_optional(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            item_count: item_count_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            role: role_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            date: date_from_headers(&headers)?,
        })
    }
}

impl Continuable for ListUserDefinedFunctionsResponse {
    fn continuation(&self) -> Option<String> {
        self.continuation_token.clone()
    }
}
