use std::time::Duration;

use azure_core::auth::{TokenCredential, TokenResponse};

use super::shared_access_signature::{self, SharedAccessSignature};

pub(crate) struct AzureNamedKeyCredential(pub (String, String));

pub(crate) struct AzureSasCredential(pub String);

pub(crate) struct SharedAccessCredential {
    source_key_credential: Option<AzureNamedKeyCredential>,
    source_sas_credential: Option<AzureSasCredential>,
    shared_access_signature: SharedAccessSignature,
}

impl From<SharedAccessSignature> for SharedAccessCredential {
    fn from(shared_access_signature: SharedAccessSignature) -> Self {
        Self {
            source_key_credential: None,
            source_sas_credential: None,
            shared_access_signature,
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
            shared_access_signature,
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
    ) -> Result<Self, shared_access_signature::Error> {
        let shared_access_signature =
            SharedAccessSignature::try_from_signature(&source_sas_credential.0)?;

        Ok(Self {
            source_key_credential: None,
            source_sas_credential: Some(source_sas_credential),
            shared_access_signature,
        })
    }

    /// <summary>
    ///   Initializes a new instance of the <see cref="SharedAccessCredential" /> class.
    /// </summary>
    ///
    /// <param name="sourceCredential">The <see cref="AzureNamedKeyCredential"/> to base signatures on.</param>
    /// <param name="signatureResource">The fully-qualified identifier for the resource to which this credential is intended to serve as authorization for.  This is also known as the "token audience" in some contexts.</param>
    ///
    pub fn try_from_key_credential(
        source_credential: AzureNamedKeyCredential,
        signature_resource: impl Into<String>,
    ) -> Result<Self, shared_access_signature::Error> {
        let (name, key) = (&source_credential.0 .0, &source_credential.0 .1);
        let shared_access_signature =
            SharedAccessSignature::try_from_parts(signature_resource, name, key, None)?;

        Ok(Self {
            source_key_credential: Some(source_credential),
            source_sas_credential: None,
            shared_access_signature,
        })
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SharedAccessCredential {
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        todo!()
    }
}
