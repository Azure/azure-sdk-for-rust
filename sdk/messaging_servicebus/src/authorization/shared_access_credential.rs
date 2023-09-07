use std::{sync::Mutex, time::Duration};

use azure_core::auth::{AccessToken, TokenResponse};

use super::{
    azure_named_key_credential::AzureNamedKeyCredential,
    shared_access_signature::{SasSignatureError, SharedAccessSignature},
    AzureSasCredential,
};

/// TODO: visibility?
#[derive(Debug)]
pub struct SharedAccessCredential {
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
    /// The buffer to apply when considering refreshing; signatures that expire less than this duration will be refreshed.
    const SIGNATURE_REFRESH_BUFFER: Duration = Duration::from_secs(10 * 60); // 10 mins

    /// The length of time extend signature validity, if a token was requested.
    const SIGNATURE_EXTENSION_DURATION: Duration = Duration::from_secs(30 * 60); // 30 mins

    pub(crate) fn from_signature(shared_access_signature: SharedAccessSignature) -> Self {
        Self {
            source_key_credential: None,
            source_sas_credential: None,
            shared_access_signature: Mutex::new(shared_access_signature),
        }
    }

    /// Initializes a new instance of the [`SharedAccessCredential`] class.
    ///
    /// - `source_sas_credential` - The [`AzureSasCredential`] to base signatures on.
    pub fn try_from_sas_credential(
        source_sas_credential: AzureSasCredential,
    ) -> Result<Self, SasSignatureError> {
        let shared_access_signature =
            SharedAccessSignature::try_from_signature(source_sas_credential.signature())?;

        Ok(Self {
            source_key_credential: None,
            source_sas_credential: Some(source_sas_credential),
            shared_access_signature: Mutex::new(shared_access_signature),
        })
    }

    /// Initializes a new instance of the [`SharedAccessCredential`] class.
    ///
    /// - `source_credential` - The [`AzureNamedKeyCredential`] to base signatures on.
    /// - `signature_resource` - The fully-qualified identifier for the resource to which this credential is intended to serve as authorization for.  This is also known as the "token audience" in some contexts.
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

impl SharedAccessCredential {
    /// Retrieves the token that represents the shared access signature credential, for
    /// use in authorization against a Service Bus entity.
    ///
    /// FIXME: this is a temporary workaround until specialization is stablized.
    pub(crate) async fn get_token(&self, _resource: &str) -> azure_core::Result<TokenResponse> {
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
                expires_on: *signature.signature_expiration(),
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
            <= crate::util::time::now_utc() + Self::SIGNATURE_REFRESH_BUFFER
        {
            // Modify in-place to avoid unnecessary clone
            signature.update_with_new_expiration(Self::SIGNATURE_EXTENSION_DURATION)?;
        }

        Ok(TokenResponse {
            token: AccessToken::new(signature.value().to_string()),
            expires_on: *signature.signature_expiration(),
        })
    }
}

#[cfg(test)]
mod tests {
    use time::Duration as TimeSpan;

    use crate::authorization::shared_access_signature::SharedAccessSignature;

    use super::{AzureNamedKeyCredential, SharedAccessCredential};

    #[tokio::test]
    async fn get_token_returns_signature_value_with_key_constsructor_initializes_properties() {
        let signature =
            SharedAccessSignature::try_from_parts("hub-name", "keyName", "key", None).unwrap();
        let source_credential = AzureNamedKeyCredential::new(
            signature.shared_access_key_name(),
            signature.shared_access_key(),
        );
        let credential = SharedAccessCredential::try_from_named_key_credential(
            source_credential,
            signature.resource(),
        )
        .unwrap();

        let token = credential.get_token("").await.unwrap();
        assert_eq!(token.token.secret(), signature.value());
    }

    #[tokio::test]
    async fn get_token_returns_signature_value() {
        let signature =
            SharedAccessSignature::try_from_parts("hub-name", "keyName", "key", None).unwrap();
        let credential = SharedAccessCredential::from_signature(signature.clone());

        let token = credential.get_token("").await.unwrap();
        assert_eq!(token.token.secret(), signature.value());
    }

