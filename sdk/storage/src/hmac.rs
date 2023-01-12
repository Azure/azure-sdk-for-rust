use azure_core::error::{ErrorKind, ResultExt};
use base64::{prelude::BASE64_STANDARD, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn sign(data: &str, key: &str) -> azure_core::Result<String> {
    let mut hmac = Hmac::<Sha256>::new_from_slice(
        &BASE64_STANDARD
            .decode(key)
            .with_context(ErrorKind::DataConversion, || {
                format!("failed to decode hmac. key: {key}")
            })?,
    )
    .with_context(ErrorKind::DataConversion, || {
        format!("failed to create hmac from key: {key}")
    })?;
    hmac.update(data.as_bytes());
    let signature = hmac.finalize().into_bytes();
    Ok(BASE64_STANDARD.encode(signature))
}
