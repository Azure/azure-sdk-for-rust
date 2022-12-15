use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use fe2o3_amqp_management::client::MgmtClient;
use fe2o3_amqp::link::DetachError;
use std::time::Duration as StdDuration;

use crate::{
    administration::RuleProperties, core::TransportRuleManager, ServiceBusRetryPolicy,
    primitives::error::RetryError
};

use super::{error::AmqpRequestResponseError, amqp_request_message::add_rule::AddRuleRequest, amqp_response_message::add_rule::AddRuleResponse};

#[derive(Debug)]
pub struct AmqpRuleManager<RP> {
    pub(crate) management_client: MgmtClient,
    pub(crate) retry_policy: RP,
}

#[async_trait]
impl<RP> TransportRuleManager for AmqpRuleManager<RP>
where
    RP: ServiceBusRetryPolicy + Send,
{
    type RequestResponseError = RetryError<AmqpRequestResponseError>;
    type CloseError = DetachError;

    // /// Indicates whether or not this rule manager has been closed.
    // fn is_closed(&self) -> bool {
    //     todo!()
    // }

    /// Adds a rule to the current subscription to filter the messages reaching from topic to the
    /// subscription.
    ///
    /// # Parameters
    ///
    /// * `properties` - The rule properties for the rule to add.
    /// * `cancellation_token` - An optional [CancellationToken] instance to signal the
    ///   request to cancel the operation.
    ///
    /// # Returns
    ///
    /// A future that represents the asynchronous add rule operation.
    async fn create_rule(
        &mut self,
        _properties: RuleProperties,
    ) -> Result<(), Self::RequestResponseError> {
        todo!()
    }

    /// Removes the rule on the subscription identified by <paramref name="ruleName" />.
    ///
    /// # Parameters
    ///
    /// * `rule_name` - Name of the rule
    /// * `cancellation_token` - An optional <see cref="CancellationToken"/> instance to signal the
    ///   request to cancel the operation.
    ///
    /// # Returns
    ///
    /// A future that represents the asynchronous remove rule operation.
    async fn delete_rule(
        &mut self,
        _rule_name: impl Into<String> + Send,
    ) -> Result<(), Self::RequestResponseError> {
        todo!()
    }

    /// Get all rules associated with the subscription.
    ///
    /// # Parameters
    ///
    /// * `skip` - The number of rules to skip when retrieving the next set of rules.
    /// * `top` - The number of rules to retrieve per service request.
    /// * `cancellation_token` - An optional <see cref="CancellationToken"/> instance to signal the
    ///   request to cancel the operation.
    ///
    /// # Returns
    ///
    /// Returns a list of rules description
    async fn get_rules(
        &mut self,
        _skip: i32,
        _top: i32,
    ) -> Result<Vec<RuleProperties>, Self::RequestResponseError> {
        todo!()
    }

    /// Closes the connection to the transport rule manager instance.
    async fn close(
        mut self,
    ) -> Result<(), Self::CloseError> {
        self.management_client.close().await
    }
}

async fn create_rule<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut AddRuleRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<AddRuleResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}
