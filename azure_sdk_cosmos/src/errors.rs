#[derive(Debug, Fail)]
pub enum TokenParsingError {
    #[fail(
        display = "string has an unsupported starting token. Token: \"{}\", String: \"{}\"",
        token, s
    )]
    UnsupportedToken { token: String, s: String },

    #[fail(
        display = "string has unsufficient number of tokens. Required {}, found {}. String: \"{}\"",
        required, found, s
    )]
    UnsufficientTokens {
        s: String,
        required: u32,
        found: u32,
    },
    #[fail(
        display = "A required token is missing. Required: {}. String: \"{}\"",
        missing_token, s
    )]
    MissingToken { s: String, missing_token: String },

    #[fail(
        display = "Replicated token found. Token: {}. Occurrencies: {}. String: \"{}\"",
        token, occurrencies, s
    )]
    ReplicatedToken {
        s: String,
        token: String,
        occurrencies: u32,
    },
}

#[inline]
pub(crate) fn item_or_error<'a>(
    s: &'a str,
    tokens: &[&'a str],
    token: &'a str,
) -> Result<&'a str, TokenParsingError> {
    let tokens = tokens
        .iter()
        .filter(|t| t.starts_with(token))
        .collect::<Vec<_>>();

    if tokens.is_empty() {
        return Err(TokenParsingError::MissingToken {
            s: s.to_owned(),
            missing_token: token.to_owned(),
        });
    }

    if tokens.len() > 1 {
        return Err(TokenParsingError::ReplicatedToken {
            s: s.to_owned(),
            token: token.to_owned(),
            occurrencies: tokens.len() as u32,
        });
    }

    // we checked for < 1 and > 1 so this is == 1
    // Unwrap is safe.
    Ok(&tokens.first().unwrap()[token.len()..])
}
