use azure_core::error::ErrorKind;
use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use time::{macros::format_description, OffsetDateTime};

pub(crate) fn format_sas_time(dt: OffsetDateTime) -> Result<String, time::error::Format> {
    dt.format(format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second]Z"
    ))
}

/// Signs `string_to_sign` with HMAC-SHA256 using the base64-encoded `key_value`.
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-signature>
pub(crate) fn compute_hmac_signature(
    key_value: &str,
    string_to_sign: &str,
) -> azure_core::Result<String> {
    let key_bytes = STANDARD.decode(key_value)?;
    let mut mac = Hmac::<Sha256>::new_from_slice(&key_bytes).map_err(|_| {
        azure_core::Error::with_message(
            ErrorKind::Other,
            "HMAC key initialization failed: invalid key bytes",
        )
    })?;
    mac.update(string_to_sign.as_bytes());
    Ok(STANDARD.encode(mac.finalize().into_bytes()))
}
