use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Database;
use crate::ResourceQuota;

use azure_core::headers::{continuation_token_from_headers_optional, session_token_from_headers};
use azure_core::{collect_pinned_stream, prelude::*, Request, Response};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ListDatabasesOptions {
    consistency_level: Option<ConsistencyLevel>,
    max_item_count: MaxItemCount,
}

impl ListDatabasesOptions {
    pub fn new() -> Self {
        Self {
            consistency_level: None,
            max_item_count: MaxItemCount::new(-1),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
    }

    pub async fn decorate_request(&self, request: &mut Request) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        azure_core::headers::add_mandatory_header2(&self.max_item_count, request)?;
        Ok(())
    }

    // pub fn stream(&self) -> impl Stream<Item = Result<ListDatabasesResponse, crate::Error>> + '_ {
    //     #[derive(Debug, Clone, PartialEq)]
    //     enum States {
    //         Init,
    //         Continuation(String),
    //     }

    //     unfold(
    //         Some(States::Init),
    //         move |continuation_token: Option<States>| {
    //             async move {
    //                 debug!("continuation_token == {:?}", &continuation_token);
    //                 let response = match continuation_token {
    //                     Some(States::Init) => self.decorate_request().await,
    //                     Some(States::Continuation(continuation_token)) => {
    //                         self.clone()
    //                             .continuation(continuation_token.as_str())
    //                             .decorate_request()
    //                             .await
    //                     }
    //                     None => return None,
    //                 };

    //                 // the ? operator does not work in async move (yet?)
    //                 // so we have to resort to this boilerplate
    //                 let response = match response {
    //                     Ok(response) => response,
    //                     Err(err) => return Some((Err(err), None)),
    //                 };

    //                 let continuation_token = response
    //                     .continuation_token
    //                     .as_ref()
    //                     .map(|ct| States::Continuation(ct.to_owned()));

    //                 Some((Ok(response), continuation_token))
    //             }
    //         },
    //     )
    // }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct ListDatabasesResponse {
    pub rid: String,
    pub databases: Vec<Database>,
    pub count: u32,
    pub activity_id: uuid::Uuid,
    pub charge: f64,
    pub session_token: String,
    pub last_state_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub schema_version: String,
    pub service_version: String,
    pub continuation_token: Option<String>,
    pub gateway_version: String,
}

impl ListDatabasesResponse {
    pub(crate) async fn try_from(response: Response) -> Result<Self, crate::Error> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

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
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            service_version: service_version_from_headers(&headers)?.to_owned(),
            continuation_token: continuation_token_from_headers_optional(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
        })
    }
}
