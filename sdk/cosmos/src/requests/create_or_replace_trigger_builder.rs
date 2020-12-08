use crate::prelude::*;
use crate::resources::trigger::*;
use crate::responses::CreateTriggerResponse;
use azure_core::{ActivityId, No, ToAssign, UserAgent, Yes};
use http::StatusCode;
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
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> CreateOrReplaceTriggerBuilder<'a, No, No, No> {
    pub(crate) fn new(trigger_client: &'a TriggerClient, is_create: bool) -> Self {
        Self {
            trigger_client,
            is_create,
            p_trigger_operation: PhantomData,
            trigger_operation: TriggerOperation::All,
            p_trigger_type: PhantomData,
            trigger_type: TriggerType::Pre,
            p_body: PhantomData,
            body: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
    CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    setters! {
        user_agent: &'a str => Some(UserAgent::new(user_agent)),
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a, TriggerTypeSet, BodySet> CreateOrReplaceTriggerBuilder<'a, Yes, TriggerTypeSet, BodySet>
where
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    fn trigger_operation(&self) -> TriggerOperation {
        self.trigger_operation
    }
}

impl<'a, TriggerOperationSet, BodySet>
    CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, Yes, BodySet>
where
    TriggerOperationSet: ToAssign,
    BodySet: ToAssign,
{
    fn trigger_type(&self) -> TriggerType {
        self.trigger_type
    }
}

impl<'a, TriggerOperationSet, TriggerTypeSet>
    CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, Yes>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
{
    fn body(&self) -> &'a str {
        self.body.unwrap()
    }
}

impl<'a, TriggerTypeSet, BodySet> CreateOrReplaceTriggerBuilder<'a, No, TriggerTypeSet, BodySet>
where
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
{
    pub fn with_trigger_operation(
        self,
        trigger_operation: TriggerOperation,
    ) -> CreateOrReplaceTriggerBuilder<'a, Yes, TriggerTypeSet, BodySet> {
        CreateOrReplaceTriggerBuilder {
            trigger_operation,
            trigger_client: self.trigger_client,
            is_create: self.is_create,
            trigger_type: self.trigger_type,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_trigger_operation: PhantomData,
            p_trigger_type: PhantomData,
            p_body: PhantomData,
        }
    }
}

impl<'a, TriggerOperationSet, BodySet>
    CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, No, BodySet>
where
    TriggerOperationSet: ToAssign,
    BodySet: ToAssign,
{
    pub fn with_trigger_type(
        self,
        trigger_type: TriggerType,
    ) -> CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, Yes, BodySet> {
        CreateOrReplaceTriggerBuilder {
            trigger_type,
            trigger_client: self.trigger_client,
            is_create: self.is_create,
            trigger_operation: self.trigger_operation,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_trigger_operation: PhantomData,
            p_trigger_type: PhantomData,
            p_body: PhantomData,
        }
    }
}

impl<'a, TriggerOperationSet, TriggerTypeSet>
    CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, No>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
{
    pub fn with_body(
        self,
        body: &'a str,
    ) -> CreateOrReplaceTriggerBuilder<'a, TriggerOperationSet, TriggerTypeSet, Yes> {
        CreateOrReplaceTriggerBuilder {
            body: Some(body),
            trigger_client: self.trigger_client,
            is_create: self.is_create,
            trigger_operation: self.trigger_operation,
            trigger_type: self.trigger_type,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_trigger_operation: PhantomData,
            p_trigger_type: PhantomData,
            p_body: PhantomData,
        }
    }
}

impl<'a> CreateOrReplaceTriggerBuilder<'a, Yes, Yes, Yes> {
    pub async fn execute(&self) -> Result<CreateTriggerResponse, CosmosError> {
        trace!("CreateOrReplaceTriggerBuilder::execute called");

        let req = self.trigger_client;
        let req = if self.is_create {
            req.prepare_request(http::Method::POST)
        } else {
            req.prepare_request_with_trigger_name(http::Method::PUT)
        };

        // add trait headers
        let req = crate::headers::add_header(self.user_agent, req);
        let req = crate::headers::add_header(self.activity_id, req);
        let req = crate::headers::add_header(self.consistency_level.clone(), req);

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
        let request = req.body(request.as_bytes())?;

        let expected_status = if self.is_create {
            StatusCode::CREATED
        } else {
            StatusCode::OK
        };

        Ok(self
            .trigger_client
            .http_client()
            .execute_request_check_status(request, expected_status)
            .await?
            .try_into()?)
    }
}
