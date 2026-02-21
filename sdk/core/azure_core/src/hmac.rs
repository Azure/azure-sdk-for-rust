// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HMAC encoding and decoding functions.

#[cfg(any(feature = "hmac_rust", feature = "hmac_openssl"))]
use crate::error::{ErrorKind, ResultExt};
use crate::{base64, credentials::Secret};

/// Tries to create an HMAC SHA256 signature from the given `data` and `key`.
///
/// The `key` is expected to be a base64 encoded string and will be decoded
/// before using it for signing. The returned signature is also base64 encoded.
///
/// If both `hmac_rust` and `hmac_openssl` are enabled, use `hmac_openssl`.
///
/// # Errors
///
/// - If the `key` is not a valid base64 encoded string.
/// - If it fails to create the HMAC from the `key`.
/// - If `hmac_rust` and/or `hmac_openssl` are not enabled.
pub fn hmac_sha256(data: &str, key: &Secret) -> crate::Result<String> {
    let key = base64::decode(key.secret())?;
    hmac_sha256_bytes(data.as_bytes(), &key)
}

/// Tries to create an HMAC SHA256 signature from the given `data` and `key`.
///
/// The returned signature is also base64 encoded.
///
/// If both `hmac_rust` and `hmac_openssl` are enabled, use `hmac_openssl`.
///
/// # Errors
///
/// - If the `key` is not a valid base64 encoded string.
/// - If it fails to create the HMAC from the `key`.
#[cfg(all(feature = "hmac_rust", not(feature = "hmac_openssl")))]
pub fn hmac_sha256_bytes(data: &[u8], key: &[u8]) -> crate::Result<String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let mut hmac = Hmac::<Sha256>::new_from_slice(key)
        .with_context_fn(ErrorKind::DataConversion, || {
            "failed to create hmac from key"
        })?;
    hmac.update(data);
    let signature = hmac.finalize().into_bytes();
    Ok(base64::encode(signature))
}

/// Tries to create an HMAC SHA256 signature from the given `data` and `key`.
///
/// The returned signature is also base64 encoded.
///
/// If both `hmac_rust` and `hmac_openssl` are enabled, use `hmac_openssl`.
///
/// # Errors
///
/// - If the `key` is not a valid base64 encoded string.
/// - If it fails to create the HMAC from the `key`.
#[cfg(feature = "hmac_openssl")]
pub fn hmac_sha256_bytes(data: &[u8], key: &[u8]) -> crate::Result<String> {
    // cspell:ignore pkey
    use openssl::{error::ErrorStack, hash::MessageDigest, pkey::PKey, sign::Signer};

    let signature = || -> Result<Vec<u8>, ErrorStack> {
        let pkey = PKey::hmac(key)?;
        let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
        signer.update(data)?;
        signer.sign_to_vec()
    }()
    .with_context_fn(ErrorKind::DataConversion, || {
        "failed to create hmac from key"
    })?;
    Ok(base64::encode(signature))
}

/// Tries to create an HMAC SHA256 signature from the given `data` and `key`.
///
/// # Errors
///
/// This implementation always returns an error. Enable `hmac_rust` and/or `hmac_openssl`.
#[cfg(not(any(feature = "hmac_rust", feature = "hmac_openssl")))]
pub fn hmac_sha256_bytes(_data: &[u8], _key: &[u8]) -> crate::Result<String> {
    unimplemented!("An HMAC signing request was called without an hmac implementation. Make sure to enable either the `hmac_rust` or `hmac_openssl` feature");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[cfg_attr(any(feature = "hmac_rust", feature = "hmac_openssl"), test)]
    fn test_hmac_sign() {
        let data = "create hmac signature for data";
        let key = Secret::new("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");

        let sig = hmac_sha256(data, &key).unwrap();

        let expected_sig = "D/y9XyIEdUzEbdV570h8dou/mfkbMA1lKCOPqPDPAd0=";
        assert_eq!(sig, expected_sig);
    }
}
