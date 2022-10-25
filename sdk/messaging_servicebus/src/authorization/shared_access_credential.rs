use std::{sync::Mutex, time::Duration};

use azure_core::auth::{AccessToken, TokenCredential, TokenResponse};
use time::OffsetDateTime;

use super::shared_access_signature::{SasSignatureError, SharedAccessSignature};

#[derive(Debug)]
pub(crate) struct AzureNamedKeyCredential {
    name: String,
    key: String,
}

impl AzureNamedKeyCredential {
    pub fn new(name: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            key: key.into(),
        }
    }

    pub fn update(&mut self, name: impl Into<String>, key: impl Into<String>) {
        self.name = name.into();
        self.key = key.into();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

impl AzureNamedKeyCredential {}

#[derive(Debug)]
pub struct AzureSasCredential(String);

impl AzureSasCredential {
    pub fn new(signature: impl Into<String>) -> Self {
        Self(signature.into())
    }

    pub fn signature(&self) -> &str {
        &self.0
    }

    fn signature_mut(&mut self) -> &mut String {
        &mut self.0
    }

    pub fn update(&mut self, signature: impl Into<String>) {
        self.0 = signature.into();
    }
}

#[derive(Debug)]
pub(crate) struct SharedAccessCredential {
    source_key_credential: Option<AzureNamedKeyCredential>,
    source_sas_credential: Option<AzureSasCredential>,
    shared_access_signature: Mutex<SharedAccessSignature>,
}

impl From<SharedAccessSignature> for SharedAccessCredential {
    fn from(shared_access_signature: SharedAccessSignature) -> Self {
        Self {
            source_key_credential: None,
            source_sas_credential: None,
            shared_access_signature: Mutex::new(shared_access_signature),
        }
    }
}

impl SharedAccessCredential {
    /// <summary>The buffer to apply when considering refreshing; signatures that expire less than this duration will be refreshed.</summary>
    const SIGNATURE_REFRESH_BUFFER: Duration = Duration::from_secs(10 * 60); // 10 mins

    /// <summary>The length of time extend signature validity, if a token was requested.</summary>
    const SIGNATURE_EXTENSION_DURATION: Duration = Duration::from_secs(30 * 60); // 30 mins

    pub fn from_signature(shared_access_signature: SharedAccessSignature) -> Self {
        Self {
            source_key_credential: None,
            source_sas_credential: None,
            shared_access_signature: Mutex::new(shared_access_signature),
        }
    }

    /// <summary>
    ///   Initializes a new instance of the <see cref="SharedAccessCredential" /> class.
    /// </summary>
    ///
    /// <param name="sourceCredential">The <see cref="AzureSasCredential"/> to base signatures on.</param>
    ///
    pub fn try_from_sas_credential(
        source_sas_credential: AzureSasCredential,
    ) -> Result<Self, SasSignatureError> {
        let shared_access_signature =
            SharedAccessSignature::try_from_signature(&source_sas_credential.0)?;

        Ok(Self {
            source_key_credential: None,
            source_sas_credential: Some(source_sas_credential),
            shared_access_signature: Mutex::new(shared_access_signature),
        })
    }

    /// <summary>
    ///   Initializes a new instance of the <see cref="SharedAccessCredential" /> class.
    /// </summary>
    ///
    /// <param name="sourceCredential">The <see cref="AzureNamedKeyCredential"/> to base signatures on.</param>
    /// <param name="signatureResource">The fully-qualified identifier for the resource to which this credential is intended to serve as authorization for.  This is also known as the "token audience" in some contexts.</param>
    ///
    pub fn try_from_named_key_credential(
        source_credential: AzureNamedKeyCredential,
        signature_resource: impl Into<String>,
    ) -> Result<Self, SasSignatureError> {
        let (name, key) = (source_credential.name(), source_credential.key());
        let shared_access_signature =
            SharedAccessSignature::try_from_parts(signature_resource, name, key, None)?;

        Ok(Self {
            source_key_credential: Some(source_credential),
            source_sas_credential: None,
            shared_access_signature: Mutex::new(shared_access_signature),
        })
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SharedAccessCredential {
    /// <summary>
    ///   Retrieves the token that represents the shared access signature credential, for
    ///   use in authorization against a Service Bus entity.
    /// </summary>
    ///
    /// <param name="requestContext">The details of the authentication request.</param>
    /// <param name="cancellationToken">The token used to request cancellation of the operation.</param>
    ///
    /// <returns>The token representing the shared access signature for this credential.</returns>
    ///
    async fn get_token(&self, _resource: &str) -> azure_core::Result<TokenResponse> {
        let mut signature = self.shared_access_signature.lock().map_err(|error| {
            azure_core::Error::new(azure_core::error::ErrorKind::Other, error.to_string())
        })?;

        // If the signature was derived from a precomputed shared access signature,
        // it should not be extended.  Bypass expiration checks and generate the
        // token.
        if signature.shared_access_key().is_empty() {
            // Before forming the token, regenerate the signature if the source
            // credential has been updated.
            match &self.source_sas_credential {
                Some(sas_credential) if sas_credential.signature() != signature.value() => {
                    *signature =
                        SharedAccessSignature::try_from_signature(sas_credential.signature())?;
                }
                _ => {}
            }

            return Ok(TokenResponse {
                token: AccessToken::new(signature.value().to_string()),
                expires_on: signature.signature_expiration().clone(),
            });
        }

        // If the signature was derived from a shared key that has been updated, regenerate
        // the signature.
        if let Some(key_credential) = &self.source_key_credential {
            let (name, key) = (key_credential.name(), key_credential.key());

            if signature.shared_access_key_name() != name || signature.shared_access_key() != key {
                *signature =
                    SharedAccessSignature::try_from_parts(signature.resource(), name, key, None)?;
            }
        }

        // If the key-based signature is approaching expiration, extend it.
        if *signature.signature_expiration()
            <= OffsetDateTime::now_utc() + Self::SIGNATURE_REFRESH_BUFFER
        {
            // Modify in-place to avoid unnecessary clone
            signature.update_with_new_expiration(Self::SIGNATURE_EXTENSION_DURATION)?;
        }

        Ok(TokenResponse {
            token: AccessToken::new(signature.value().to_string()),
            expires_on: signature.signature_expiration().clone(),
        })
    }
}
