// Copyright (c) Microsoft Corporation. All rights reserved
// Licensed under the MIT license.

//! Parsing for Event Hubs (Service Bus) connection strings.
//!
//! A connection string is a semicolon-delimited list of `Key=Value` pairs, for
//! example:
//!
//! ```text
//! Endpoint=sb://<namespace>.servicebus.windows.net/;SharedAccessKeyName=<policy>;SharedAccessKey=<key>;EntityPath=<eventhub>
//! ```
//!
//! The same shape is produced by the Azure portal and the `az` CLI for both
//! Event Hubs and Service Bus, and it is interchangeable with the other Azure
//! SDKs. Either a `SharedAccessKeyName`/`SharedAccessKey` pair or a pre-formed
//! `SharedAccessSignature` must be present.

use azure_core::{credentials::Secret, error::ErrorKind, fmt::SafeDebug, http::Url, Error};
use std::str::FromStr;

/// A parsed Event Hubs connection string.
///
/// Construct one with [`str::parse`] or [`ConnectionString::try_from`]:
///
/// ```
/// use azure_messaging_eventhubs::ConnectionString;
///
/// let cs: ConnectionString =
///     "Endpoint=sb://example.servicebus.windows.net/;SharedAccessKeyName=policy;SharedAccessKey=abc123"
///         .parse()?;
/// assert_eq!(cs.fully_qualified_namespace, "example.servicebus.windows.net");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// The type is `#[non_exhaustive]`: it is only ever constructed by parsing, so
/// fields can be read but not built with a struct literal outside this crate,
/// which lets new fields be added without a breaking change.
#[derive(Clone, PartialEq, Eq, SafeDebug)]
#[non_exhaustive]
pub struct ConnectionString {
    /// The raw `Endpoint` value, e.g. `sb://example.servicebus.windows.net/`.
    pub endpoint: String,

    /// The host extracted from `endpoint`, e.g. `example.servicebus.windows.net`.
    ///
    /// This is the value the client passes as the fully qualified namespace.
    pub fully_qualified_namespace: String,

    /// The `SharedAccessKeyName` (the authorization policy name), if present.
    pub shared_access_key_name: Option<String>,

    /// The `SharedAccessKey` (the secret), if present.
    pub shared_access_key: Option<Secret>,

    /// A pre-formed `SharedAccessSignature` token, if supplied instead of a key.
    pub shared_access_signature: Option<Secret>,

    /// The `EntityPath` (the Event Hub name), if the connection string is
    /// scoped to a specific Event Hub.
    pub entity_path: Option<String>,
}

impl TryFrom<&Secret> for ConnectionString {
    type Error = Error;
    fn try_from(secret: &Secret) -> Result<Self, Self::Error> {
        secret.secret().parse()
    }
}

impl FromStr for ConnectionString {
    type Err = Error;
    fn from_str(connection_string: &str) -> Result<Self, Self::Err> {
        if connection_string.is_empty() {
            return Err(Error::new(
                ErrorKind::DataConversion,
                "connection string cannot be empty",
            ));
        }

        let mut endpoint = None;
        let mut shared_access_key_name = None;
        let mut shared_access_key = None;
        let mut shared_access_signature = None;
        let mut entity_path = None;

        for part in connection_string.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            // Split on the *first* '=' only: base64 keys end in '='/'==', and a
            // `SharedAccessSignature` value contains several '=' of its own.
            let (key, value) = part.split_once('=').ok_or_else(|| {
                Error::new(ErrorKind::DataConversion, "invalid connection string")
            })?;

            // Keys are matched case-insensitively to mirror the other Azure SDKs.
            if key.eq_ignore_ascii_case("Endpoint") {
                endpoint = Some(value.to_string());
            } else if key.eq_ignore_ascii_case("SharedAccessKeyName") {
                shared_access_key_name = Some(value.to_string());
            } else if key.eq_ignore_ascii_case("SharedAccessKey") {
                shared_access_key = Some(Secret::new(value.to_string()));
            } else if key.eq_ignore_ascii_case("SharedAccessSignature") {
                shared_access_signature = Some(Secret::new(value.to_string()));
            } else if key.eq_ignore_ascii_case("EntityPath") {
                entity_path = Some(value.to_string());
            }
            // Unknown keys are ignored for forward compatibility.
        }

