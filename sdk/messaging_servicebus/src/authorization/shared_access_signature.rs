use std::{borrow::Cow, time::Duration};

use digest::{InvalidLength, KeyInit, Mac};
use hmac::Hmac;
use sha2::Sha256;
use time::OffsetDateTime;

use crate::constants::DEFAULT_OFFSET_DATE_TIME;

#[derive(Debug, thiserror::Error)]
pub enum SasSignatureError {
    #[error(transparent)]
    HmacSha256(#[from] InvalidLength),

    #[error("shared_access_key_name exceeds MAXIMUM_KEY_NAME_LENGTH")]
    SasKeyNameTooLong,

    #[error("shared_access_key exceeds MAXIMUM_KEY_LENGTH")]
    SasKeyTooLong,

    #[error("Malformed shared_access_signature")]
    InvalidSharedAccessSignaure,

    #[error("Argument is empty")]
    ArgumentIsEmpty,

    #[error("Shared Access Key is required")]
    SharedAccessKeyIsRequired,
}

impl From<SasSignatureError> for azure_core::Error {
    fn from(error: SasSignatureError) -> Self {
        Self::new(azure_core::error::ErrorKind::Other, error.to_string())
    }
}

/// TODO: visibility?
#[derive(Debug, Clone)]
pub(crate) struct SharedAccessSignature {
    shared_access_key_name: String,
    shared_access_key: String,
    signature_expiration: OffsetDateTime,
    resource: String,
    value: String,
}

pub(crate) struct SignatureParts<'a> {
    pub key_name: Cow<'a, str>,
    pub resource: Cow<'a, str>,
    pub expiration_time: OffsetDateTime,
}

impl SharedAccessSignature {
    /// The maximum allowed length of the SAS key name.
    pub(crate) const MAXIMUM_KEY_NAME_LENGTH: usize = 256;

    /// The maximum allowed length of the SAS key.
    const MAXIMUM_KEY_LENGTH: usize = 256;

    /// The token that represents the type of authentication used.
    const AUTHENTICATION_TYPE_TOKEN: &'static str = "SharedAccessSignature";

    /// The token that identifies the signed component of the shared access signature.
    const SIGNED_RESOURCE_TOKEN: &'static str = "sr";

    /// The token that identifies the signature component of the shared access signature.
    const SIGNATURE_TOKEN: &'static str = "sig";

    /// The token that identifies the signed SAS key component of the shared access signature.
    const SIGNED_KEY_NAME_TOKEN: &'static str = "skn";

    /// The token that identifies the signed expiration time of the shared access signature.
    const SIGNED_EXPIRY_TOKEN: &'static str = "se";

    /// The token that fully identifies the signed resource within the signature.
    // AuthenticationTypeToken + " " + SignedResourceToken;
    const SIGNED_RESOURCE_FULL_IDENTIFIER_TOKEN: &'static str = "SharedAccessSignature sr";

    /// The character used to separate a token and its value in the connection string.
    const TOKEN_VALUE_SEPARATOR: char = '=';

    /// The character used to mark the beginning of a new token/value pair in the signature.
    const TOKEN_VALUE_PAIR_DELIMITER: char = '&';

    /// The default length of time to consider a signature valid, if not otherwise specified.
    const DEFAULT_SIGNATURE_VALIDITY_DURATION: Duration = Duration::from_secs(30 * 60); // 30 mins
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

    /// Initializes a new instance of the [`SharedAccessSignature`] class.
    ///
    /// - `service_bus_resource` - The Service Bus resource to which the token is intended to serve as authorization.
    /// - `shared_access_key_name` - The name of the shared access key that the signature should be based on.
    /// - `shared_access_key` - The value of the shared access key for the signature.
    /// - `signature_validity_duration` - The duration that the signature should be considered valid; if not specified, a default will be assumed.
    pub fn try_from_parts(
        service_bus_resource: impl Into<String>,
        shared_access_key_name: impl Into<String>,
        shared_access_key: impl Into<String>,
        signature_validity_duration: Option<Duration>,
    ) -> Result<Self, SasSignatureError> {
        let signature_validity_duration =
            signature_validity_duration.unwrap_or(Self::DEFAULT_SIGNATURE_VALIDITY_DURATION);

        let now = OffsetDateTime::now_utc().replace_millisecond(0).unwrap(); // This won't fail
        let signature_expiration = now + signature_validity_duration;

        Self::try_new(
            service_bus_resource,
            shared_access_key_name,
            shared_access_key,
            signature_expiration,
        )
    }

    /// Initializes a new instance of the [`SharedAccessSignature`] class.
    ///
    /// - `shared_access_signature` - The shared access signature that will be parsed as the basis of this instance.
    pub fn try_from_signature(
        shared_access_signature: impl Into<String>,
    ) -> Result<Self, SasSignatureError> {
        // TODO: Optional or just empty string?
        Self::try_from_signature_and_key(shared_access_signature, "")
    }

