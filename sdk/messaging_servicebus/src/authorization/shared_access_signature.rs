use std::{borrow::Cow, num::ParseIntError, string::FromUtf8Error, time::Duration};

use azure_core::error::ResultExt;
use const_format::formatcp;
use digest::{Digest, InvalidLength, KeyInit, Mac};
use hmac::Hmac;
use sha2::Sha256;
use time::OffsetDateTime;

use crate::constants::DEFAULT_OFFSET_DATE_TIME;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    HmacSha256(#[from] InvalidLength),

    #[error("shared_access_key_name exceeds MAXIMUM_KEY_NAME_LENGTH")]
    SasKeyNameTooLong,

    #[error("shared_access_key exceeds MAXIMUM_KEY_LENGTH")]
    SasKeyTooLong,

    #[error("Malformed shared_access_signature")]
    InvalidSharedAccessSignaure,
}

pub(crate) struct SharedAccessSignature {
    shared_access_key_name: String,
    shared_access_key: String,
    signature_expiration: OffsetDateTime,
    resource: String,
    value: String,
}

struct SignatureParts<'a> {
    pub key_name: Option<Cow<'a, str>>,
    pub resource: Option<Cow<'a, str>>,
    pub expiration_time: OffsetDateTime,
}

impl SharedAccessSignature {
    /// <summary>The maximum allowed length of the SAS key name.</summary>
    pub(crate) const MAXIMUM_KEY_NAME_LENGTH: usize = 256;

    /// <summary>The maximum allowed length of the SAS key.</summary>
    const MAXIMUM_KEY_LENGTH: usize = 256;

    /// <summary>The token that represents the type of authentication used.</summary>
    const AUTHENTICATION_TYPE_TOKEN: &'static str = "SharedAccessSignature";

    /// <summary>The token that identifies the signed component of the shared access signature.</summary>
    const SIGNED_RESOURCE_TOKEN: &'static str = "sr";

    /// <summary>The token that identifies the signature component of the shared access signature.</summary>
    const SIGNATURE_TOKEN: &'static str = "sig";

    /// <summary>The token that identifies the signed SAS key component of the shared access signature.</summary>
    const SIGNED_KEY_NAME_TOKEN: &'static str = "skn";

    /// <summary>The token that identifies the signed expiration time of the shared access signature.</summary>
    const SIGNED_EXPIRY_TOKEN: &'static str = "se";

    /// <summary>The token that fully identifies the signed resource within the signature.</summary>
    // AuthenticationTypeToken + " " + SignedResourceToken;
    const SIGNED_RESOURCE_FULL_IDENTIFIER_TOKEN: &'static str = "SharedAccessSignature sr";

    /// <summary>The character used to separate a token and its value in the connection string.</summary>
    const TOKEN_VALUE_SEPARATOR: char = '=';

    /// <summary>The character used to mark the beginning of a new token/value pair in the signature.</summary>
    const TOKEN_VALUE_PAIR_DELIMITER: char = '&';

    /// <summary>The default length of time to consider a signature valid, if not otherwise specified.</summary>
    const DEFAULT_SIGNATURE_VALIDITY_DURATION: Duration = Duration::from_secs(30 * 60); // 30 mins

    /// <summary>Represents the Unix epoch time value, January 1, 1970 12:00:00, UTC.</summary>
    // static readonly DateTimeOffset Epoch = new DateTimeOffset(1970, 1, 1, 0, 0, 0, TimeSpan.Zero);
    const EPOCH: OffsetDateTime = OffsetDateTime::UNIX_EPOCH;
}

impl SharedAccessSignature {
    /// The name of the shared access key, either for the Service Bus namespace or the Service Bus
    /// entity.
    pub fn shared_access_key_name(&self) -> &str {
        &self.shared_access_key_name
    }

    /// The value of the shared access key, either for the Service Bus namespace or the Service Bus
    /// entity.
    pub fn shared_access_key(&self) -> &str {
        &self.shared_access_key
    }

    /// The date and time that the shared access signature expires, in UTC.
    pub fn signature_expiration(&self) -> &OffsetDateTime {
        &self.signature_expiration
    }

