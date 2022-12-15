use async_trait::async_trait;
use fe2o3_amqp::link::DetachError;
use fe2o3_amqp_management::client::MgmtClient;
use std::time::Duration as StdDuration;

use crate::{
    administration::RuleProperties,
    amqp::amqp_request_message::add_rule::SupportedRuleFilter,
    core::TransportRuleManager,
    primitives::{error::RetryError, service_bus_retry_policy::run_operation},
    ServiceBusRetryPolicy,
};

use super::{
    amqp_management_link::AmqpManagementLink,
    amqp_request_message::{
        add_rule::AddRuleRequest, enumerate_rules::EnumerateRulesRequest,
        remove_rule::RemoveRuleRequest,
    },
    amqp_response_message::{
        add_rule::AddRuleResponse, enumerate_rules::EnumerateRulesResponse,
        remove_rule::RemoveRuleResponse,
    },
    error::{AmqpRequestResponseError, CreateRuleError},
};

#[derive(Debug)]
pub struct AmqpRuleManager<RP> {
    pub(crate) management_link: AmqpManagementLink,
    pub(crate) retry_policy: RP,
}

#[async_trait]
impl<RP> TransportRuleManager for AmqpRuleManager<RP>
where
    RP: ServiceBusRetryPolicy + Send,
{
    type CreateRuleError = RetryError<CreateRuleError>;
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
        rule_name: String,
        filter: SupportedRuleFilter,
        sql_rule_action: Option<String>,
    ) -> Result<(), Self::CreateRuleError> {
        let mut request = AddRuleRequest::new(rule_name, filter, sql_rule_action, None)
            .map_err(CreateRuleError::from)
            .map_err(RetryError::Operation)?;
        let mgmt_client = self.management_link.client_mut();
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let _response = run_operation!(
            policy,
            RP,
            CreateRuleError,
            try_timeout,
            create_rule(mgmt_client, &mut request, &try_timeout).await
        )?;
        Ok(())
    }

    /// Removes the rule on the subscription identified by <paramref name="ruleName" />.
    async fn delete_rule(&mut self, rule_name: String) -> Result<(), Self::RequestResponseError> {
        let mut request = RemoveRuleRequest::new(rule_name, None);
        let mgmt_client = self.management_link.client_mut();
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let _response = run_operation!(
            policy,
            RP,
            AmqpRequestResponseError,
            try_timeout,
            delete_rule(mgmt_client, &mut request, &try_timeout).await
        )?;
        Ok(())
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
        skip: i32,
        top: i32,
    ) -> Result<Vec<RuleProperties>, Self::RequestResponseError> {
        let mut request = EnumerateRulesRequest::new(skip, top, None);
        let mgmt_client = self.management_link.client_mut();
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let response = run_operation!(
            policy,
            RP,
            AmqpRequestResponseError,
            try_timeout,
            get_rules(mgmt_client, &mut request, &try_timeout).await
        )?;
        Ok(response.into_get_rules_response())
    }

    /// Closes the connection to the transport rule manager instance.
    async fn close(mut self) -> Result<(), Self::CloseError> {
        self.management_link.close().await
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

async fn delete_rule<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut RemoveRuleRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<RemoveRuleResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}

async fn get_rules<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut EnumerateRulesRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<EnumerateRulesResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}
