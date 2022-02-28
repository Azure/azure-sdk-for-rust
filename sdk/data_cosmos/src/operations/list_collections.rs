use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Collection;
use crate::ResourceQuota;
use azure_core::collect_pinned_stream;
use azure_core::headers::{continuation_token_from_headers_optional, session_token_from_headers};
use azure_core::prelude::*;
use azure_core::Response as HttpResponse;
use chrono::{DateTime, Utc};
use futures::stream::unfold;
use futures::Stream;

/// Macro for short cutting a stream on error
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

/// Stream state
#[derive(Debug, Clone, PartialEq)]
enum State {
    Init,
    Continuation(String),
    Done,
}
#[derive(Debug, Clone)]
pub struct ListCollectionsBuilder {
    client: DatabaseClient,
    consistency_level: Option<ConsistencyLevel>,
    max_item_count: MaxItemCount,
    context: Context,
}

impl ListCollectionsBuilder {
    pub(crate) fn new(client: DatabaseClient) -> Self {
        Self {
            client,
            max_item_count: MaxItemCount::new(-1),
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
        context: Context => context,
    }

    pub fn into_stream(
        self,
    ) -> impl Stream<Item = crate::Result<ListCollectionsResponse>> + 'static {
        unfold(State::Init, move |state: State| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let response = match state {
                    State::Init => {
                        let mut request = this.client.cosmos_client().prepare_request_pipeline(
                            &format!("dbs/{}/colls", this.client.database_name()),
                            http::Method::GET,
                        );

                        r#try!(azure_core::headers::add_optional_header2(
                            &this.consistency_level,
                            &mut request,
                        ));
                        r#try!(azure_core::headers::add_mandatory_header2(
                            &this.max_item_count,
                            &mut request,
                        ));
                        let response = r#try!(
                            this.client
                                .pipeline()
                                .send(ctx.clone().insert(ResourceType::Collections), &mut request)
                                .await
                        );
                        ListCollectionsResponse::try_from(response).await
                    }
                    State::Continuation(continuation_token) => {
                        let continuation = Continuation::new(continuation_token.as_str());
                        let mut request = this.client.cosmos_client().prepare_request_pipeline(
                            &format!("dbs/{}/colls", this.client.database_name()),
                            http::Method::GET,
                        );

                        r#try!(azure_core::headers::add_optional_header2(
                            &this.consistency_level,
                            &mut request,
                        ));
                        r#try!(azure_core::headers::add_mandatory_header2(
                            &this.max_item_count,
                            &mut request,
                        ));
                        r#try!(continuation.add_as_header2(&mut request));
                        let response = r#try!(
                            this.client
                                .pipeline()
                                .send(ctx.clone().insert(ResourceType::Collections), &mut request)
                                .await
                        );
                        ListCollectionsResponse::try_from(response).await
                    }
                    State::Done => return None,
                };

                let response = r#try!(response);

                let next_state = response
                    .continuation_token
                    .clone()
                    .map(State::Continuation)
                    .unwrap_or(State::Done);

                Some((Ok(response), next_state))
            }
        })
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

impl ListCollectionsResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
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

/// The future returned by calling `into_future` on the builder.
pub type ListCollections =
    futures::future::BoxFuture<'static, crate::Result<ListCollectionsResponse>>;