    /// The resource to which the shared access signature is intended to serve as authorization.
    pub fn resource(&self) -> &str {
        &self.resource
    }

    /// The shared access signature to be used for authorization, either for the Service Bus
    /// namespace or the Service Bus entity.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// <summary>
    ///   Initializes a new instance of the <see cref="SharedAccessSignature"/> class.
    /// </summary>
    ///
    /// <param name="serviceBusResource">The Service Bus resource to which the token is intended to serve as authorization.</param>
    /// <param name="sharedAccessKeyName">The name of the shared access key that the signature should be based on.</param>
    /// <param name="sharedAccessKey">The value of the shared access key for the signature.</param>
    /// <param name="signatureValidityDuration">The duration that the signature should be considered valid; if not specified, a default will be assumed.</param>
    ///
    pub fn try_from_parts(
        service_bus_resource: impl Into<String>,
        shared_access_key_name: impl Into<String>,
        shared_access_key: impl Into<String>,
        signature_validity_duration: Option<Duration>,
    ) -> Result<Self, Error> {
        let service_bus_resource = service_bus_resource.into();
        let shared_access_key_name = shared_access_key_name.into();
        let shared_access_key = shared_access_key.into();

        let signature_validity_duration =
            signature_validity_duration.unwrap_or(Self::DEFAULT_SIGNATURE_VALIDITY_DURATION);
        if shared_access_key_name.len() > Self::MAXIMUM_KEY_NAME_LENGTH {
            return Err(Error::SasKeyNameTooLong);
        }
        if shared_access_key.len() > Self::MAXIMUM_KEY_LENGTH {
            return Err(Error::SasKeyTooLong);
        }

        let signature_expiration = OffsetDateTime::now_utc() + signature_validity_duration;

        let resource = service_bus_resource;
        let value = Self::build_signature(
            &resource,
            &shared_access_key_name,
            &shared_access_key,
            signature_expiration,
        )?;

        Ok(Self {
            shared_access_key_name,
            shared_access_key,
            signature_expiration,
            resource,
            value,
        })
    }

    /// <summary>
    ///   Initializes a new instance of the <see cref="SharedAccessSignature"/> class.
    /// </summary>
    ///
    /// <param name="sharedAccessSignature">The shared access signature that will be parsed as the basis of this instance.</param>
    /// <param name="sharedAccessKey">The value of the shared access key for the signature.</param>
    ///
    pub fn try_from_signature_and_key(
        shared_access_signature: impl Into<String>,
        shared_access_key: impl Into<String>,
    ) -> Result<Self, Error> {
        // Argument.AssertNotNullOrEmpty(sharedAccessSignature, nameof(sharedAccessSignature));
        // Argument.AssertNotTooLong(sharedAccessKey, MaximumKeyLength, nameof(sharedAccessKey));

        // (SharedAccessKeyName, Resource, SignatureExpiration) =
        //     ParseSignature(sharedAccessSignature);

        // SharedAccessKey = sharedAccessKey;
        // Value = sharedAccessSignature;

        let shared_access_signature = shared_access_signature.into();
        let shared_access_key = shared_access_key.into();

        if shared_access_key.len() > Self::MAXIMUM_KEY_LENGTH {
            return Err(Error::SasKeyTooLong);
        }

        todo!()
    }