    /// Initializes a new instance of the [`SharedAccessSignature`] class.
    ///
    /// - `shared_access_signature` - The shared access signature that will be parsed as the basis of this instance.
    /// - `shared_access_key` - The value of the shared access key for the signature.
    pub fn try_from_signature_and_key(
        shared_access_signature: impl Into<String>,
        shared_access_key: impl Into<String>,
    ) -> Result<Self, SasSignatureError> {
        let shared_access_signature = shared_access_signature.into();
        let shared_access_key = shared_access_key.into();

        if shared_access_key.len() > Self::MAXIMUM_KEY_LENGTH {
            return Err(SasSignatureError::SasKeyTooLong);
        }

        let parts = Self::parse_signature(&shared_access_signature)?;

        Ok(Self {
            shared_access_key_name: parts.key_name.into_owned(),
            shared_access_key,
            signature_expiration: parts.expiration_time,
            resource: parts.resource.into_owned(),
            value: shared_access_signature,
        })
    }

    /// Initializes a new instance of the [`SharedAccessSignature`] class.
    ///
    /// - `event_hub_resource` - The Service Bus resource to which the token is intended to serve as authorization.
    /// - `shared_access_key_name` - The name of the shared access key that the signature should be based on.
    /// - `shared_access_key` - The value of the shared access key for the signature.
    /// - `signature_expiration` - The date and time that the shared access signature expires, in UTC.
    pub fn try_new(
        resource: impl Into<String>,
        shared_access_key_name: impl Into<String>,
        shared_access_key: impl Into<String>,
        signature_expiration: OffsetDateTime,
    ) -> Result<Self, SasSignatureError> {
        let resource = resource.into();
        let shared_access_key_name = shared_access_key_name.into();
        let shared_access_key = shared_access_key.into();

        if resource.is_empty() {
            return Err(SasSignatureError::ArgumentIsEmpty);
        }
        if shared_access_key_name.is_empty() {
            return Err(SasSignatureError::ArgumentIsEmpty);
        }
        if shared_access_key.is_empty() {
            return Err(SasSignatureError::ArgumentIsEmpty);
        }

        if shared_access_key_name.len() > Self::MAXIMUM_KEY_NAME_LENGTH {
            return Err(SasSignatureError::SasKeyNameTooLong);
        }
        if shared_access_key.len() > Self::MAXIMUM_KEY_LENGTH {
            return Err(SasSignatureError::SasKeyTooLong);
        }

        let expiry = convert_to_unix_time(signature_expiration).to_string();

        let value = Self::build_signature(
            &resource,
            &shared_access_key_name,
            &shared_access_key,
            &expiry,
        )?;

        Ok(Self {
            shared_access_key_name,
            shared_access_key,
            signature_expiration,
            resource,
            value,
        })
    }

    // ///   Creates a new signature with the specified period for which the shared access signature is considered valid.
    // pub fn clone_with_new_expiration(
    //     &self,
    //     signature_validity_duration: Duration,
    // ) -> Result<Self, SasSignatureError> {
    //     if self.shared_access_key.is_empty() {
    //         return Err(SasSignatureError::SharedAccessKeyIsRequired);
    //     }

    //     Self::try_from_parts(
    //         &self.resource,
    //         &self.shared_access_key_name,
    //         &self.shared_access_key,
    //         Some(signature_validity_duration),
    //     )
    // }

    /// Creates a new signature with the specified period for which the shared access signature is considered valid.
    pub fn update_with_new_expiration(
        &mut self,
        signature_validity_duration: Duration,
    ) -> Result<(), SasSignatureError> {
        if self.shared_access_key.is_empty() {
            return Err(SasSignatureError::SharedAccessKeyIsRequired);
        }
        let signature_expiration = OffsetDateTime::now_utc() + signature_validity_duration;
        self.signature_expiration = signature_expiration;
        self.value = Self::build_signature(
            &self.resource,
            &self.shared_access_key_name,
            &self.shared_access_key,
            &convert_to_unix_time(signature_expiration).to_string(),
        )?;
        Ok(())
    }