        let Some(endpoint) = endpoint else {
            return Err(Error::new(
                ErrorKind::DataConversion,
                "invalid connection string, missing 'Endpoint'",
            ));
        };

        // Require either a name+key pair or a pre-formed signature.
        let has_key = shared_access_key_name.is_some() && shared_access_key.is_some();
        if !has_key && shared_access_signature.is_none() {
            return Err(Error::new(
                ErrorKind::DataConversion,
                "invalid connection string, missing shared access key or signature",
            ));
        }

        // The fully qualified namespace is the host of the endpoint. The scheme
        // is `sb://`, but we accept any scheme and simply take the host.
        let parsed = Url::parse(&endpoint).map_err(|e| {
            Error::with_error(
                ErrorKind::DataConversion,
                e,
                "invalid connection string, 'Endpoint' is not a valid URL",
            )
        })?;
        let fully_qualified_namespace = parsed
            .host_str()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::DataConversion,
                    "invalid connection string, 'Endpoint' has no host",
                )
            })?
            .to_string();

        Ok(Self {
            endpoint,
            fully_qualified_namespace,
            shared_access_key_name,
            shared_access_key,
            shared_access_signature,
            entity_path,
        })
    }
}

/// Resolves the Event Hub name from an explicit argument and the connection
/// string's `EntityPath`, rejecting a conflict between the two.
///
/// * explicit only -> explicit
/// * `EntityPath` only -> `EntityPath`
/// * both, equal -> that value
/// * both, different -> error (a silent precedence rule hides copy/paste bugs)
/// * neither -> error
pub(crate) fn resolve_eventhub(
    connection_string: &ConnectionString,
    explicit: Option<&str>,
) -> Result<String, Error> {
    match (explicit, connection_string.entity_path.as_deref()) {
        (Some(arg), Some(entity)) if arg != entity => Err(Error::new(
            ErrorKind::Other,
            format!(
                "event hub name '{arg}' conflicts with EntityPath '{entity}' in the connection string"
            ),
        )),
        (Some(arg), _) => Ok(arg.to_string()),
        (None, Some(entity)) => Ok(entity.to_string()),
        (None, None) => Err(Error::new(
            ErrorKind::Other,
            "no event hub name: provide one or include 'EntityPath' in the connection string",
        )),
    }
}

#[cfg(test)]
mod tests {
    // cspell:ignore fexample sharedaccesskeyname sharedaccesskey fhub supersecretkey topsecretsig
    use super::{resolve_eventhub, ConnectionString};
    use azure_core::credentials::Secret;

    #[test]
    fn valid_key_connection_string() {
        let cs: ConnectionString = "Endpoint=sb://example.servicebus.windows.net/;SharedAccessKeyName=RootManageSharedAccessKey;SharedAccessKey=abc123=="
            .parse()
            .unwrap();
        assert_eq!(cs.endpoint, "sb://example.servicebus.windows.net/");
        assert_eq!(
            cs.fully_qualified_namespace,
            "example.servicebus.windows.net"
        );
        assert_eq!(
            cs.shared_access_key_name.as_deref(),
            Some("RootManageSharedAccessKey")
        );
        // A base64 value ending in '==' must survive `split_once`.
        assert_eq!(cs.shared_access_key.unwrap().secret(), "abc123==");
        assert!(cs.shared_access_signature.is_none());
        assert!(cs.entity_path.is_none());
    }

    #[test]
    fn valid_signature_connection_string() {
        let sig = "SharedAccessSignature sr=sb%3a%2f%2fexample.servicebus.windows.net%2feh&sig=abc%3d&se=1700000000&skn=policy";
        let cs: ConnectionString =
            format!("Endpoint=sb://example.servicebus.windows.net/;SharedAccessSignature={sig}")
                .parse()
                .unwrap();
        assert_eq!(cs.shared_access_signature.unwrap().secret(), sig);
        assert!(cs.shared_access_key.is_none());
    }

    #[test]
    fn case_insensitive_keys() {
        let cs: ConnectionString = "endpoint=sb://example.servicebus.windows.net/;sharedaccesskeyname=policy;sharedaccesskey=key"
            .parse()
            .unwrap();
        assert_eq!(
            cs.fully_qualified_namespace,
            "example.servicebus.windows.net"
        );
        assert_eq!(cs.shared_access_key_name.as_deref(), Some("policy"));
    }

