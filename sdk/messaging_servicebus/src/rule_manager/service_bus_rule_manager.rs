//! Rule manager for Service Bus subscriptions.



use crate::{
    administration::RuleProperties,
    amqp::{
        amqp_request_message::add_rule::CreateRuleFilter,
        amqp_rule_manager::AmqpRuleManager,
    },
    core::TransportRuleManager, util::IntoAzureCoreError,
};

/// A `ServiceBusRuleManager` is used to manage rules for a subscription.
#[derive(Debug)]
pub struct ServiceBusRuleManager {
    pub(crate) inner: AmqpRuleManager,
}

impl ServiceBusRuleManager {
    const MAX_RULES_PER_REQUEST: i32 = 100;

    /// Get the ID to identify this client.
    pub fn identifier(&self) -> &str {
        self.inner.identifier()
    }

    /// The path of the Service Bus subscription that the rule manager is connected to, specific to the
    /// Service Bus namespace that contains it.
    pub fn subscription_path(&self) -> &str {
        self.inner.subscription_path()
    }

    /// Closes the rule manager and perform any cleanup required.
    pub async fn dispose(self) -> Result<(), azure_core::Error> {
        self.inner.close().await.map_err(IntoAzureCoreError::into_azure_core_error)
    }

    /// Add a rule to the current subscription to filter the messages reaching from topic to the
    /// subscription.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use azservicebus::administration::{
    ///     SqlRuleFilter, SqlRuleAction, CorrelationRuleFilter, TrueRuleFilter,
    /// };
    ///
    /// // Create a correlation rule filter
    /// let filter = CorrelationRuleFilter::builder()
    ///     .subject("test")
    ///     .build();
    /// rule_manager.create_rule("correlation_rule", filter).await.unwrap();
    ///
    /// // Create a SQL rule filter without an action
    /// let filter = SqlRuleFilter::new("user.color='green'");
    /// rule_manager.create_rule("sql_rule", filter).await.unwrap();
    ///
    /// // Create a SQL rule filter with an action
    /// let filter = SqlRuleFilter::new("user.color='red'");
    /// let action = SqlRuleAction::new("SET quantity = quantity / 2;");
    /// rule_manager.create_rule("sql_rule_with_action", (filter, action)).await.unwrap();
    ///
    /// // Create a True rule filter
    /// rule_manager.create_rule("true_rule", TrueRuleFilter).await.unwrap();
    /// ```
    pub async fn create_rule(
        &mut self,
        name: impl Into<String>,
        filter: impl Into<CreateRuleFilter>,
    ) -> Result<(), azure_core::Error> {
        self.inner.create_rule(name.into(), filter.into()).await.map_err(Into::into)
    }

    /// Remove a rule from the current subscription.
    pub async fn delete_rule(
        &mut self,
        rule_name: impl Into<String>,
    ) -> Result<(), azure_core::Error> {
        self.inner.delete_rule(rule_name.into()).await.map_err(Into::into)
    }

    /// Get the rules associated with the current subscription.
    pub async fn get_rules(
        &mut self,
    ) -> Result<Vec<RuleProperties>, azure_core::Error> {
        let mut skip = 0;
        let mut buffer = Vec::new();
        loop {
            let rule_descriptions = self
                .inner
                .get_rules(skip, Self::MAX_RULES_PER_REQUEST)
                .await?;
            let len = rule_descriptions.len();
            skip += len as i32;

            buffer.extend(rule_descriptions);
            if len < Self::MAX_RULES_PER_REQUEST as usize {
                break;
            }
        }

        Ok(buffer)
    }
}
