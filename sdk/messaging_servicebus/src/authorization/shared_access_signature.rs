use std::time::Duration;

use const_format::formatcp;
use time::OffsetDateTime;

pub(crate) struct SharedAccessSignature {}

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
