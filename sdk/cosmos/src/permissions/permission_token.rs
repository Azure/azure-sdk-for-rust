use crate::errors::{self, TokenParsingError};
use crate::AuthorizationToken;

const PERMISSION_TYPE_PREFIX: &str = "type=";
const VERSION_PREFIX: &str = "ver=";
const SIGNATURE_PREFIX: &str = "sig=";

/// The token field of a [`Permissions`](crate::Permissions) object.
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
    type Error = failure::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::convert::TryFrom<&str> for PermissionToken {
    type Error = failure::Error;
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
        let version = errors::item_or_error(s, &tokens, VERSION_PREFIX)?;
        if version != "1.0" && version != "1" {
            return Err(failure::format_err!(
                "unrecognized version number: {}",
                version
            ));
        }

        let permission_type = errors::item_or_error(s, &tokens, PERMISSION_TYPE_PREFIX)?;
        let signature = errors::item_or_error(s, &tokens, SIGNATURE_PREFIX)?.to_owned();
        let token = match permission_type {
            "master" => AuthorizationToken::Primary(base64::decode(signature)?),
            "resource" => AuthorizationToken::Resource(signature),
            _ => {
                return Err(failure::format_err!(
                    "Unrecognized error permission type {}",
                    permission_type
                ))
            }
        };
        Ok(Self { token })
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
