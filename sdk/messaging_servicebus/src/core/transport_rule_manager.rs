use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::administration::RuleProperties;

#[async_trait]
pub(crate) trait TransportRuleManager {
    type Error: Send;

    /// Indicates whether or not this rule manager has been closed.
    ///
    /// # Return
    ///
    /// `true` if the rule manager is closed; otherwise, `false`.
    fn is_closed(&self) -> bool;

    /// Adds a rule to the current subscription to filter the messages reaching from topic to the
    /// subscription.
    ///
    /// # Arguments
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
        properties: RuleProperties,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<(), Self::Error>;

    /// Removes the rule on the subscription identified by <paramref name="ruleName" />.
    ///
    /// # Arguments
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
        rule_name: impl Into<String>,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<(), Self::Error>;

    /// Get all rules associated with the subscription.
    ///
    /// # Arguments
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
        cancellation_token: Option<CancellationToken>,
    ) -> Result<Vec<RuleProperties>, Self::Error>;

    /// Closes the connection to the transport rule manager instance.
    ///
    /// # Arguments
    ///
    /// * `cancellation_token` - An optional [CancellationToken] instance to signal the request to
    ///   cancel the operation.
    async fn close(
        &mut self,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<(), Self::Error>;
}
