use azservicebus::administration::{
    CorrelationRuleFilter, FalseRuleFilter, SqlRuleAction, SqlRuleFilter, TrueRuleFilter,
};
use azservicebus::ServiceBusClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let topic_name = std::env::var("SERVICE_BUS_TOPIC")?;
    let subscription_name = std::env::var("SERVICE_BUS_SUBSCRIPTION")?;

    let mut client =
        ServiceBusClient::new_from_connection_string(connection_string, Default::default()).await?;
    let mut rule_manager = client
        .create_rule_manager(topic_name, subscription_name)
        .await?;

    let rules = rule_manager.get_rules().await?;

    // Remove all existing rules
    for rule in rules {
        let name = rule.name;
        rule_manager.delete_rule(name).await?;
    }

    // Add a correlation rule filter
    let correlation_filter = CorrelationRuleFilter::builder().subject("subject").build();
    rule_manager
        .create_rule("brand-filter", correlation_filter)
        .await?;

    // Add a SQL rule filter
    let filter = SqlRuleFilter::new("user.color='red'");
    let action = SqlRuleAction::new("SET quantity = quantity / 2;");
    rule_manager
        .create_rule("color-filter", (filter, action))
        .await?;

    // Add a true filter
    rule_manager
        .create_rule("true-filter", TrueRuleFilter::new())
        .await?;

    // Add a false filter
    rule_manager
        .create_rule("false-filter", FalseRuleFilter::new())
        .await?;

    rule_manager.dispose().await?;
    client.dispose().await?;

    Ok(())
}
