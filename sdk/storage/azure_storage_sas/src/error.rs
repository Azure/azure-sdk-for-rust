#[derive(Debug, thiserror::Error)]
pub enum SasError {
    #[error("failed to acquire storage token: {0}")]
    TokenError(#[from] azure_core::Error),

    #[error("user delegation key request failed with HTTP {status}: {message}")]
    DelegationKeyError { status: u16, message: String },

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("XML parse error: {0}")]
    XmlError(#[from] quick_xml::DeError),

    #[error("base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("HMAC key initialization failed: invalid key bytes")]
    HmacError,

    #[error("time format error: {0}")]
    TimeError(#[from] time::error::Format),

    #[error("URL parse error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error(
        "key_expiry_duration exceeds the Azure-enforced maximum; see UserDelegationSasBuilder::MAX_KEY_EXPIRY"
    )]
    KeyExpiryTooLong,
}
