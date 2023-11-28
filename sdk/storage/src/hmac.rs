use azure_core::{
    auth::Secret,
    base64,
    error::{ErrorKind, ResultExt},
};

#[cfg(not(feature = "enable_openssl_sign"))]
pub fn sign(data: &str, key: &Secret) -> azure_core::Result<String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let mut hmac = Hmac::<Sha256>::new_from_slice(&base64::decode(key.secret())?)
        .with_context(ErrorKind::DataConversion, || {
            "failed to create hmac from key".to_string()
        })?;
    hmac.update(data.as_bytes());
    let signature = hmac.finalize().into_bytes();
    Ok(base64::encode(signature))
}

#[cfg(feature = "enable_openssl_sign")]
pub fn sign(data: &str, key: &Secret) -> azure_core::Result<String> {
    use openssl::{error::ErrorStack, hash::MessageDigest, pkey::PKey, sign::Signer};
    let dkey = base64::decode(key.secret())?;
    let signature = || -> Result<Vec<u8>, ErrorStack> {
        let pkey = PKey::hmac(&dkey)?;
        let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
        signer.update(data.as_bytes())?;
        Ok(signer.sign_to_vec()?)
    }()
    .with_context(ErrorKind::DataConversion, || {
        "failed to create hmac from key".to_string()
    })?;
    Ok(base64::encode(signature))
}

#[cfg(test)]
mod tests {
    use azure_core::auth::Secret;

    #[test]
    fn test_hmac_sign() {
        let data = "create hmac signature for data";
        let key = Secret::new("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");

        let sig = super::sign(data, &key).unwrap();

        let expected_sig = "D/y9XyIEdUzEbdV570h8dou/mfkbMA1lKCOPqPDPAd0=";
        assert_eq!(sig, expected_sig);
    }
}
