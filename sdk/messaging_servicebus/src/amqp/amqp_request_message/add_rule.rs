use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::add_rule::AddRuleResponse,
    error::CorrelationFilterError,
    filter::{CorrelationFilter, SqlFilter},
    management_constants::{
        operations::ADD_RULE_OPERATION,
        properties::{
            CORRELATION_RULE_FILTER, EXPRESSION, RULE_DESCRIPTION, RULE_NAME, SQL_RULE_ACTION,
            SQL_RULE_FILTER,
        },
    },
};

// type SqlFilter = OrderedMap<String, String>;
// type CorrelationFilter = OrderedMap<String, String>;
// type SqlRuleAction = OrderedMap<String, String>;

pub enum RuleFilter {
    Sql(SqlFilter),
    Correlation(CorrelationFilter),
}

impl From<SqlFilter> for RuleFilter {
    fn from(sql_filter: SqlFilter) -> Self {
        RuleFilter::Sql(sql_filter)
    }
}

impl From<CorrelationFilter> for RuleFilter {
    fn from(correlation_filter: CorrelationFilter) -> Self {
        RuleFilter::Correlation(correlation_filter)
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
        filter: impl Into<RuleFilter>,
        sql_rule_action: String,
    ) -> Result<Self, CorrelationFilterError> {
        let filter = filter.into();
        let mut rule_description: OrderedMap<Value, Value> = OrderedMap::new();
        match filter {
            RuleFilter::Sql(sql_filter) => {
                let mut sql_filter_map: OrderedMap<Value, Value> = OrderedMap::new();
                sql_filter_map.insert(EXPRESSION.into(), sql_filter.expression.into());
                rule_description.insert(SQL_RULE_FILTER.into(), sql_filter_map.into());
            }
            RuleFilter::Correlation(correlation_filter) => {
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
