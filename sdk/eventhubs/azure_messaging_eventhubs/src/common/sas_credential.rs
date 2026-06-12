// Copyright (c) Microsoft Corporation. All rights reserved
// Licensed under the MIT license.

// cspell:ignore sastoken skn

//! A [`TokenCredential`] that produces Event Hubs Shared Access Signature (SAS)
//! tokens from a shared access key (or wraps a pre-formed signature).
//!
//! The token format and signing algorithm match the other Azure SDKs (verified
//! against `azure-sdk-for-go`), so the broker accepts the result:
//!
//! ```text
//! SharedAccessSignature sr={url-encoded resource}&sig={url-encoded HMAC}&se={expiry}&skn={key name}
//! ```
//!
//! where the string-to-sign is `url_encode(resource) + "\n" + expiry` and the
//! HMAC-SHA256 is computed over the **raw** key bytes (the `SharedAccessKey` is
//! not base64-decoded first).

use super::connection_string::ConnectionString;
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::ErrorKind,
    time::{Duration, OffsetDateTime},
    Error, Result,
};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use sha2::Sha256;

/// Default validity window for a generated SAS token. The connection's
/// authorizer refreshes tokens well before this elapses.
const DEFAULT_TOKEN_VALIDITY: Duration = Duration::hours(1);

/// Percent-encoding set matching Go's `url.QueryEscape` (and thus the other
/// Azure SDKs): escape everything that is not alphanumeric, except the four
/// unreserved marks `-` `_` `.` `~`. The resource must be encoded identically
/// in the string-to-sign and in the emitted `sr` field, or the broker computes
/// a different signature and rejects the token.
const SAS_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

#[derive(Debug)]
enum Kind {
    /// Sign on demand with a shared access key.
    SharedKey {
        /// Resource URI to sign, e.g. `amqps://ns.servicebus.windows.net/hub`.
        audience: String,
        key_name: String,
        key: Secret,
        validity: Duration,
    },
    /// A pre-formed `SharedAccessSignature ...` token supplied by the caller.
    ///
    /// It cannot be refreshed (we have no key), so we report a rolling
    /// client-side expiry and let the broker reject it once its own `se`
    /// elapses. This matches the behavior of the other Azure SDKs.
    Preformed { token: Secret },
}

/// A SAS [`TokenCredential`] for Event Hubs connection-string authentication.
#[derive(Debug)]
pub(crate) struct SasCredential {
    kind: Kind,
}

impl SasCredential {
    /// Builds a credential that signs the entity-scoped resource
    /// `amqps://{fully_qualified_namespace}/{eventhub}` with a shared access
    /// key. An entity-scoped token authorizes every partition and consumer
    /// group beneath the Event Hub, so a single token covers all links.
    pub(crate) fn from_shared_access_key(
        fully_qualified_namespace: &str,
        eventhub: &str,
        key_name: String,
        key: Secret,
    ) -> Self {
        Self {
            kind: Kind::SharedKey {
                audience: format!("amqps://{fully_qualified_namespace}/{eventhub}"),
                key_name,
                key,
                validity: DEFAULT_TOKEN_VALIDITY,
            },
        }
    }

    /// Wraps a pre-formed `SharedAccessSignature` token from a connection string.
    pub(crate) fn from_signature(signature: Secret) -> Self {
        Self {
            kind: Kind::Preformed { token: signature },
        }
    }

    /// Builds the appropriate credential from a parsed connection string for the
    /// given Event Hub: a pre-formed signature if present, otherwise a shared
    /// access key signer. The parser guarantees one of the two is available.
    pub(crate) fn from_connection_string(
        connection_string: &ConnectionString,
        eventhub: &str,
    ) -> Self {
        if let Some(signature) = &connection_string.shared_access_signature {
            return Self::from_signature(signature.clone());
        }
        Self::from_shared_access_key(
            &connection_string.fully_qualified_namespace,
            eventhub,
            connection_string
                .shared_access_key_name
                .clone()
                .unwrap_or_default(),
            connection_string
                .shared_access_key
                .clone()
                .unwrap_or_else(|| Secret::new(String::new())),
        )
    }
}

