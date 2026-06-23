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
//!
//! Note that the percent-encoded resource is fully lowercased, including the
//! escape hex digits (`%3a`/`%2f`, not `%3A`/`%2F`), to match Go's
//! `strings.ToLower(url.QueryEscape(...))`. A reader comparing against the C#
//! samples (which emit uppercase hex) should expect this difference; the broker
//! recomputes the HMAC over whatever `sr` it receives, so case only has to be
//! self-consistent between the string-to-sign and the emitted `sr`.

use super::connection_string::ConnectionString;
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::ErrorKind,
    fmt::SafeDebug,
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

// `SafeDebug` (not the std `Debug` derive) so the `audience` resource URI and
// the `key_name` authorization policy are redacted, matching the rest of this
// crate's secret hygiene. `key`/`token` are already `Secret` and self-redact.
#[derive(SafeDebug)]
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
    /// It cannot be refreshed (we have no key), so `get_token` reports the
    /// token's own `se` as the expiry rather than a rolling client-side window.
    /// Reporting the real deadline keeps the client's view in sync with the
    /// broker's; the connection's refresher detects the non-advancing expiry
    /// and stops trying to renew it (see `Authorizer::refresh_tokens`).
    Preformed {
        token: Secret,
        /// The token's own `se` expiry, parsed once at construction.
        expires_on: OffsetDateTime,
    },
}

/// A SAS [`TokenCredential`] for Event Hubs connection-string authentication.
#[derive(SafeDebug)]
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
    ///
    /// Validates the token's shape and parses its `se` expiry up front, so a
    /// malformed or truncated signature fails at open time with a clear error
    /// rather than as an opaque broker 401 at connect time.
    pub(crate) fn from_signature(signature: Secret) -> Result<Self> {
        let expires_on = parse_sas_expiry(signature.secret())?;
        Ok(Self {
            kind: Kind::Preformed {
                token: signature,
                expires_on,
            },
        })
    }

    /// Builds the appropriate credential from a parsed connection string for the
    /// given Event Hub: a pre-formed signature if present, otherwise a shared
    /// access key signer. The parser guarantees one of the two is available.
    pub(crate) fn from_connection_string(
        connection_string: &ConnectionString,
        eventhub: &str,
    ) -> Result<Self> {
        if let Some(signature) = &connection_string.shared_access_signature {
            return Self::from_signature(signature.clone());
        }
        // The parser guarantees that when no signature is present, both the key
        // name and key are `Some` (see the `has_key` check in `from_str`). Assert
        // that invariant rather than silently signing with empty values, which
        // would produce a token the broker rejects with an opaque 401.
        Ok(Self::from_shared_access_key(
            &connection_string.fully_qualified_namespace,
            eventhub,
            connection_string.shared_access_key_name.clone().expect(
                "parser guarantees SharedAccessKeyName is present when no signature is set",
            ),
            connection_string
                .shared_access_key
                .clone()
                .expect("parser guarantees SharedAccessKey is present when no signature is set"),
        ))
    }
}