    #[tokio::test]
    async fn get_token_extends_an_expired_token_when_created_with_shared_key() {
        let expires_on = crate::util::time::now_utc() - TimeSpan::hours(2);
        let signature =
            SharedAccessSignature::try_new("hub-name", "keyName", "key", expires_on).unwrap();
        let credential = SharedAccessCredential::from_signature(signature);

        let expected_expiration =
            crate::util::time::now_utc() + SharedAccessCredential::SIGNATURE_EXTENSION_DURATION;
        let token = credential.get_token("").await.unwrap();

        // There will be a small time difference between the two calls to `now_utc()`
        assert!(token.expires_on - expected_expiration < TimeSpan::seconds(1));
    }

    #[tokio::test]
    async fn get_token_extends_a_token_close_to_expiring_when_created_with_shared_key() {
        let expires_on =
            crate::util::time::now_utc() + SharedAccessCredential::SIGNATURE_REFRESH_BUFFER / 2;
        let signature =
            SharedAccessSignature::try_new("hub-name", "keyName", "key", expires_on).unwrap();
        let credential = SharedAccessCredential::from_signature(signature);

        let expected_expiration =
            crate::util::time::now_utc() + SharedAccessCredential::SIGNATURE_EXTENSION_DURATION;
        let token = credential.get_token("").await.unwrap();
        assert!(token.expires_on - expected_expiration < TimeSpan::seconds(1));
    }

    #[tokio::test]
    async fn get_token_does_not_extend_an_expired_token_when_created_without_the_key() {
        let expires_on = crate::util::time::now_utc() - TimeSpan::hours(2);
        let value = format!("SharedAccessSignature sr=https%3A%2F%2Ffake-test.servicebus.windows.net%2F&sig=nNBNavJfBiHuXUzWOLhSvI3bVgqbQUzA7Po8%2F4wQQng%3D&se={}&skn=fakeKey", expires_on.unix_timestamp());
        let source_signature = SharedAccessSignature::try_from_signature(&value).unwrap();
        let signature =
            SharedAccessSignature::try_from_signature(source_signature.value()).unwrap();
        let credential = SharedAccessCredential::from_signature(signature);

        let expected_expiration = expires_on;
        let token = credential.get_token("").await.unwrap();
        assert!(token.expires_on - expected_expiration < TimeSpan::seconds(1));
    }

    #[tokio::test]
    async fn get_token_does_not_extend_a_token_close_to_expiring_when_created_without_the_key() {
        let expires_on =
            crate::util::time::now_utc() + SharedAccessCredential::SIGNATURE_REFRESH_BUFFER / 2;
        let value = format!("SharedAccessSignature sr=https%3A%2F%2Ffake-test.servicebus.windows.net%2F&sig=nNBNavJfBiHuXUzWOLhSvI3bVgqbQUzA7Po8%2F4wQQng%3D&se={}&skn=fakeKey", expires_on.unix_timestamp());
        let source_signature = SharedAccessSignature::try_from_signature(&value).unwrap();
        let signature =
            SharedAccessSignature::try_from_signature(source_signature.value()).unwrap();
        let credential = SharedAccessCredential::from_signature(signature);

        let expected_expiration = expires_on;
        let token = credential.get_token("").await.unwrap();
        assert!(token.expires_on - expected_expiration < TimeSpan::seconds(1));
    }

    // // TODO: This test won't work in rust because source_credential is moved into the credential
    // #[test]
    // fn named_key_credential_updates_are_respected() {
    //     let updated_key_name = "updated-name";
    //     let updated_key = "updated-key";
    //     let signature =
    //         SharedAccessSignature::try_from_parts("hub-name", "keyName", "key", None).unwrap();
    //     let source_credential = AzureNamedKeyCredential::new(
    //         signature.shared_access_key_name(),
    //         signature.shared_access_key(),
    //     );
    //     let mut credential = SharedAccessCredential::try_from_named_key_credential(
    //         source_credential,
    //         signature.resource(),
    //     ).unwrap();

    //     source_credential.update(updated_key_name, updated_key);
    //     todo!("")
    // }

    // // TODO: Similar to the one above, this wouldn't work in rust because the source_credential
    // // is moved into the SharedAccessCredential
    // #[test]
    // fn sas_credential_updates_are_respected() {
    //     todo!()
    // }
}
