use crate::clients::{
    ChinaCosmosUri, ClientBuilder, CustomCosmosUri, DefaultCosmosUri, HTTPClientNotAssigned,
};
use crate::AuthorizationToken;
use azure_core::errors::AzureError;
use std::borrow::Cow;
use std::marker::PhantomData;

pub fn new<'a, IntoCowStr>(
    account: IntoCowStr,
    auth_token: AuthorizationToken,
) -> Result<ClientBuilder<'a, DefaultCosmosUri, HTTPClientNotAssigned>, AzureError>
where
    IntoCowStr: Into<Cow<'a, str>>,
{
    let account = account.into();
    let cosmos_uri_builder = DefaultCosmosUri::new(account.as_ref());

    Ok(ClientBuilder {
        http_client: None,
        p_http_client_to_assign: PhantomData {},
        account: account,
        auth_token,
        cosmos_uri_builder,
    })
}

pub fn new_china<'a, IntoCowStr>(
    account: IntoCowStr,
    auth_token: AuthorizationToken,
) -> Result<ClientBuilder<'a, ChinaCosmosUri, HTTPClientNotAssigned>, AzureError>
where
    IntoCowStr: Into<Cow<'a, str>>,
{
    let account = account.into();
    let cosmos_uri_builder = ChinaCosmosUri::new(account.as_ref());

    Ok(ClientBuilder {
        http_client: None,
        p_http_client_to_assign: PhantomData {},
        account,
        auth_token,
        cosmos_uri_builder,
    })
}

pub fn new_custom<'a, IntoCowStr>(
    account: IntoCowStr,
    auth_token: AuthorizationToken,
    uri: String,
) -> Result<ClientBuilder<'a, CustomCosmosUri, HTTPClientNotAssigned>, AzureError>
where
    IntoCowStr: Into<Cow<'a, str>>,
{
    Ok(ClientBuilder {
        http_client: None,
        p_http_client_to_assign: PhantomData {},
        account: account.into(),
        auth_token,
        cosmos_uri_builder: CustomCosmosUri { uri },
    })
}

pub fn new_emulator(
    address: &str,
    port: u16,
) -> Result<ClientBuilder<CustomCosmosUri, HTTPClientNotAssigned>, AzureError> {
    //Account name: localhost:<port>
    //Account key: C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==
    let auth_token = AuthorizationToken::new_master(
        "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==",
    )
    .unwrap();
    Ok(ClientBuilder {
        http_client: None,
        p_http_client_to_assign: PhantomData {},
        account: Cow::Owned(format!("{}:{}", address, port)),
        auth_token,
        cosmos_uri_builder: CustomCosmosUri {
            uri: format!("https://{}:{}", address, port),
        },
    })
}
