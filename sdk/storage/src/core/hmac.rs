use crate::Result;
use base64::encode;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn sign(data: &str, key: &str) -> Result<String> {
    let mut hmac = Hmac::<Sha256>::new_from_slice(&base64::decode(key)?)?;
    hmac.update(data.as_bytes());
    let signature = hmac.finalize().into_bytes();
    Ok(encode(&signature))
}
