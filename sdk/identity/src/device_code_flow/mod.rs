//! Authorize using the device authorization grant flow
//!
//! This flow allows users to sign in to input-constrained devices such as a smart TV, IoT device, or printer.
//!
//! You can learn more about this authorization flow [here](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-device-code).
mod device_code_responses;

use azure_core::error::{ErrorKind, Result, ResultExt};
pub use device_code_responses::*;

use async_timer::timer::new_timer;
use futures::stream::unfold;
use oauth2::ClientId;
use serde::Deserialize;
use url::form_urlencoded;

use std::borrow::Cow;
use std::convert::TryInto;
use std::time::Duration;

/// Start the device authorization grant flow.
/// The user has only 15 minutes to sign in (the usual value for expires_in).
pub async fn start<'a, 'b, T>(
    client: &'a reqwest::Client,
    tenant_id: T,
    client_id: &'a ClientId,
    scopes: &'b [&'b str],
) -> Result<DeviceCodePhaseOneResponse<'a>>
where
    T: Into<Cow<'a, str>>,
{
    let mut encoded = form_urlencoded::Serializer::new(String::new());
    let encoded = encoded.append_pair("client_id", client_id.as_str());
    let encoded = encoded.append_pair("scope", &scopes.join(" "));
    let encoded = encoded.finish();

    let tenant_id = tenant_id.into();

    let url = url::Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/devicecode",
        tenant_id
    ))
    .with_context(ErrorKind::Credential, || {
        format!("the supplied tenant id could not be url encoded: {tenant_id}")
    })?;

    let response = client
        .post(url)
        .header("ContentType", "application/x-www-form-urlencoded")
        .body(encoded)
        .send()
        .await
        .context(
            ErrorKind::Io,
            "an error occurred when trying to make a request",
        )?;

    let rsp_status = response.status();
    let rsp_body = response.bytes().await.context(
        ErrorKind::Io,
        "an error occurred when trying to make a request",
    )?;
    if !rsp_status.is_success() {
        return Err(
            ErrorKind::http_response_from_body(rsp_status.as_u16(), &rsp_body).into_error(),
        );
    }

    let device_code_response = serde_json::from_slice::<DeviceCodePhaseOneResponse>(&rsp_body)
    .with_context(
        ErrorKind::DataConversion,
        || format!("the http response body could not be turned into a device code response: {rsp_body:?}")
    )?;

    // we need to capture some variables that will be useful in
    // the second phase (the client, the tenant_id and the client_id)
    Ok(DeviceCodePhaseOneResponse {
        device_code: device_code_response.device_code,
        user_code: device_code_response.user_code,
        verification_uri: device_code_response.verification_uri,
        expires_in: device_code_response.expires_in,
        interval: device_code_response.interval,
        message: device_code_response.message,
        client: Some(client),
        tenant_id,
        client_id: client_id.as_str().to_string(),
    })
}

/// Contains the required information to allow a user to sign in.
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodePhaseOneResponse<'a> {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
    message: String,
    // The skipped fields below do not come from the Azure answer.
    // They will be added manually after deserialization
    #[serde(skip)]
    client: Option<&'a reqwest::Client>,
    #[serde(skip)]
    tenant_id: Cow<'a, str>,
    // We store the ClientId as string instead of the original type, because it
    // does not implement Default, and it's in another crate
    #[serde(skip)]
    client_id: String,
}

impl<'a> DeviceCodePhaseOneResponse<'a> {
    /// The message containing human readable instructions for the user.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Polls the token endpoint while the user signs in.
    /// This will continue until either success or error is returned.
    pub fn stream(&self) -> impl futures::Stream<Item = Result<DeviceCodeResponse>> + '_ {
        #[derive(Debug, Clone, PartialEq)]
        enum NextState {
            Continue,
            Finish,
        }

        unfold(NextState::Continue, move |state: NextState| async move {
            match state {
                NextState::Continue => {
                    let uri = format!(
                        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                        self.tenant_id,
                    );

                    // Throttle down as specified by Azure. This could be
                    // smarter: we could calculate the elapsed time since the
                    // last poll and wait only the delta.
                    new_timer(Duration::from_secs(self.interval)).await;

                    let mut encoded = form_urlencoded::Serializer::new(String::new());
                    let encoded = encoded
                        .append_pair("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
                    let encoded = encoded.append_pair("client_id", self.client_id.as_str());
                    let encoded = encoded.append_pair("device_code", &self.device_code);
                    let encoded = encoded.finish();

                    let result = match self
                        .client
                        .unwrap()
                        .post(&uri)
                        .header("ContentType", "application/x-www-form-urlencoded")
                        .body(encoded)
                        .send()
                        .await
                        .context(
                            ErrorKind::Io,
                            "an error occurred when trying to make a request",
                        ) {
                        Ok(result) => result,
                        Err(error) => return Some((Err(error), NextState::Finish)),
                    };

                    let result = match result.text().await.context(
                        ErrorKind::Io,
                        "an error occurred when trying to make a request",
                    ) {
                        Ok(result) => result,
                        Err(error) => return Some((Err(error), NextState::Finish)),
                    };

                    // Here either we get an error response from Azure
                    // or we get a success. A success can be either "Pending" or
                    // "Completed". We finish the loop only on "Completed"
                    match result.try_into() {
                        Ok(device_code_response) => {
                            let next_state = match &device_code_response {
                                DeviceCodeResponse::AuthorizationSucceeded(_) => NextState::Finish,
                                DeviceCodeResponse::AuthorizationPending(_) => NextState::Continue,
                            };

                            Some((Ok(device_code_response), next_state))
                        }
                        Err(error) => Some((Err(error), NextState::Finish)),
                    }
                }
                NextState::Finish => None,
            }
        })
    }
}
