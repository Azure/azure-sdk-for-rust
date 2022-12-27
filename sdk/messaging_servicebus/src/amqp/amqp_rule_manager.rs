use async_trait::async_trait;
use fe2o3_amqp::link::DetachError;
use std::{sync::Arc, time::Duration as StdDuration};
use tokio::sync::Mutex;
use url::Url;

use crate::{
    administration::RuleProperties,
    amqp::amqp_request_message::add_rule::CreateRuleFilter,
    core::{RecoverableTransport, TransportRuleManager},
    primitives::{error::RetryError, service_bus_retry_policy::run_operation},
    sealed::Sealed,
    ServiceBusRetryPolicy,
};

use super::{
    amqp_connection_scope::AmqpConnectionScope,
    amqp_management_link::AmqpManagementLink,
    amqp_request_message::{
        add_rule::AddRuleRequest, enumerate_rules::EnumerateRulesRequest,
        remove_rule::RemoveRuleRequest,
    },
    amqp_response_message::{
        add_rule::AddRuleResponse, enumerate_rules::EnumerateRulesResponse,
        remove_rule::RemoveRuleResponse,
    },
    error::{AmqpRequestResponseError, CreateRuleError, OpenRuleManagerError},
};

/// An AMQP implementation for Service Bus rule management.
#[derive(Debug)]
pub struct AmqpRuleManager {
    pub(crate) identifier_str: String,
    pub(crate) service_endpoint: Arc<Url>,
    pub(crate) subscription_path: String,

    pub(crate) management_link: AmqpManagementLink,
    pub(crate) retry_policy: Box<dyn ServiceBusRetryPolicy>,

    /// This is ONLY used for recovery
    pub(crate) connection_scope: Arc<Mutex<AmqpConnectionScope>>,
}

#[async_trait]
impl RecoverableTransport for AmqpRuleManager {
    type RecoverError = OpenRuleManagerError;

    async fn recover(&mut self) -> Result<(), Self::RecoverError> {
        let mut scope = self.connection_scope.lock().await;

        scope
            .recover()
            .await
            // Unable to recover the connection scope
            .map_err(|conn_scope_error| {
                log::error!("Unable to recover connection scope: {:?}", conn_scope_error);
                Self::RecoverError::ConnectionScopeDisposed
            })?;

        self.management_link = scope
            .open_management_link(
                &self.service_endpoint,
                &self.subscription_path,
                &self.identifier_str,
            )
            .await?;
        Ok(())
    }
}

impl AmqpRuleManager {
    async fn create_rule(
        &mut self,
        request: &mut AddRuleRequest,
        try_timeout: &StdDuration,
    ) -> Result<AddRuleResponse, AmqpRequestResponseError> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));

        let response = self.management_link.client_mut().call(request).await?;
        Ok(response)
    }

    async fn delete_rule(
        &mut self,
        request: &mut RemoveRuleRequest,
        try_timeout: &StdDuration,
    ) -> Result<RemoveRuleResponse, AmqpRequestResponseError> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));

        let response = self.management_link.client_mut().call(request).await?;
        Ok(response)
    }

    async fn get_rules(
        &mut self,
        request: &mut EnumerateRulesRequest,
        try_timeout: &StdDuration,
    ) -> Result<EnumerateRulesResponse, AmqpRequestResponseError> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));

        let response = self.management_link.client_mut().call(request).await?;
        Ok(response)
    }
}

impl Sealed for AmqpRuleManager {}

#[async_trait]
impl TransportRuleManager for AmqpRuleManager {
    type CreateRuleError = RetryError<CreateRuleError>;
    type DeleteRuleError = RetryError<AmqpRequestResponseError>;
    type GetRulesError = RetryError<AmqpRequestResponseError>;
    type CloseError = DetachError;

    fn identifier(&self) -> &str {
        &self.identifier_str
    }

    fn subscription_path(&self) -> &str {
        &self.subscription_path
    }

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
        filter: CreateRuleFilter,
    ) -> Result<(), Self::CreateRuleError> {
        let mut request = AddRuleRequest::new(rule_name, filter, None)
            .map_err(CreateRuleError::from)
            .map_err(RetryError::Operation)?;
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        let _response = run_operation!(
            { &self.retry_policy },
            CreateRuleError,
            try_timeout,
            self.create_rule(&mut request, &try_timeout),
            self.recover()
        )?;
        Ok(())
    }

    /// Removes the rule on the subscription identified by <paramref name="ruleName" />.
    async fn delete_rule(&mut self, rule_name: String) -> Result<(), Self::DeleteRuleError> {
        let mut request = RemoveRuleRequest::new(rule_name, None);
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        let _response = run_operation!(
            { &self.retry_policy },
            AmqpRequestResponseError,
            try_timeout,
            self.delete_rule(&mut request, &try_timeout),
            self.recover()
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
    ) -> Result<Vec<RuleProperties>, Self::GetRulesError> {
        let mut request = EnumerateRulesRequest::new(skip, top, None);
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        let response = run_operation!(
            { &self.retry_policy },
            AmqpRequestResponseError,
            try_timeout,
            self.get_rules(&mut request, &try_timeout),
            self.recover()
        )?;
        Ok(response.into_get_rules_response())
    }

    /// Closes the connection to the transport rule manager instance.
    async fn close(mut self) -> Result<(), Self::CloseError> {
        self.management_link.close().await
    }
}
