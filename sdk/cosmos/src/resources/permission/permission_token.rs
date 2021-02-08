use super::AuthorizationToken;
use crate::errors::TokenParsingError;

const PERMISSION_TYPE_PREFIX: &str = "type=";
const VERSION_PREFIX: &str = "ver=";
const SIGNATURE_PREFIX: &str = "sig=";

/// The token field of a [`Permission`](super::Permission) object.
///
/// This field is a url encoded string with the type of permission, the signature, and the version (currently only 1.0)
/// This type is a wrapper around AuthorizationToken.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(try_from = "String")]
pub struct PermissionToken {
    pub(crate) token: AuthorizationToken,
}

impl serde::Serialize for PermissionToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl std::fmt::Display for PermissionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::borrow::Cow;
        let (permission_type, signature) = match &self.token {
            AuthorizationToken::Resource(s) => ("resource", Cow::Borrowed(s)),
            AuthorizationToken::Primary(s) => ("master", Cow::Owned(base64::encode(s))),
        };
        write!(
            f,
            "{}{}&{}1.0&{}{}",
            PERMISSION_TYPE_PREFIX, permission_type, VERSION_PREFIX, SIGNATURE_PREFIX, signature
        )
    }
}

impl std::convert::TryFrom<String> for PermissionToken {
    type Error = TokenParsingError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::convert::TryFrom<&str> for PermissionToken {
    type Error = TokenParsingError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        trace!("converting {} into PermissionToken", s);

        let tokens: Vec<&str> = s.split('&').collect();

        if tokens.len() < 3 {
            return Err(TokenParsingError::InsufficientTokens {
                s: s.to_owned(),
                required: 3,
                found: tokens.len() as u32,
            }
            .into());
        }
        let version = get_item(s, &tokens, VERSION_PREFIX)?;
        if version != "1.0" && version != "1" {
            return Err(TokenParsingError::UnrecognizedVersionNumber {
                provided_version: version.to_owned(),
            });
        }

        let permission_type = get_item(s, &tokens, PERMISSION_TYPE_PREFIX)?;
        let signature = get_item(s, &tokens, SIGNATURE_PREFIX)?.to_owned();
        let token = match permission_type {
            "master" => AuthorizationToken::Primary(base64::decode(signature)?),
            "resource" => AuthorizationToken::Resource(signature),
            _ => {
                return Err(TokenParsingError::UnrecognizedPermissionType {
                    provided_type: permission_type.to_owned(),
                })
            }
        };
        Ok(Self { token })
    }
}

fn get_item<'a>(s: &'a str, tokens: &[&'a str], token: &str) -> Result<&'a str, TokenParsingError> {
    let mut tokens = tokens.iter().filter(|t| t.starts_with(token));

    match tokens.next() {
        Some(t) if tokens.next().is_some() => Err(TokenParsingError::ReplicatedToken {
            s: s.to_owned(),
            token: token.to_owned(),
            occurrences: 2 + tokens.count() as u32,
        }),
        Some(t) => {
            // we checked for < 1 and > 1 so this is == 1
            Ok(&t[token.len()..])
        }
        None => Err(TokenParsingError::MissingToken {
            s: s.to_owned(),
            missing_token: token.to_owned(),
        }),
    }
}

impl std::convert::From<AuthorizationToken> for PermissionToken {
    fn from(token: AuthorizationToken) -> Self {
        Self { token }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    const PERMISSION: &str = r#"type=resource&ver=1&sig=m32/00W65F8ADb3psljJ0g==;v0kQGihedau1pVGGQmuPgzlEcfsYDWSdfn2kyjDc1qF1aZfPHXzIS/BFMcuZQRUr6C5c5PgiyCSwhiAgZMJne2DorfMbE/GUHmxBLjOnykLARqwn3zpZpz9b2axWtL8+qQFX81nocdEDvBVzFuobyul6QimbmeZ7D6D1K4qJT9feuJkIBfczeAp/sKaSupXEgB3qyih0rej5N6Wv14Gufohh1QTlCRIzK3FqQv4xjcY={"#;

    #[test]
    fn parse_permission_token() {
        let permission_token: PermissionToken = PERMISSION.try_into().unwrap();
        assert!(matches!(
            permission_token.token,
            AuthorizationToken::Resource(_)
        ));
    }
}
