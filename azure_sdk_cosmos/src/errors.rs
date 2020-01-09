#[derive(Debug, Fail)]
pub enum PermissionTokenParsingError {
    #[fail(
        display = "string has unsufficient number of tokens. Required {}, found {}. String: \"{}\"",
        s, required, found
    )]
    UnsufficientTokens {
        s: String,
        required: u32,
        found: u32,
    },
    #[fail(
        display = "A required token is missing. Required: {}. String: \"{}\"",
        s, missing_token
    )]
    MissingToken { s: String, missing_token: String },

    #[fail(
        display = "Replicated token found. Token: {}. Occurrencies: {}. String: \"{}\"",
        s, token, occurrencies
    )]
    ReplicatedToken {
        s: String,
        token: String,
        occurrencies: u32,
    },
}
