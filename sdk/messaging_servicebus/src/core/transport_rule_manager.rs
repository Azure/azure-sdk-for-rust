use async_trait::async_trait;

use crate::{
    administration::RuleProperties, amqp::amqp_request_message::add_rule::CreateRuleFilter,
    sealed::Sealed,
};

/// Trait for rule manager implementations.
#[async_trait]
pub(crate) trait TransportRuleManager: Sealed {
    /// Error with creating a rule
    type CreateRuleError: std::error::Error + Send;

    /// Error with deleting a rule
    type DeleteRuleError: std::error::Error + Send;

    /// Error with getting rules
    type GetRulesError: std::error::Error + Send;

    /// Error with closing a rule manager
    type CloseError: std::error::Error + Send;

    /// Gets the identifier of the rule manager.
    fn identifier(&self) -> &str;

    /// Gets the subscription path of the rule manager.
    fn subscription_path(&self) -> &str;

    // /// Indicates whether or not this rule manager has been closed.
    // ///
    // /// # Return
    // ///
    // /// `true` if the rule manager is closed; otherwise, `false`.
    // fn is_closed(&self) -> bool; // TODO: there is currently no good way to detect remote close without polling

    /// Adds a rule to the current subscription to filter the messages reaching from topic to the
    /// subscription.
    async fn create_rule(
        &mut self,
        rule_name: String,
        filter: CreateRuleFilter,
    ) -> Result<(), Self::CreateRuleError>;

    /// Removes the rule on the subscription identified by `rule_name`.
    async fn delete_rule(&mut self, rule_name: String) -> Result<(), Self::DeleteRuleError>;

    /// Get all rules associated with the subscription.
    ///
    /// # Parameters
    ///
    /// * `skip` - The number of rules to skip when retrieving the next set of rules.
    /// * `top` - The number of rules to retrieve per service request.
    async fn get_rules(
        &mut self,
        skip: i32,
        top: i32,
    ) -> Result<Vec<RuleProperties>, Self::GetRulesError>;

    /// Closes the connection to the transport rule manager instance.
    async fn close(mut self) -> Result<(), Self::CloseError>;
}