/// Builds a SAS token string for `audience`, valid until `expiry` (Unix
/// seconds), signed with `key` (used as raw HMAC-SHA256 key bytes).
///
/// Factored out of [`SasCredential::get_token`] so it can be tested against a
/// fixed expiry / known golden vector.
fn sign_sas(audience: &str, key_name: &str, key: &Secret, expiry: i64) -> Result<String> {
    // Resource is lowercased after encoding, matching the reference SDKs.
    let resource = utf8_percent_encode(audience, SAS_ENCODE_SET)
        .to_string()
        .to_lowercase();
    let string_to_sign = format!("{resource}\n{expiry}");

    let mut mac = Hmac::<Sha256>::new_from_slice(key.secret().as_bytes())
        .map_err(|e| Error::with_error(ErrorKind::Other, e, "invalid SAS signing key"))?;
    mac.update(string_to_sign.as_bytes());
    let signature = BASE64_STANDARD.encode(mac.finalize().into_bytes());
    // The signature hex is NOT lowercased (only the resource is).
    let signature = utf8_percent_encode(&signature, SAS_ENCODE_SET).to_string();

    Ok(format!(
        "SharedAccessSignature sr={resource}&sig={signature}&se={expiry}&skn={key_name}"
    ))
}

#[async_trait::async_trait]
impl TokenCredential for SasCredential {
    async fn get_token(
        &self,
        _scopes: &[&str],
        _options: Option<TokenRequestOptions<'_>>,
    ) -> Result<AccessToken> {
        match &self.kind {
            Kind::SharedKey {
                audience,
                key_name,
                key,
                validity,
            } => {
                let expires_on = OffsetDateTime::now_utc() + *validity;
                let token = sign_sas(audience, key_name, key, expires_on.unix_timestamp())?;
                Ok(AccessToken::new(Secret::new(token), expires_on))
            }
            Kind::Preformed { token } => {
                // Rolling client-side expiry: the real lifetime is enforced by
                // the broker via the token's own `se` field.
                let expires_on = OffsetDateTime::now_utc() + DEFAULT_TOKEN_VALIDITY;
                Ok(AccessToken::new(token.clone(), expires_on))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // cspell:ignore fexample fmyhub myhub mykey fhub WDCF Bfsxrm Lsorq
    use super::*;

    // Golden vector generated independently (Python, replicating Go's
    // `azure-sdk-for-go` SAS signer). Pins the raw-key HMAC, the encoding, the
    // field order, and the `sr` (lowercase hex) vs `sig` (uppercase hex)
    // casing asymmetry. If any of those regress, this fails.
    #[test]
    fn sign_sas_matches_reference_vector() {
        let token = sign_sas(
            "amqps://example.servicebus.windows.net/myhub",
            "RootManageSharedAccessKey",
            &Secret::new("mykey"),
            1_700_000_000,
        )
        .unwrap();

        assert_eq!(
            token,
            "SharedAccessSignature \
             sr=amqps%3a%2f%2fexample.servicebus.windows.net%2fmyhub\
             &sig=SgJoMn7K6nWDCF6e1%2BfsxrmJLsorqPeZ3B8N1uQ31dc%3D\
             &se=1700000000\
             &skn=RootManageSharedAccessKey"
        );
    }

    // The key is used as raw bytes, NOT base64-decoded. "bXlrZXk=" is the
    // base64 of "mykey"; signing with it must NOT equal signing with "mykey".
    #[test]
    fn sign_sas_uses_raw_key_bytes() {
        let raw = sign_sas("amqps://ns/hub", "policy", &Secret::new("mykey"), 100).unwrap();
        let b64 = sign_sas("amqps://ns/hub", "policy", &Secret::new("bXlrZXk="), 100).unwrap();
        assert_ne!(raw, b64);
    }

    #[tokio::test]
    async fn shared_key_token_has_future_expiry_and_sas_shape() {
        let cred = SasCredential::from_shared_access_key(
            "example.servicebus.windows.net",
            "myhub",
            "policy".to_string(),
            Secret::new("key"),
        );
        let before = OffsetDateTime::now_utc();
        let token = cred.get_token(&[], None).await.unwrap();
        assert!(token
            .token
            .secret()
            .starts_with("SharedAccessSignature sr="));
        assert!(token.expires_on > before);
    }

    #[tokio::test]
    async fn preformed_token_is_returned_verbatim() {
        let sig =
            "SharedAccessSignature sr=amqps%3a%2f%2fns%2fhub&sig=abc%3d&se=1700000000&skn=policy";
        let cred = SasCredential::from_signature(Secret::new(sig));
        let token = cred.get_token(&[], None).await.unwrap();
        assert_eq!(token.token.secret(), sig);
    }
}
