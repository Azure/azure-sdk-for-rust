use crate::auth::Secret;
#[cfg(any(feature = "hmac_rust", feature = "hmac_openssl"))]
use crate::{
    base64,
    error::{ErrorKind, ResultExt},
};

// if both hmac_rust and hmac_openssl are enabled, use hmac_openssl
#[cfg(all(feature = "hmac_rust", not(feature = "hmac_openssl")))]
pub fn hmac_sha256(data: &str, key: &Secret) -> crate::Result<String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let key = base64::decode(key.secret())?;
    let mut hmac = Hmac::<Sha256>::new_from_slice(&key)
        .with_context(ErrorKind::DataConversion, || {
            "failed to create hmac from key"
        })?;
    hmac.update(data.as_bytes());
    let signature = hmac.finalize().into_bytes();
    Ok(base64::encode(signature))
}

#[cfg(feature = "hmac_openssl")]
pub fn hmac_sha256(data: &str, key: &Secret) -> crate::Result<String> {
    use openssl::{error::ErrorStack, hash::MessageDigest, pkey::PKey, sign::Signer};

    let dkey = base64::decode(key.secret())?;
    let signature = || -> Result<Vec<u8>, ErrorStack> {
        let pkey = PKey::hmac(&dkey)?;
        let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
        signer.update(data.as_bytes())?;
        Ok(signer.sign_to_vec()?)
    }()
    .with_context(ErrorKind::DataConversion, || {
        "failed to create hmac from key"
    })?;
    Ok(base64::encode(signature))
}

#[cfg(not(any(feature = "hmac_rust", feature = "hmac_openssl")))]
pub fn hmac_sha256(_data: &str, _key: &Secret) -> crate::Result<String> {
    unimplemented!("An HMAC signing request was called without an hmac implementation.  Make sure to enable either the `hmac_rust` or `hmac_openssl` feature");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_sign() {
        let data = "create hmac signature for data";
        let key = Secret::new("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");

        let sig = hmac_sha256(data, &key).unwrap();

        let expected_sig = "D/y9XyIEdUzEbdV570h8dou/mfkbMA1lKCOPqPDPAd0=";
        assert_eq!(sig, expected_sig);
    }
}
