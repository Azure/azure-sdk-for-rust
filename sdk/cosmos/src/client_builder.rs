use crate::clients::{ClientBuilder, DefaultCosmosUri, HTTPClientNotAssigned};
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
