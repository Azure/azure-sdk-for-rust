use crate::clients::{
    ChinaCosmosUri, ClientBuilder, CustomCosmosUri, DefaultCosmosUri, HTTPClientNotAssigned,
};
use crate::{AuthorizationToken, CosmosError};
use std::borrow::Cow;
use std::marker::PhantomData;

pub fn build_default_client<'a, IntoCowStr>(
    account: IntoCowStr,
    auth_token: AuthorizationToken,
) -> Result<ClientBuilder<'a, DefaultCosmosUri, HTTPClientNotAssigned>, CosmosError>
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

pub fn build_china_client<'a, IntoCowStr>(
    account: IntoCowStr,
    auth_token: AuthorizationToken,
) -> Result<ClientBuilder<'a, ChinaCosmosUri, HTTPClientNotAssigned>, CosmosError>
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

pub fn build_custom_client<'a, IntoCowStr>(
    account: IntoCowStr,
    auth_token: AuthorizationToken,
    uri: String,
) -> Result<ClientBuilder<'a, CustomCosmosUri, HTTPClientNotAssigned>, CosmosError>
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

pub fn build_emulator_client(
    address: &str,
    port: u16,
) -> Result<ClientBuilder<CustomCosmosUri, HTTPClientNotAssigned>, CosmosError> {
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