    /// Parses a shared access signature into its component parts.
    pub(crate) fn parse_signature(
        shared_access_signature: &str,
    ) -> Result<SignatureParts, SasSignatureError> {
        let mut key_name = None;
        let mut resource = None;
        let mut expiration_time = DEFAULT_OFFSET_DATE_TIME;

        let token_value_pairs = shared_access_signature.split(Self::TOKEN_VALUE_PAIR_DELIMITER);
        for token_value_pair in token_value_pairs {
            let mut split = token_value_pair.split(Self::TOKEN_VALUE_SEPARATOR);
            let token = split
                .next()
                .ok_or(SasSignatureError::InvalidSharedAccessSignaure)?
                .trim();
            let value = split
                .next()
                .ok_or(SasSignatureError::InvalidSharedAccessSignaure)?
                .trim();

            if value.is_empty() {
                return Err(SasSignatureError::InvalidSharedAccessSignaure);
            }

            match token {
                Self::SIGNED_RESOURCE_FULL_IDENTIFIER_TOKEN => {
                    resource = Some(
                        urlencoding::decode(value)
                            .map_err(|_| SasSignatureError::InvalidSharedAccessSignaure)?,
                    );
                }
                Self::SIGNED_KEY_NAME_TOKEN => {
                    key_name = Some(
                        urlencoding::decode(value)
                            .map_err(|_| SasSignatureError::InvalidSharedAccessSignaure)?,
                    );
                }
                Self::SIGNED_EXPIRY_TOKEN => {
                    let value = urlencoding::decode(value)
                        .map_err(|_| SasSignatureError::InvalidSharedAccessSignaure)?;
                    let unix_time: i64 = value
                        .parse()
                        .map_err(|_| SasSignatureError::InvalidSharedAccessSignaure)?;
                    expiration_time = OffsetDateTime::from_unix_timestamp(unix_time)
                        .map_err(|_| SasSignatureError::InvalidSharedAccessSignaure)?;
                }
                _ => {}
            }
        }

        Ok(SignatureParts {
            key_name: key_name.ok_or(SasSignatureError::InvalidSharedAccessSignaure)?, // TODO: Optional or SasSignatureError?
            resource: resource.ok_or(SasSignatureError::InvalidSharedAccessSignaure)?,
            expiration_time,
        })
    }

    /// Builds the shared access signature value, which can be used as a token for
    /// access to the Service Bus service.
    ///
    /// - `audience` - The audience scope to which this signature applies.
    /// - `shared_access_key_name` - The name of the shared access key that the signature should be based on.
    /// - `shared_access_key` - The value of the shared access key for the signature.
    /// - `expiration_time` - The date/time, in UTC, that the signature expires.
    ///
    /// Returns the value of the shared access signature.
    fn build_signature(
        audience: &str,
        shared_access_key_name: &str,
        shared_access_key: &str,
        expiry: &str,
    ) -> Result<String, InvalidLength> {
        let encoded_audience: String =
            url::form_urlencoded::byte_serialize(audience.as_bytes()).collect();
        // let expiration = convert_to_unix_time(expiration_time).to_string();
        let message = format!("{encoded_audience}\n{expiry}");
        let mac = mac::<Hmac<Sha256>>(shared_access_key.as_bytes(), message.as_bytes())?;
        let signature = base64::encode(mac.as_ref());

        let encoded_signature = urlencoding::encode(&signature);
        let encoded_expiration = urlencoding::encode(expiry);
        let encoded_shared_access_key_name = urlencoding::encode(shared_access_key_name);

        let s = format!(
            "{} {}={}&{}={}&{}={}&{}={}",
            Self::AUTHENTICATION_TYPE_TOKEN,
            Self::SIGNATURE_TOKEN,
            encoded_signature,
            Self::SIGNED_EXPIRY_TOKEN,
            encoded_expiration,
            Self::SIGNED_KEY_NAME_TOKEN,
            encoded_shared_access_key_name,
            Self::SIGNED_RESOURCE_TOKEN,
            encoded_audience,
        );
        Ok(s)
    }
}

impl ToString for SharedAccessSignature {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

fn convert_to_unix_time(offset_date_time: OffsetDateTime) -> i64 {
    offset_date_time.unix_timestamp()
}

fn mac<M: Mac + KeyInit>(key: &[u8], input: &[u8]) -> Result<impl AsRef<[u8]>, InvalidLength> {
    let mut mac = <M as Mac>::new_from_slice(key)?;
    mac.update(input);
    Ok(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build_signature() {
        let built_signature = super::SharedAccessSignature::build_signature(
            "amqps://fe2o3-amqp-example.servicebus.windows.net",
            "RootManageSharedAccessKey",
            "r9rdIglXNiIPN2Lgj/HyhgOuq+aGht0qH3n+/lYQhfo=",
            "1667344375",
        )
        .unwrap();
        assert_eq!("SharedAccessSignature sig=WOVqJi%2B2fowHpCC2g3ztxEQrYAU173BGWrkVaPlvPj4%3D&se=1667344375&skn=RootManageSharedAccessKey&sr=amqps%3A%2F%2Ffe2o3-amqp-example.servicebus.windows.net", built_signature);
    }
}