/// Validates a pre-formed `SharedAccessSignature` token's shape and extracts its
/// `se` (expiry, Unix seconds) field. A token that does not look like the
/// signer's output (wrong prefix, missing or non-numeric `se`) is rejected here
/// so the failure surfaces at open time rather than as an opaque broker 401.
fn parse_sas_expiry(token: &str) -> Result<OffsetDateTime> {
    let body = token
        .strip_prefix("SharedAccessSignature ")
        .ok_or_else(|| {
            Error::new(
                ErrorKind::DataConversion,
                "pre-formed SharedAccessSignature must start with 'SharedAccessSignature '",
            )
        })?;
    let se = body
        .split('&')
        .find_map(|field| field.strip_prefix("se="))
        .ok_or_else(|| {
            Error::new(
                ErrorKind::DataConversion,
                "pre-formed SharedAccessSignature is missing its 'se' expiry field",
            )
        })?;
    let se: i64 = se.parse().map_err(|e| {
        Error::with_error(
            ErrorKind::DataConversion,
            e,
            "pre-formed SharedAccessSignature has a non-numeric 'se' expiry",
        )
    })?;
    OffsetDateTime::from_unix_timestamp(se).map_err(|e| {
        Error::with_error(
            ErrorKind::DataConversion,
            e,
            "pre-formed SharedAccessSignature has an out-of-range 'se' expiry",
        )
    })
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
    // The signature (base64-encoded HMAC bytes, then percent-encoded) is NOT
    // lowercased; only the resource is.
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
            Kind::Preformed { token, expires_on } => {
                // Report the token's own `se` as the expiry. A pre-formed token
                // cannot be re-signed, so once it is past `se` there is nothing
                // to hand back; fail with a clear error instead of re-presenting
                // an expired token and letting the broker drop the link.
                if OffsetDateTime::now_utc() >= *expires_on {
                    return Err(Error::new(
                        ErrorKind::Credential,
                        "pre-formed SharedAccessSignature has expired and cannot be refreshed; \
                         supply a fresh token or use a SharedAccessKey for automatic renewal",
                    ));
                }
                Ok(AccessToken::new(token.clone(), *expires_on))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // cspell:ignore fexample fmyhub myhub mykey fhub WDCF Bfsxrm Lsorq supersecretkey
    use super::*;

    // Golden vector generated independently (Python, replicating Go's
    // `azure-sdk-for-go` SAS signer). Pins the raw-key HMAC, the encoding, the
    // field order, and the `sr` (lowercased percent-encoded resource) vs `sig`
    // (base64-encoded HMAC, percent-encoded, NOT lowercased) casing asymmetry.
    // If any of those regress, this fails.
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

    // `SafeDebug` must redact the audience and key name (and the `Secret` key).
    // Guards against a regression to the std `Debug` derive on this
    // security-sensitive type.
    #[test]
    fn debug_does_not_leak_key_name_or_audience() {
        let cred = SasCredential::from_shared_access_key(
            "example.servicebus.windows.net",
            "myhub",
            "RootManageSharedAccessKey".to_string(),
            Secret::new("supersecretkey"),
        );
        let debug = format!("{cred:?}");
        assert!(!debug.contains("RootManageSharedAccessKey"), "{debug}");
        assert!(!debug.contains("supersecretkey"), "{debug}");
    }

    #[tokio::test]
    async fn preformed_token_is_returned_verbatim_with_real_se_expiry() {
        // `se` far in the future so the token is still valid. The token is
        // returned unchanged and its expiry is the token's own `se`, not a
        // rolling client-side window.
        let sig =
            "SharedAccessSignature sr=amqps%3a%2f%2fns%2fhub&sig=abc%3d&se=4102444800&skn=policy";
        let cred = SasCredential::from_signature(Secret::new(sig)).unwrap();
        let token = cred.get_token(&[], None).await.unwrap();
        assert_eq!(token.token.secret(), sig);
        assert_eq!(token.expires_on.unix_timestamp(), 4102444800);
    }

    #[tokio::test]
    async fn preformed_expired_token_errors() {
        // `se` in the past (2023): the shape is valid so construction succeeds,
        // but `get_token` refuses to hand back an expired token.
        let sig =
            "SharedAccessSignature sr=amqps%3a%2f%2fns%2fhub&sig=abc%3d&se=1700000000&skn=policy";
        let cred = SasCredential::from_signature(Secret::new(sig)).unwrap();
        let err = cred.get_token(&[], None).await.unwrap_err();
        assert!(format!("{err}").contains("expired"), "{err}");
    }

    #[test]
    fn from_signature_rejects_malformed_tokens() {
        // Wrong prefix.
        assert!(SasCredential::from_signature(Secret::new("se=4102444800")).is_err());
        // Missing `se`.
        assert!(SasCredential::from_signature(Secret::new(
            "SharedAccessSignature sr=amqps%3a%2f%2fns%2fhub&sig=abc%3d&skn=policy"
        ))
        .is_err());
        // Non-numeric `se`.
        assert!(SasCredential::from_signature(Secret::new(
            "SharedAccessSignature sr=amqps%3a%2f%2fns%2fhub&sig=abc%3d&se=abc&skn=policy"
        ))
        .is_err());
    }
}