    #[test]
    fn entity_path_parsed() {
        let cs: ConnectionString = "Endpoint=sb://example.servicebus.windows.net/;SharedAccessKeyName=policy;SharedAccessKey=key;EntityPath=my-hub"
            .parse()
            .unwrap();
        assert_eq!(cs.entity_path.as_deref(), Some("my-hub"));
    }

    #[test]
    fn try_from_secret() {
        let secret = Secret::new(
            "Endpoint=sb://example.servicebus.windows.net/;SharedAccessKeyName=policy;SharedAccessKey=key"
                .to_string(),
        );
        let cs = ConnectionString::try_from(&secret).unwrap();
        assert_eq!(
            cs.fully_qualified_namespace,
            "example.servicebus.windows.net"
        );
    }

    #[test]
    fn empty_is_rejected() {
        assert_bad("", "connection string cannot be empty");
    }

    #[test]
    fn part_without_equals_is_rejected() {
        assert_bad(
            "Endpoint=sb://example.servicebus.windows.net/;SharedAccessKeyName;SharedAccessKey=key",
            "invalid connection string",
        );
    }

    #[test]
    fn missing_endpoint_is_rejected() {
        assert_bad(
            "SharedAccessKeyName=policy;SharedAccessKey=key",
            "invalid connection string, missing 'Endpoint'",
        );
    }

    #[test]
    fn missing_key_and_signature_is_rejected() {
        assert_bad(
            "Endpoint=sb://example.servicebus.windows.net/;SharedAccessKeyName=policy",
            "invalid connection string, missing shared access key or signature",
        );
    }

    #[test]
    fn endpoint_without_host_is_rejected() {
        assert_bad(
            "Endpoint=not-a-url;SharedAccessKeyName=policy;SharedAccessKey=key",
            "invalid connection string, 'Endpoint' is not a valid URL",
        );
    }

    #[test]
    fn resolve_eventhub_rules() {
        let with_entity: ConnectionString = "Endpoint=sb://example.servicebus.windows.net/;SharedAccessKeyName=policy;SharedAccessKey=key;EntityPath=hub"
            .parse()
            .unwrap();
        let without_entity: ConnectionString = "Endpoint=sb://example.servicebus.windows.net/;SharedAccessKeyName=policy;SharedAccessKey=key"
            .parse()
            .unwrap();

        // explicit only
        assert_eq!(
            resolve_eventhub(&without_entity, Some("hub")).unwrap(),
            "hub"
        );
        // entity only
        assert_eq!(resolve_eventhub(&with_entity, None).unwrap(), "hub");
        // both, equal
        assert_eq!(resolve_eventhub(&with_entity, Some("hub")).unwrap(), "hub");
        // both, different -> error
        assert!(resolve_eventhub(&with_entity, Some("other")).is_err());
        // neither -> error
        assert!(resolve_eventhub(&without_entity, None).is_err());
    }

    #[test]
    fn debug_does_not_leak_secrets() {
        // `SafeDebug` must redact the key (and every other field). This guards
        // against a future `#[derive(Debug)]` or `#[safe(true)]` regression on a
        // security-sensitive type.
        let cs: ConnectionString = "Endpoint=sb://example.servicebus.windows.net/;SharedAccessKeyName=policy;SharedAccessKey=supersecretkey;EntityPath=hub"
            .parse()
            .unwrap();
        let debug = format!("{cs:?}");
        assert!(
            !debug.contains("supersecretkey"),
            "Debug output leaked the shared access key: {debug}"
        );
    }

    #[test]
    fn debug_does_not_leak_preformed_signature() {
        let sig = "SharedAccessSignature sr=amqps%3a%2f%2fns%2fhub&sig=topsecretsig&se=1700000000&skn=policy";
        let cs: ConnectionString =
            format!("Endpoint=sb://example.servicebus.windows.net/;SharedAccessSignature={sig}")
                .parse()
                .unwrap();
        let debug = format!("{cs:?}");
        assert!(
            !debug.contains("topsecretsig"),
            "Debug output leaked the pre-formed signature: {debug}"
        );
    }

    fn assert_bad(connection_string: &str, expected: &str) {
        let err = connection_string.parse::<ConnectionString>().unwrap_err();
        assert_eq!(format!("{err}"), expected);
    }
}
