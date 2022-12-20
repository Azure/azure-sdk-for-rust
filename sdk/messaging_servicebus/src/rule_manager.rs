use crate::{
    administration::RuleProperties, amqp::amqp_request_message::add_rule::SupportedRuleFilter,
    core::TransportRuleManager,
};

#[derive(Debug)]
pub struct ServiceBusRuleManager<T> {
    pub(crate) inner: T,
}

impl<T> ServiceBusRuleManager<T>
where
    T: TransportRuleManager,
{
    const MAX_RULES_PER_REQUEST: i32 = 100;

    pub fn identifier(&self) -> &str {
        self.inner.identifier()
    }

    pub fn subscription_path(&self) -> &str {
        self.inner.subscription_path()
    }

    pub async fn dispose(self) -> Result<(), T::CloseError> {
        self.inner.close().await
    }

    pub async fn create_rule(
        &mut self,
        name: impl Into<String>,
        filter: impl Into<SupportedRuleFilter>,
        sql_rule_action: impl Into<Option<String>>,
    ) -> Result<(), T::CreateRuleError> {
        self.inner
            .create_rule(name.into(), filter.into(), sql_rule_action.into())
            .await
    }

    pub async fn delete_rule(
        &mut self,
        rule_name: impl Into<String>,
    ) -> Result<(), T::RequestResponseError> {
        self.inner.delete_rule(rule_name.into()).await
    }

    pub async fn get_rules(&mut self) -> Result<Vec<RuleProperties>, T::RequestResponseError> {
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
