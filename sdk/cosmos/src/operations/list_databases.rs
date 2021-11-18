use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Database;
use crate::ResourceQuota;

use azure_core::headers::{
    self, continuation_token_from_headers_optional, session_token_from_headers,
};
use azure_core::{collect_pinned_stream, prelude::*, Request, Response};
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
}

impl ListDatabases {
    pub fn new(client: CosmosClient) -> Self {
        Self {
            client,
            consistency_level: None,
            max_item_count: MaxItemCount::new(-1),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
    }

    pub async fn decorate_request(&self, request: &mut Request) -> crate::Result<()> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        azure_core::headers::add_mandatory_header2(&self.max_item_count, request)?;
        Ok(())
    }

    fn into_stream(self, ctx: Context) -> Pageable<ListDatabasesResponse> {
        let make_request =
            move |this: ListDatabases, ctx: Context, continuation: Option<String>| async move {
                let mut request = this
                    .client
                    .prepare_request_pipeline("dbs", http::Method::GET);

                // if let Err(e) = options.decorate_request(&mut request).await {
                //     return Err(azure_core::Error::PolicyError(e.into()));
                // }
                // TODO inline options

                if let Some(c) = continuation {
                    match http::HeaderValue::from_str(c.as_str()) {
                        Ok(h) => request.headers_mut().append(headers::CONTINUATION, h),
                        Err(e) => return Err(azure_core::Error::PolicyError(e.into())),
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
                    Err(e) => return Err(e),
                };

                Ok(ListDatabasesResponse::try_from(response).await)
            };

        Pageable::new(self, ctx, Box::new(make_request))
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
    // TODO: To remove pragma when list_databases has been re-enabled
    #[allow(dead_code)]
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
    // make_request: Box<dyn Fn(O, Context) -> F>,
    stream: Box<dyn Stream<Item = crate::Result<T>>>,
}

impl<T: Continuable> Pageable<T> {
    fn new<O, F>(
        builder: O,
        ctx: Context,
        make_request: Box<dyn Fn(O, Context, Option<String>) -> F>,
    ) -> Self
    where
        O: Clone + 'static,
        F: std::future::Future<Output = crate::Result<crate::Result<T>>> + 'static,
    {
        let stream = unfold(State::Init, move |state: State| {
            let this = builder.clone();
            let ctx = ctx.clone();
            async move {
                let response = match state {
                    State::Init => r#try!(make_request(this, ctx, None).await),
                    State::Continuation(token) => {
                        r#try!(make_request(this, ctx, Some(token)).await)
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
            stream: Box::new(stream),
        }
    }
}

trait Continuable {
    fn continuation(&self) -> Option<String>;
}

// impl<O> Stream for Pageable<O, F, T> {
//     type Item = crate::Result<T>;

//     fn poll_next(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<Self::Item>> {
//         todo!()
//     }
// }

#[derive(Debug, Clone, PartialEq)]
enum State {
    Init,
    Continuation(String),
    Done,
}