    /// <summary>
    ///   Parses a shared access signature into its component parts.
    /// </summary>
    ///
    /// <param name="sharedAccessSignature">The shared access signature to parse.</param>
    ///
    /// <returns>The set of composite properties parsed from the signature.</returns>
    ///
    fn parse_signature(shared_access_signature: &str) -> Result<SignatureParts, Error> {
        let mut key_name = None;
        let mut resource = None;
        let mut expiration_time = DEFAULT_OFFSET_DATE_TIME;

        let token_value_pairs = shared_access_signature.split(Self::TOKEN_VALUE_PAIR_DELIMITER);
        for token_value_pair in token_value_pairs {
            let mut split = token_value_pair.split(Self::TOKEN_VALUE_SEPARATOR);
            let token = split
                .next()
                .ok_or(Error::InvalidSharedAccessSignaure)?
                .trim();
            let value = split
                .next()
                .ok_or(Error::InvalidSharedAccessSignaure)?
                .trim();

            if value.is_empty() {
                return Err(Error::InvalidSharedAccessSignaure);
            }

            match token {
                Self::SIGNED_RESOURCE_FULL_IDENTIFIER_TOKEN => {
                    resource = Some(
                        urlencoding::decode(value)
                            .map_err(|_| Error::InvalidSharedAccessSignaure)?,
                    );
                }
                Self::SIGNED_KEY_NAME_TOKEN => {
                    key_name = Some(
                        urlencoding::decode(value)
                            .map_err(|_| Error::InvalidSharedAccessSignaure)?,
                    );
                }
                Self::SIGNED_EXPIRY_TOKEN => {
                    let value = urlencoding::decode(value)
                        .map_err(|_| Error::InvalidSharedAccessSignaure)?;
                    let unix_time: i64 = value
                        .parse()
                        .map_err(|_| Error::InvalidSharedAccessSignaure)?;
                    expiration_time = OffsetDateTime::from_unix_timestamp(unix_time)
                        .map_err(|_| Error::InvalidSharedAccessSignaure)?;
                }
                _ => {}
            }
        }

        Ok(SignatureParts {
            key_name,
            resource,
            expiration_time,
        })
    }

    /// <summary>
    ///   Builds the shared access signature value, which can be used as a token for
    ///   access to the Service Bus service.
    /// </summary>
    ///
    /// <param name="audience">The audience scope to which this signature applies.</param>
    /// <param name="sharedAccessKeyName">The name of the shared access key that the signature should be based on.</param>
    /// <param name="sharedAccessKey">The value of the shared access key for the signature.</param>
    /// <param name="expirationTime">The date/time, in UTC, that the signature expires.</param>
    ///
    /// <returns>The value of the shared access signature.</returns>
    ///
    fn build_signature(
        audience: &str,
        shared_access_key_name: &str,
        shared_access_key: &str,
        expiration_time: OffsetDateTime,
    ) -> Result<String, InvalidLength> {
        let encoded_audience: String =
            url::form_urlencoded::byte_serialize(audience.as_bytes()).collect();
        let expiration = convert_to_unix_time(expiration_time).to_string();
        let message = format!("{encoded_audience}\n{expiration}");
        let mac = mac::<Hmac<Sha256>>(shared_access_key.as_bytes(), message.as_bytes())?;
        let signature = base64::encode(mac.as_ref());

        let encoded_signature = urlencoding::encode(&signature);
        let encoded_expiration = urlencoding::encode(&expiration);
        let encoded_shared_access_key_name = urlencoding::encode(&shared_access_key_name);

        let s = format!(
            "{} {}={}&{}={}&{}={}&{}={}",
            Self::AUTHENTICATION_TYPE_TOKEN,
            Self::SIGNED_RESOURCE_TOKEN,
            encoded_audience,
            Self::SIGNATURE_TOKEN,
            encoded_signature,
            Self::SIGNED_EXPIRY_TOKEN,
            encoded_expiration,
            Self::SIGNED_KEY_NAME_TOKEN,
            encoded_shared_access_key_name
        );
        Ok(s)
    }
}

/// <summary>
///   Converts a <see cref="DateTimeOffset" /> value to the corresponding Unix-style timestamp.
/// </summary>
///
/// <param name="dateTimeOffset">The date/time to convert.</param>
///
/// <returns>The Unix-style timestamp which corresponds to the specified date/time.</returns>
fn convert_to_unix_time(offset_date_time: OffsetDateTime) -> i64 {
    offset_date_time.unix_timestamp()
}

fn mac<M: Mac + KeyInit>(key: &[u8], input: &[u8]) -> Result<impl AsRef<[u8]>, InvalidLength> {
    let mut mac = <M as Mac>::new_from_slice(key)?;
    mac.update(input);
    Ok(mac.finalize().into_bytes())
}
