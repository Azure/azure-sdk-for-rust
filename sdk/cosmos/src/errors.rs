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
    InsufficientTokens {
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

#[derive(Debug, Fail)]
pub enum ConversionToDocumentError {
    #[fail(display = "Conversion to document failed because at lease one element is raw.")]
    RawElementFound {},
}

pub(crate) fn item_or_error<'a>(
    s: &'a str,
    tokens: &[&'a str],
    token: &'a str,
) -> Result<&'a str, TokenParsingError> {
    let mut tokens = tokens.iter().filter(|t| t.starts_with(token));

    match tokens.next() {
        Some(t) => {
            if tokens.next().is_some() {
                return Err(TokenParsingError::ReplicatedToken {
                    s: s.to_owned(),
                    token: token.to_owned(),
                    occurrencies: 2 + tokens.count() as u32,
                });
            }
            // we checked for < 1 and > 1 so this is == 1
            Ok(&t[token.len()..])
        }
        None => Err(TokenParsingError::MissingToken {
            s: s.to_owned(),
            missing_token: token.to_owned(),
        }),
    }
}
