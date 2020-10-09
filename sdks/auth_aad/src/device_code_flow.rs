use crate::device_code_responses::*;
use async_timer::timer::new_timer;
use azure_sdk_core::errors::AzureError;
use futures::stream::unfold;
use log::debug;
pub use oauth2::{ClientId, ClientSecret};
use std::borrow::Cow;
use std::convert::TryInto;
use std::time::Duration;
use url::form_urlencoded;

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

pub async fn begin_authorize_device_code_flow<'a, 'b, T>(
    client: &'a reqwest::Client,
    tenant_id: T,
    client_id: &'a ClientId,
    scopes: &'b [&'b str],
) -> Result<DeviceCodePhaseOneResponse<'a>, AzureError>
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
        .await
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))?
        .text()
        .await
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))
        .and_then(|s| {
            serde_json::from_str::<DeviceCodePhaseOneResponse>(&s)
                // we need to capture some variables that will be useful in
                // the second phase (the client, the tenant_id and the client_id)
                .map(|device_code_reponse| DeviceCodePhaseOneResponse {
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
                .map_err(|e| {
                    serde_json::from_str::<crate::errors::ErrorResponse>(&s)
                        .map(|er| AzureError::GenericErrorWithText(er.to_string()))
                        .unwrap_or_else(|_| {
                            AzureError::GenericErrorWithText(format!(
                                "Failed to parse Azure response: {}",
                                e.to_string()
                            ))
                        })
                })
        })
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

        unfold(
            NextState::Continue,
            async move |state: NextState| match state {
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
            },
        )
    }
}
