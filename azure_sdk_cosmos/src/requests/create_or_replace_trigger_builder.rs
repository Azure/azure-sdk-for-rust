use crate::clients::CosmosUriBuilder;
use crate::prelude::*;
use crate::responses::CreateTriggerResponse;
use crate::trigger::*;
use crate::TriggerClient;
use crate::TriggerClientRequired;
use crate::{TriggerBuilderTrait, TriggerTrait};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    trigger_client: &'a TriggerClient<'a, CUB>,
    is_create: bool,
    p_trigger_operation: PhantomData<TriggerOperationSet>,
    p_trigger_type: PhantomData<TriggerTypeSet>,
    p_body: PhantomData<BodySet>,
    trigger_operation: TriggerOperation,
    trigger_type: TriggerType,
    body: Option<&'a str>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, CUB> CreateOrReplaceTriggerBuilder<'a, CUB, No, No, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        trigger_client: &'a TriggerClient<'a, CUB>,
        is_create: bool,
    ) -> CreateOrReplaceTriggerBuilder<'a, CUB, No, No, No> {
        CreateOrReplaceTriggerBuilder {
            trigger_client,
            is_create,
            p_trigger_operation: PhantomData {},
            trigger_operation: TriggerOperation::All,
            p_trigger_type: PhantomData {},
            trigger_type: TriggerType::Pre,
            p_body: PhantomData {},
            body: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> TriggerClientRequired<'a, CUB>
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn trigger_client(&self) -> &'a TriggerClient<'a, CUB> {
        self.trigger_client
    }
}

//set mandatory no traits methods
impl<'a, CUB, TriggerTypeSet, BodySet> TriggerOperationRequired
    for CreateOrReplaceTriggerBuilder<'a, CUB, Yes, TriggerTypeSet, BodySet>
where
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn trigger_operation(&self) -> TriggerOperation {
        self.trigger_operation
    }
}

impl<'a, CUB, TriggerOperationSet, BodySet> TriggerTypeRequired
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, Yes, BodySet>
where
    TriggerOperationSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn trigger_type(&self) -> TriggerType {
        self.trigger_type
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet> TriggerBodyRequired<'a>
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, Yes>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn body(&self) -> &'a str {
        self.body.unwrap()
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> UserAgentOption<'a>
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> ActivityIdOption<'a>
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> ConsistencyLevelOption<'a>
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, CUB, TriggerTypeSet, BodySet> TriggerOperationSupport
    for CreateOrReplaceTriggerBuilder<'a, CUB, No, TriggerTypeSet, BodySet>
where
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateOrReplaceTriggerBuilder<'a, CUB, Yes, TriggerTypeSet, BodySet>;

    #[inline]
    fn with_trigger_operation(self, trigger_operation: TriggerOperation) -> Self::O {
        CreateOrReplaceTriggerBuilder {
            trigger_client: self.trigger_client,
            is_create: self.is_create,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation,
            trigger_type: self.trigger_type,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, BodySet> TriggerTypeSupport
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, No, BodySet>
where
    TriggerOperationSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, Yes, BodySet>;

    #[inline]
    fn with_trigger_type(self, trigger_type: TriggerType) -> Self::O {
        CreateOrReplaceTriggerBuilder {
            trigger_client: self.trigger_client,
            is_create: self.is_create,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet> TriggerBodySupport<'a>
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, No>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, Yes>;

    #[inline]
    fn with_body(self, body: &'a str) -> Self::O {
        CreateOrReplaceTriggerBuilder {
            trigger_client: self.trigger_client,
            is_create: self.is_create,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type: self.trigger_type,
            body: Some(body),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> UserAgentSupport<'a>
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        CreateOrReplaceTriggerBuilder {
            trigger_client: self.trigger_client,
            is_create: self.is_create,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type: self.trigger_type,
            body: self.body,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> ActivityIdSupport<'a>
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        CreateOrReplaceTriggerBuilder {
            trigger_client: self.trigger_client,
            is_create: self.is_create,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type: self.trigger_type,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> ConsistencyLevelSupport<'a>
    for CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        CreateOrReplaceTriggerBuilder {
            trigger_client: self.trigger_client,
            is_create: self.is_create,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type: self.trigger_type,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable regardless
impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
    CreateOrReplaceTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn is_create(&self) -> bool {
        self.is_create
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> CreateOrReplaceTriggerBuilder<'a, CUB, Yes, Yes, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<CreateTriggerResponse, AzureError> {
        trace!("CreateOrReplaceTriggerBuilder::execute called");

        let req = self.trigger_client;
        let req = if self.is_create() {
            req.prepare_request(hyper::Method::POST, false)
        } else {
            req.prepare_request(hyper::Method::PUT, true)
        };

        // add trait headers
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
        struct _Request<'a> {
            pub id: &'a str,
            #[serde(rename = "triggerOperation")]
            pub trigger_operation: TriggerOperation,
            #[serde(rename = "triggerType")]
            pub trigger_type: TriggerType,
            pub body: &'a str,
        }

        let request = _Request {
            id: self.trigger_client.trigger_name().name(),
            trigger_operation: self.trigger_operation(),
            trigger_type: self.trigger_type(),
            body: self.body(),
        };

        let request = serde_json::to_string(&request)?;
        let request = req.body(hyper::Body::from(request))?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.trigger_client().hyper_client().request(request),
            if self.is_create() {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            },
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
