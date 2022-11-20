use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::add_rule::AddRuleResponse,
    management_constants::{
        operations::ADD_RULE_OPERATION,
        properties::{
            CONTENT_TYPE, CORRELATION_ID, CORRELATION_RULE_FILTER,
            CORRELATION_RULE_FILTER_PROPERTIES, EXPRESSION, LABEL, MESSAGE_ID, REPLY_TO,
            REPLY_TO_SESSION_ID, RULE_DESCRIPTION, RULE_NAME, SESSION_ID, SQL_RULE_ACTION,
            SQL_RULE_FILTER, TO,
        },
    },
};

use super::error::CorrelationFilterError;

// type SqlFilter = OrderedMap<String, String>;
// type CorrelationFilter = OrderedMap<String, String>;
// type SqlRuleAction = OrderedMap<String, String>;

pub struct SqlFilter {
    pub expression: String,
}

pub struct CorrelationFilter {
    pub correlation_id: Option<String>,
    pub message_id: Option<String>,
    pub to: Option<String>,
    pub reply_to: Option<String>,
    pub label: Option<String>,
    pub session_id: Option<String>,
    pub reply_to_session_id: Option<String>,
    pub content_type: Option<String>,
    pub properties: Option<OrderedMap<String, Value>>,
}

impl TryFrom<CorrelationFilter> for OrderedMap<Value, Value> {
    type Error = CorrelationFilterError;

    fn try_from(filter: CorrelationFilter) -> Result<Self, Self::Error> {
        let mut map = OrderedMap::new();
        if let Some(correlation_id) = filter.correlation_id {
            map.insert(
                Value::String(CORRELATION_ID.to_string()),
                Value::String(correlation_id),
            );
        }
        if let Some(message_id) = filter.message_id {
            map.insert(
                Value::String(MESSAGE_ID.to_string()),
                Value::String(message_id),
            );
        }
        if let Some(to) = filter.to {
            map.insert(Value::String(TO.to_string()), Value::String(to));
        }
        if let Some(reply_to) = filter.reply_to {
            map.insert(Value::String(REPLY_TO.to_string()), Value::String(reply_to));
        }
        if let Some(label) = filter.label {
            map.insert(Value::String(LABEL.to_string()), Value::String(label));
        }
        if let Some(session_id) = filter.session_id {
            map.insert(
                Value::String(SESSION_ID.to_string()),
                Value::String(session_id),
            );
        }
        if let Some(reply_to_session_id) = filter.reply_to_session_id {
            map.insert(
                Value::String(REPLY_TO_SESSION_ID.to_string()),
                Value::String(reply_to_session_id),
            );
        }
        if let Some(content_type) = filter.content_type {
            map.insert(
                Value::String(CONTENT_TYPE.to_string()),
                Value::String(content_type),
            );
        }
        if let Some(properties) = filter.properties {
            map.insert(
                Value::String(CORRELATION_RULE_FILTER_PROPERTIES.to_string()),
                properties.into(),
            );
        }
        if map.is_empty() {
            Err(CorrelationFilterError::EmptyFilter)
        } else {
            Ok(map)
        }
    }
}

pub enum Filter {
    Sql(SqlFilter),
    Correlation(CorrelationFilter),
}

impl From<SqlFilter> for Filter {
    fn from(sql_filter: SqlFilter) -> Self {
        Filter::Sql(sql_filter)
    }
}

impl From<CorrelationFilter> for Filter {
    fn from(correlation_filter: CorrelationFilter) -> Self {
        Filter::Correlation(correlation_filter)
    }
}

type RuleDescription = OrderedMap<String, OrderedMap<String, String>>;

type AddRuleRequestBody = OrderedMap<String, Value>;

pub(crate) struct AddRuleRequest {
    server_timeout: Option<u32>,
    body: AddRuleRequestBody,
}

impl AddRuleRequest {
    pub fn new(
        rule_name: String,
        filter: impl Into<Filter>,
        sql_rule_action: String,
    ) -> Result<Self, CorrelationFilterError> {
        let filter = filter.into();
        let mut rule_description: OrderedMap<Value, Value> = OrderedMap::new();
        match filter {
            Filter::Sql(sql_filter) => {
                let mut sql_filter_map: OrderedMap<Value, Value> = OrderedMap::new();
                sql_filter_map.insert(EXPRESSION.into(), sql_filter.expression.into());
                rule_description.insert(SQL_RULE_FILTER.into(), sql_filter_map.into());
            }
            Filter::Correlation(correlation_filter) => {
                let correlation_filter = OrderedMap::try_from(correlation_filter)?;
                rule_description.insert(CORRELATION_RULE_FILTER.into(), correlation_filter.into());
            }
        }

        rule_description.insert(SQL_RULE_ACTION.into(), sql_rule_action.into());

        let mut body = OrderedMap::new();
        body.insert(RULE_NAME.into(), rule_name.into());
        body.insert(RULE_DESCRIPTION.into(), rule_description.into());

        Ok(Self {
            server_timeout: None,
            body,
        })
    }
}

impl Request for AddRuleRequest {
    const OPERATION: &'static str = ADD_RULE_OPERATION;

    type Response = AddRuleResponse;

    type Body = AddRuleRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a AddRuleRequest {
    const OPERATION: &'static str = ADD_RULE_OPERATION;

    type Response = AddRuleResponse;

    type Body = &'a AddRuleRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a mut AddRuleRequest {
    const OPERATION: &'static str = ADD_RULE_OPERATION;

    type Response = AddRuleResponse;

    type Body = &'a AddRuleRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
