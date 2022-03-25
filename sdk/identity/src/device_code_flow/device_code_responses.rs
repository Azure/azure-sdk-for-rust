use oauth2::AccessToken;

use serde::Deserialize;
use thiserror::Error;

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

/// Errors when performing the device code flow
#[derive(Error, Debug)]
pub enum DeviceCodeError {
    /// The authorization response returned a "declined" response
    #[error("authorization declined: {0}")]
    AuthorizationDeclined(DeviceCodeErrorResponse),
    /// The authorization response returned a "bad verification" response
    #[error("bad verification code: {0}")]
    BadVerificationCode(DeviceCodeErrorResponse),
    /// The authorization response returned a "expired" response
    #[error("expired token: {0}")]
    ExpiredToken(DeviceCodeErrorResponse),
    /// The authorization response returned an unrecognized error
    #[error("unrecognized device code error response error kind: {0}")]
    UnrecognizedResponse(DeviceCodeErrorResponse),
    /// The supplied tenant id could not be url encoded
    #[error("the supplied tenant id could not be url encoded: {0}")]
    InvalidTenantId(String),
    /// The HTTP response returned an unsuccessful HTTP status code
    #[error("the http response was unsuccessful with status {0}: {}", .1.as_deref().unwrap_or("<NO UTF-8 BODY>"))]
    UnsuccessfulResponse(u16, Option<String>),
    /// The response body could not be turned into a device code response
    #[error("the http response body could not be turned into a device code response: {0}")]
    InvalidResponseBody(String),
    /// An error occurred when trying to make a request
    #[error("an error occurred when trying to make a request")]
    Request(#[source] Box<dyn std::error::Error + Send + Sync>),
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
    type Error = DeviceCodeError;

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
                            "authorization_declined" => Err(
                                DeviceCodeError::AuthorizationDeclined(device_code_error_response),
                            ),

                            "bad_verification_code" => Err(DeviceCodeError::BadVerificationCode(
                                device_code_error_response,
                            )),
                            "expired_token" => {
                                Err(DeviceCodeError::ExpiredToken(device_code_error_response))
                            }
                            _ => Err(DeviceCodeError::UnrecognizedResponse(
                                device_code_error_response,
                            )),
                        }
                    }
                    // If we cannot, we bail out giving the full error as string
                    Err(_) => Err(DeviceCodeError::InvalidResponseBody(self)),
                }
            }
        }
    }
}
