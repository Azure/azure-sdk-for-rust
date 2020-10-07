use crate::responses::RefreshTokenResponse;
use azure_sdk_core::errors::AzureError;
use log::debug;
use oauth2::{AccessToken, ClientId, ClientSecret};
use std::convert::TryInto;
use url::form_urlencoded;

pub async fn exchange_refresh_token(
    client: &reqwest::Client,
    tenant_id: &str,
    client_id: &ClientId,
    client_secret: Option<&ClientSecret>,
    refresh_token: &AccessToken,
) -> Result<RefreshTokenResponse, AzureError> {
    let mut encoded = form_urlencoded::Serializer::new(String::new());
    let encoded = encoded.append_pair("grant_type", "refresh_token");
    let encoded = encoded.append_pair("client_id", client_id.as_str());
    // optionally add the client secret
    let encoded = if let Some(client_secret) = client_secret {
        encoded.append_pair("client_secret", client_secret.secret())
    } else {
        encoded
    };
    let encoded = encoded.append_pair("refresh_token", refresh_token.secret());
    let encoded = encoded.finish();

    debug!("encoded ==> {}", encoded);

    let url = url::Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    ))?;

    let ret = client
        .post(url)
        .header("ContentType", "application/x-www-form-urlencoded")
        .body(encoded)
        .send()
        .await
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))?
        .text()
        .await
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))?;
    debug!("{}", ret);

    Ok(ret.try_into()?)
}
