use azure_core::error::{Error, ErrorKind};
use oauth2::AccessToken;
use serde::Deserialize;
use std::convert::TryInto;
use std::fmt;

/// Error response returned from the device code flow.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DeviceCodeErrorResponse {
    /// Name of the error.
    pub error: String,
    /// Description of the error.
    pub error_description: String,
    /// Uri to get more information on this error.
    pub error_uri: String,
}

impl fmt::Display for DeviceCodeErrorResponse {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. {}", self.error, self.error_description)
    }
}

/// A successful token response.
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodeAuthorization {
    /// Always `Bearer`.
    pub token_type: String,
    /// The scopes the access token is valid for.
    /// Format: Space separated strings
    pub scope: String,
    /// Number of seconds the included access token is valid for.
    pub expires_in: u64,
    /// Issued for the scopes that were requested.
    /// Format: Opaque string
    access_token: AccessToken,
    /// Issued if the original scope parameter included offline_access.
    /// Format: JWT
    refresh_token: Option<AccessToken>,
    /// Issued if the original scope parameter included the openid scope.
    /// Format: Opaque string
    id_token: Option<AccessToken>,
}

impl DeviceCodeAuthorization {
    /// Get the access token
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }
    /// Get the refresh token
    pub fn refresh_token(&self) -> Option<&AccessToken> {
        self.refresh_token.as_ref()
    }
    /// Get the id token
    pub fn id_token(&self) -> Option<&AccessToken> {
        self.id_token.as_ref()
    }
}

/// Expected responses while polling the /token endpoint.
#[derive(Debug, Clone)]
pub enum DeviceCodeResponse {
    /// A successful authentication (token) response.
    AuthorizationSucceeded(DeviceCodeAuthorization),
    /// The user hasn't finished authenticating, but hasn't canceled the flow.
    AuthorizationPending(DeviceCodeErrorResponse),
}

impl TryInto<DeviceCodeResponse> for String {
    type Error = Error;

    fn try_into(self) -> Result<DeviceCodeResponse, Self::Error> {
        // first we try to deserialize as DeviceCodeAuthorization (success)
        match serde_json::from_str::<DeviceCodeAuthorization>(&self) {
            Ok(device_code_authorization) => Ok(DeviceCodeResponse::AuthorizationSucceeded(
                device_code_authorization,
            )),
            Err(_) => {
                // now we try to map it to a DeviceCodeErrorResponse
                match serde_json::from_str::<DeviceCodeErrorResponse>(&self) {
                    Ok(device_code_error_response) => {
                        match &device_code_error_response.error as &str {
                            "authorization_pending" => {
                                Ok(DeviceCodeResponse::AuthorizationPending(
                                    device_code_error_response,
                                ))
                            }
                            "authorization_declined" => {
                                Err(Error::with_message(ErrorKind::Credential,
                                    format!("authorization declined: {device_code_error_response}")
                                ))
                            }
                            "bad_verification_code" => {
                                Err(Error::with_message(ErrorKind::Credential,
                                    format!("bad verification code: {device_code_error_response}")
                                ))
                            }
                            "expired_token" => {
                                Err(Error::with_message(ErrorKind::Credential,
                                    format!("expired token: {device_code_error_response}")
                                ))
                            }
                            _ => Err(Error::with_message(ErrorKind::Credential,
                                format!("unrecognized device code error response error kind: {device_code_error_response}")
                            )),
                        }
                    }
                    // If we cannot, we bail out giving the full error as string
                    Err(_) => Err(Error::with_message(ErrorKind::Credential,
                        format!("the http response body could not be turned into a device code response: {self}")
                    )),
                }
            }
        }
    }
}
