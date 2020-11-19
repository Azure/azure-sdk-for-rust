use crate::prelude::*;
use crate::responses::CreateTriggerResponse;
use crate::trigger::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    trigger_client: &'a TriggerClient,
    is_create: bool,
    p_trigger_operation: PhantomData<TriggerOperationSet>,
    p_trigger_type: PhantomData<TriggerTypeSet>,
    p_body: PhantomData<BodySet>,
    trigger_operation: TriggerOperation,
    trigger_type: TriggerType,
    body: Option<&'a str>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> CreateOrReplaceTriggerBuilder<'a, No, No, No> {
    pub(crate) fn new(trigger_client: &'a TriggerClient, is_create: bool) -> Self {
        Self {
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

impl<'a, TriggerOperationSet, TriggerTypeSet, BodySet> TriggerClientRequired<'a>
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    fn trigger_client(&self) -> &'a TriggerClient {
        self.trigger_client
    }
}

//set mandatory no traits methods
impl<'a, TriggerTypeSet, BodySet> TriggerOperationRequired
    for CreateOrReplaceTriggerBuilder<'a, Yes, TriggerTypeSet, BodySet>
where
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    fn trigger_operation(&self) -> TriggerOperation {
        self.trigger_operation
    }
}

impl<'a, TriggerOperationSet, BodySet> TriggerTypeRequired
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, Yes, BodySet>
where
    TriggerOperationSet: ToAssign,
    BodySet: ToAssign,
{
    fn trigger_type(&self) -> TriggerType {
        self.trigger_type
    }
}

impl<'a, TriggerOperationSet, TriggerTypeSet> TriggerBodyRequired<'a>
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, Yes>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
{
    fn body(&self) -> &'a str {
        self.body.unwrap()
    }
}

impl<'a, TriggerOperationSet, TriggerTypeSet, BodySet> UserAgentOption<'a>
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, TriggerOperationSet, TriggerTypeSet, BodySet> ActivityIdOption<'a>
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, TriggerOperationSet, TriggerTypeSet, BodySet> ConsistencyLevelOption<'a>
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, TriggerTypeSet, BodySet> TriggerOperationSupport
    for CreateOrReplaceTriggerBuilder<'a, No, TriggerTypeSet, BodySet>
where
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CreateOrReplaceTriggerBuilder<'a, Yes, TriggerTypeSet, BodySet>;

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

impl<'a, TriggerOperationSet, BodySet> TriggerTypeSupport
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, No, BodySet>
where
    TriggerOperationSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, Yes, BodySet>;

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

impl<'a, TriggerOperationSet, TriggerTypeSet> TriggerBodySupport<'a>
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, No>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
{
    type O = CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, Yes>;

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

impl<'a, TriggerOperationSet, TriggerTypeSet, BodySet> UserAgentSupport<'a>
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, TriggerOperationSet, TriggerTypeSet, BodySet> ActivityIdSupport<'a>
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = Self;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, TriggerOperationSet, TriggerTypeSet, BodySet> ConsistencyLevelSupport<'a>
    for CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable regardless
impl<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
    CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    fn is_create(&self) -> bool {
        self.is_create
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> CreateOrReplaceTriggerBuilder<'a, Yes, Yes, Yes> {
    pub async fn execute(&self) -> Result<CreateTriggerResponse, AzureError> {
        trace!("CreateOrReplaceTriggerBuilder::execute called");

        let req = self.trigger_client;
        let req = if self.is_create() {
            req.prepare_request(hyper::Method::POST)
        } else {
            req.prepare_request_with_trigger_name(hyper::Method::PUT)
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
            id: self.trigger_client.trigger_name(),
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
