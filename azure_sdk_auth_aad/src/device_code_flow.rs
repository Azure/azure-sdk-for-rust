use async_timer::timer::new_timer;
use azure_sdk_core::errors::AzureError;
use futures::stream::unfold;
use log::debug;
pub use oauth2::{ClientId, ClientSecret};
use std::sync::Arc;
use std::time::Duration;
use url::form_urlencoded;

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodeResponse<'a> {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
    message: String,
    #[serde(skip)]
    client: Arc<reqwest::Client>,
    #[serde(skip)]
    tenant_id: &'a str,
    // we store the ClientId as string instead of
    // the original type because it does not
    // implement Default and it's in another
    // create
    #[serde(skip)]
    client_id: String,
}

pub async fn begin_authorize_device_code_flow<'a, 'b>(
    client: Arc<reqwest::Client>,
    tenant_id: &'a str,
    client_id: &'a ClientId,
    scopes: &'b [&'b str],
) -> Result<DeviceCodeResponse<'a>, AzureError> {
    let mut encoded = form_urlencoded::Serializer::new(String::new());
    let encoded = encoded.append_pair("client_id", client_id.as_str());
    let encoded = encoded.append_pair("scope", &scopes.join(" "));
    let encoded = encoded.finish();

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
            serde_json::from_str::<DeviceCodeResponse>(&s)
                // we need to capture some variables that will be useful in
                // the second phase (the client, the tenant_id and the client_id)
                .map(|device_code_reponse| DeviceCodeResponse {
                    device_code: device_code_reponse.device_code,
                    user_code: device_code_reponse.user_code,
                    verification_uri: device_code_reponse.verification_uri,
                    expires_in: device_code_reponse.expires_in,
                    interval: device_code_reponse.interval,
                    message: device_code_reponse.message,
                    client: client.clone(),
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

impl<'a> DeviceCodeResponse<'a> {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn stream(&self) -> impl futures::Stream<Item = u32> + '_ {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            PollNumber(u32),
        }

        unfold(
            States::PollNumber(3),
            async move |state: States| match state {
                States::PollNumber(poll_number) => {
                    if poll_number < 5 {
                        println!("getting {}", &self.verification_uri);
                        new_timer(Duration::from_secs(2)).await;

                        let mut encoded = form_urlencoded::Serializer::new(String::new());
                        let encoded = encoded.append_pair(
                            "grant_type",
                            "urn:ietf:params:oauth:grant-type:device_code",
                        );
                        let encoded = encoded.append_pair("client_id", self.client_id.as_str());
                        let encoded = encoded.append_pair("device_code", &self.device_code);
                        let encoded = encoded.finish();

                        let result = self
                            .client
                            .post(&format!(
                                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                                self.tenant_id,
                            ))
                            .header("ContentType", "application/x-www-form-urlencoded")
                            .body(encoded)
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();
                        println!("result ==> {}", result);
                        Some((poll_number, States::PollNumber(poll_number + 1)))
                    } else {
                        None
                    }
                }
                _ => panic!(),
            },
        )
    }
}
