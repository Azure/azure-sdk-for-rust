use azure_core::builders::ClientOptionsBuilder;
use azure_data_cosmos::{builders::CosmosClientOptionsBuilder, CosmosClient, CosmosClientOptions};

pub fn create_cosmos_client(
    tx_context: azure_testing::TestContext,
    options: Option<CosmosClientOptions>,
) -> azure_core::Result<CosmosClient> {
    let endpoint = azure_testing::test_endpoint_url();
    let builder = if let Some(options) = options {
        CosmosClientOptionsBuilder::from(options)
    } else {
        CosmosClientOptionsBuilder::default()
    };
    let options = builder
        .with_transport(tx_context.create_transport())
        .build();
    let credentials = tx_context
        .create_credentials(|| Some(azure_identity::create_default_credential().unwrap()));

    CosmosClient::new(endpoint, credentials, Some(options))
}
