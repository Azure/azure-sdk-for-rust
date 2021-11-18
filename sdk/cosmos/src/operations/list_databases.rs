use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Database;
use crate::ResourceQuota;

use azure_core::headers::{
    self, continuation_token_from_headers_optional, session_token_from_headers,
};
use azure_core::{collect_pinned_stream, prelude::*, Response};
use chrono::{DateTime, Utc};
use futures::stream::unfold;
use futures::Stream;

macro_rules! r#try {
    ($expr:expr $(,)?) => {
        match $expr {
            Result::Ok(val) => val,
            Result::Err(err) => {
                return Some((Err(err.into()), State::Done));
            }
        }
    };
}
#[derive(Debug, Clone)]
pub struct ListDatabases {
    client: CosmosClient,
    consistency_level: Option<ConsistencyLevel>,
    max_item_count: MaxItemCount,
    context: Option<Context>,
}

impl ListDatabases {
    pub fn new(client: CosmosClient) -> Self {
        Self {
            client,
            consistency_level: None,
            max_item_count: MaxItemCount::new(-1),
            context: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
        context: Context => Some(context),
    }

    pub fn into_stream(self) -> Pageable<ListDatabasesResponse> {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let ctx = self.context.clone().unwrap_or_default();
            async move {
                let mut request = this
                    .client
                    .prepare_request_pipeline("dbs", http::Method::GET);

                azure_core::headers::add_optional_header2(&this.consistency_level, &mut request)?;
                azure_core::headers::add_mandatory_header2(&this.max_item_count, &mut request)?;

                if let Some(c) = continuation {
                    match http::HeaderValue::from_str(c.as_str()) {
                        Ok(h) => request.headers_mut().append(headers::CONTINUATION, h),
                        Err(e) => return Err(crate::Error::InvalidHeaderValue(e.into())),
                    };
                }

                let response = match this
                    .client
                    .pipeline()
                    .send(
                        &mut ctx.clone().insert(ResourceType::Databases),
                        &mut request,
                    )
                    .await
                {
                    Ok(r) => r,
                    Err(e) => return Err(crate::Error::Core(e)),
                };

                Ok(ListDatabasesResponse::try_from(response).await)
            }
        };

        Pageable::new(std::sync::Arc::new(make_request))
    }
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
    pub(crate) async fn try_from(response: Response) -> crate::Result<Self> {
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

impl Continuable for ListDatabasesResponse {
    fn continuation(&self) -> Option<String> {
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

pub struct Pageable<T> {
    stream: std::pin::Pin<Box<dyn Stream<Item = crate::Result<T>>>>,
}

impl<T: Continuable> Pageable<T> {
    fn new<F>(make_request: std::sync::Arc<dyn Fn(Option<String>) -> F>) -> Self
    where
        F: std::future::Future<Output = crate::Result<crate::Result<T>>> + 'static,
    {
        let stream = unfold(State::Init, move |state: State| {
            let make_request = make_request.clone();
            async move {
                let response = match state {
                    State::Init => r#try!(make_request(None).await),
                    State::Continuation(token) => {
                        r#try!(make_request(Some(token)).await)
                    }
                    State::Done => return None,
                };

                let response = r#try!(response);

                let next_state = response
                    .continuation()
                    .map(State::Continuation)
                    .unwrap_or(State::Done);

                Some((Ok(response), next_state))
            }
        });
        Self {
            stream: Box::pin(stream),
        }
    }
}

pub trait Continuable {
    fn continuation(&self) -> Option<String>;
}

impl<T> Stream for Pageable<T> {
    type Item = crate::Result<T>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::pin::Pin::new(&mut self.stream).poll_next(cx)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Init,
    Continuation(String),
    Done,
}
