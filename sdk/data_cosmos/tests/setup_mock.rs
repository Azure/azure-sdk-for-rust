use azure_data_cosmos::prelude::*;

pub fn initialize(transaction_name: impl Into<String>) -> azure_core::Result<CosmosClient> {
    use azure_core::TransportOptions;

    let account_name = (std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_else(|| String::from("MOCK_ACCOUNT"));
    let authorization_token = get_authorization_token();

    let transport_options = TransportOptions::new_custom_policy(
        mock_transport::new_mock_transport(transaction_name.into()),
    );
    let client = CosmosClient::builder(account_name, authorization_token)
        .transport(transport_options)
        .build();

    Ok(client)
}

fn get_account() -> String {
    std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!")
}

fn get_authorization_token() -> AuthorizationToken {
    (std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD))
    .then(|| {
        let key = std::env::var("COSMOS_PRIMARY_KEY")
            .expect("Set env variable COSMOS_PRIMARY_KEY first!");

        AuthorizationToken::primary_from_base64(&key).ok()
    })
    .flatten()
    .unwrap_or_else(|| AuthorizationToken::new_resource(String::from("MOCK_RESOURCE")))
}
