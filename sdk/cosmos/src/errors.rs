use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenParsingError {
    #[error(
        "string has an unsupported starting token. Token: \"{}\", String: \"{}\"",
        token,
        s
    )]
    UnsupportedToken { token: String, s: String },
    #[error(
        "string has unsufficient number of tokens. Required {}, found {}. String: \"{}\"",
        required,
        found,
        s
    )]
    InsufficientTokens {
        s: String,
        required: u32,
        found: u32,
    },
    #[error(
        "A required token is missing. Required: {}. String: \"{}\"",
        missing_token,
        s
    )]
    MissingToken { s: String, missing_token: String },
    #[error(
        "Replicated token found. Token: {}. Occurrences: {}. String: \"{}\"",
        token,
        occurrences,
        s
    )]
    ReplicatedToken {
        s: String,
        token: String,
        occurrences: u32,
    },
    #[error("Could not parse number: {}", error)]
    NumberParseError {
        #[from]
        error: std::num::ParseIntError,
    },
    #[error("Wrong version provided: {}", provided_version)]
    UnrecognizedVersionNumber { provided_version: String },
    #[error("Wrong version provided: {}", provided_type)]
    UnrecognizedPermissionType { provided_type: String },
    #[error("The value was not properly base64 encoded: {}", error)]
    WronglyEncodedValue {
        #[from]
        error: base64::DecodeError,
    },
}

#[derive(Debug, Error)]
pub enum ConversionToDocumentError {
    #[error("Conversion to document failed because at lease one element is raw.")]
    RawElementFound {},
}
