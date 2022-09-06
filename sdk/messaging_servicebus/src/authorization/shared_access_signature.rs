use std::time::Duration;

use const_format::formatcp;
use digest::{Digest, InvalidLength, KeyInit, Mac};
use hmac::Hmac;
use sha2::Sha256;
use time::OffsetDateTime;

pub(crate) struct SharedAccessSignature {
    shared_access_key_name: String,
    shared_access_key: String,
    signature_expiration: OffsetDateTime,
    resource: String,
    value: String,
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

        let encoded_signature: String =
            url::form_urlencoded::byte_serialize(signature.as_bytes()).collect();
        let encoded_expiration: String =
            url::form_urlencoded::byte_serialize(expiration.as_bytes()).collect();
        let encoded_shared_access_key_name: String =
            url::form_urlencoded::byte_serialize(shared_access_key_name.as_bytes()).collect();

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
