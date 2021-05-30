//! Authorize using the device authorization grant flow
//!
//! This flow allows users to sign in to input-constrained devices such as a smart TV, IoT device, or printer.
//!
//! You can learn more about this authorization flow [here](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code).
mod device_code_responses;
use crate::Error;
use async_timer::timer::new_timer;
pub use device_code_responses::*;
use futures::stream::unfold;
use log::debug;
use oauth2::ClientId;
use serde::Deserialize;
use std::borrow::Cow;
use std::convert::TryInto;
use std::time::Duration;
use url::form_urlencoded;

pub async fn start<'a, 'b, T>(
    client: &'a reqwest::Client,
    tenant_id: T,
    client_id: &'a ClientId,
    scopes: &'b [&'b str],
) -> Result<DeviceCodePhaseOneResponse<'a>, Error>
where
    T: Into<Cow<'a, str>>,
{
    let mut encoded = form_urlencoded::Serializer::new(String::new());
    let encoded = encoded.append_pair("client_id", client_id.as_str());
    let encoded = encoded.append_pair("scope", &scopes.join(" "));
    let encoded = encoded.finish();

    let tenant_id = tenant_id.into();

    debug!("encoded ==> {}", encoded);

    let url = url::Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/devicecode",
        tenant_id
    ))?;

    client
        .post(url)
        .header("ContentType", "application/x-www-form-urlencoded")
        .body(encoded)
        .send()
        .await?
        .text()
        .await
        .map(|s| -> Result<DeviceCodePhaseOneResponse, Error> {
            serde_json::from_str::<DeviceCodePhaseOneResponse>(&s)
                // we need to capture some variables that will be useful in
                // the second phase (the client, the tenant_id and the client_id)
                .map(|device_code_reponse| {
                    Ok(DeviceCodePhaseOneResponse {
                        device_code: device_code_reponse.device_code,
                        user_code: device_code_reponse.user_code,
                        verification_uri: device_code_reponse.verification_uri,
                        expires_in: device_code_reponse.expires_in,
                        interval: device_code_reponse.interval,
                        message: device_code_reponse.message,
                        client: Some(client),
                        tenant_id,
                        client_id: client_id.as_str().to_string(),
                    })
                })?
            // TODO The HTTP status code should be checked to deserialize an error response.
            // serde_json::from_str::<crate::errors::ErrorResponse>(&s).map(Error::ErrorResponse)
        })?
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodePhaseOneResponse<'a> {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
    message: String,
    // the skipped fields below do not come
    // from the Azure answer. They will be added
    // manually after deserialization
    #[serde(skip)]
    client: Option<&'a reqwest::Client>,
    #[serde(skip)]
    tenant_id: Cow<'a, str>,
    // we store the ClientId as string instead of
    // the original type because it does not
    // implement Default and it's in another
    // create
    #[serde(skip)]
    client_id: String,
}

impl<'a> DeviceCodePhaseOneResponse<'a> {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn stream<'b>(
        &'b self,
    ) -> impl futures::Stream<Item = Result<DeviceCodeResponse, DeviceCodeError>> + 'b + '_ {
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

                    // throttle down as specified by Azure. This could be
                    // smarter: we could calculate the elapsed time since the
                    // last poll and wait only the delta. For now we do not
                    // need such precision.
                    new_timer(Duration::from_secs(self.interval)).await;
                    debug!("posting to {}", &uri);

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
                        .map_err(DeviceCodeError::ReqwestError)
                    {
                        Ok(result) => result,
                        Err(error) => return Some((Err(error), NextState::Finish)),
                    };
                    debug!("result (raw) ==> {:?}", result);

                    let result = match result.text().await.map_err(DeviceCodeError::ReqwestError) {
                        Ok(result) => result,
                        Err(error) => return Some((Err(error), NextState::Finish)),
                    };
                    debug!("result (as text) ==> {}", result);

                    // here either we get an error response from Azure
                    // or we get a success. A success can be either "Pending" or
                    // "Completed". We finish the loop only on "Completed" (ie Success)
                    match result.try_into() {
                        Ok(device_code_response) => {
                            let next_state = match &device_code_response {
                                DeviceCodeResponse::AuthorizationSucceded(_) => NextState::Finish,
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
