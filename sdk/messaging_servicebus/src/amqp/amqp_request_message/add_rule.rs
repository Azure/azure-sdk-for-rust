use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::Value;

use crate::{
    administration::{FalseRuleFilter, SqlRuleAction, TrueRuleFilter},
    amqp::{
        amqp_response_message::add_rule::AddRuleResponse,
        error::CorrelationFilterError,
        management_constants::{
            operations::ADD_RULE_OPERATION,
            properties::{
                CORRELATION_RULE_FILTER, EXPRESSION, RULE_DESCRIPTION, RULE_NAME, SQL_RULE_ACTION,
                SQL_RULE_FILTER,
            },
        },
    },
};

use crate::administration::{CorrelationRuleFilter, SqlRuleFilter};

#[derive(Debug, Clone)]
pub enum CreateRuleFilter {
    Sql {
        filter: SqlRuleFilter,
        action: Option<SqlRuleAction>,
    },
    Correlation(CorrelationRuleFilter),
    True(TrueRuleFilter),
    False(FalseRuleFilter),
}

impl From<SqlRuleFilter> for CreateRuleFilter {
    fn from(sql_filter: SqlRuleFilter) -> Self {
        CreateRuleFilter::Sql {
            filter: sql_filter,
            action: None,
        }
    }
}

impl From<(SqlRuleFilter, SqlRuleAction)> for CreateRuleFilter {
    fn from((sql_filter, sql_action): (SqlRuleFilter, SqlRuleAction)) -> Self {
        CreateRuleFilter::Sql {
            filter: sql_filter,
            action: Some(sql_action),
        }
    }
}

impl From<CorrelationRuleFilter> for CreateRuleFilter {
    fn from(correlation_filter: CorrelationRuleFilter) -> Self {
        CreateRuleFilter::Correlation(correlation_filter)
    }
}

impl From<TrueRuleFilter> for CreateRuleFilter {
    fn from(true_filter: TrueRuleFilter) -> Self {
        CreateRuleFilter::True(true_filter)
    }
}

impl From<FalseRuleFilter> for CreateRuleFilter {
    fn from(false_filter: FalseRuleFilter) -> Self {
        CreateRuleFilter::False(false_filter)
    }
}

type AddRuleRequestBody = OrderedMap<String, Value>;

pub(crate) struct AddRuleRequest {
    server_timeout: Option<u32>,
    associated_link_name: Option<String>,
    body: AddRuleRequestBody,
}

impl AddRuleRequest {
    pub(crate) fn new(
        rule_name: String,
        filter: CreateRuleFilter,
        associated_link_name: Option<String>,
    ) -> Result<Self, CorrelationFilterError> {
        let mut rule_description: OrderedMap<Value, Value> = OrderedMap::new();
        let mut rule_action_map = OrderedMap::new();
        match filter {
            CreateRuleFilter::Sql { filter, action } => {
                let mut sql_filter_map: OrderedMap<Value, Value> = OrderedMap::new();
                sql_filter_map.insert(EXPRESSION.into(), filter.expression.into());
                rule_description.insert(SQL_RULE_FILTER.into(), sql_filter_map.into());

                if let Some(sql_rule_action) = action {
                    rule_action_map.insert(EXPRESSION.into(), sql_rule_action.expression.into());
                }
            }
            CreateRuleFilter::Correlation(correlation_filter) => {
                let correlation_filter = OrderedMap::try_from(correlation_filter)?;
                rule_description.insert(CORRELATION_RULE_FILTER.into(), correlation_filter.into());
            }
            CreateRuleFilter::True(_) => {
                let mut sql_filter_map: OrderedMap<Value, Value> = OrderedMap::new();
                sql_filter_map.insert(EXPRESSION.into(), Value::String(String::from("1=1")));
                rule_description.insert(SQL_RULE_FILTER.into(), sql_filter_map.into());
            }
            CreateRuleFilter::False(_) => {
                let mut sql_filter_map: OrderedMap<Value, Value> = OrderedMap::new();
                sql_filter_map.insert(EXPRESSION.into(), Value::String(String::from("1=0")));
                rule_description.insert(SQL_RULE_FILTER.into(), sql_filter_map.into());
            }
        }

        rule_description.insert(SQL_RULE_ACTION.into(), Value::Map(rule_action_map));

        let mut body = OrderedMap::new();
        body.insert(RULE_NAME.into(), rule_name.into());
        body.insert(RULE_DESCRIPTION.into(), rule_description.into());

        Ok(Self {
            server_timeout: None,
            associated_link_name,
            body,
        })
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for AddRuleRequest {
    const OPERATION: &'static str = ADD_RULE_OPERATION;

    type Response = AddRuleResponse;

    type Body = AddRuleRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        // TODO: reduce clones?
        super::encode_application_properties(self.server_timeout, self.associated_link_name.clone())
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
        // TODO: reduce clones?
        super::encode_application_properties(self.server_timeout, self.associated_link_name.clone())
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
        // TODO: reduce clones?
        super::encode_application_properties(self.server_timeout, self.associated_link_name.clone())
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
