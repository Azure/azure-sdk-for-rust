use crate::prelude::*;
use crate::resources::trigger::*;
use crate::responses::CreateTriggerResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateOrReplaceTriggerBuilder<'a> {
    trigger_client: &'a TriggerClient,
    is_create: bool,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> CreateOrReplaceTriggerBuilder<'a> {
    pub(crate) fn new(trigger_client: &'a TriggerClient, is_create: bool) -> Self {
        Self {
            trigger_client,
            is_create,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a> CreateOrReplaceTriggerBuilder<'a> {
    setters! {
        user_agent: &'a str => Some(UserAgent::new(user_agent)),
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a> CreateOrReplaceTriggerBuilder<'a> {
    pub async fn execute<B, T, O>(
        &self,
        body: B,
        trigger_type: T,
        trigger_operation: O,
    ) -> crate::Result<CreateTriggerResponse>
    where
        B: AsRef<str>,
        T: Into<TriggerType>,
        O: Into<TriggerOperation>,
    {
        trace!("CreateOrReplaceTriggerBuilder::execute called");

        let req = self.trigger_client;
        let req = if self.is_create {
            req.prepare_request(http::Method::POST)
        } else {
            req.prepare_request_with_trigger_name(http::Method::PUT)
        };

        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Deserialize, Serialize)]
        struct Request<'a> {
            pub id: &'a str,
            #[serde(rename = "triggerOperation")]
            pub trigger_operation: TriggerOperation,
            #[serde(rename = "triggerType")]
            pub trigger_type: TriggerType,
            pub body: &'a str,
        }

        let request = Request {
            id: self.trigger_client.trigger_name(),
            trigger_operation: trigger_operation.into(),
            trigger_type: trigger_type.into(),
            body: body.as_ref(),
        };

        let request = azure_core::to_json(&request)?;
        let request = req.body(request)?;

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
