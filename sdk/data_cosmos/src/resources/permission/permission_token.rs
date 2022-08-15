use super::AuthorizationToken;

const PERMISSION_TYPE_PREFIX: &str = "type=";
const VERSION_PREFIX: &str = "ver=";
const SIGNATURE_PREFIX: &str = "sig=";

/// The token field of a [`Permission`](super::Permission) object.
///
/// This field is a url encoded string with the type of permission, the signature, and the version (currently only 1.0)
/// This type is a wrapper around AuthorizationToken.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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
    type Error = PermissionTokenParseError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

const MIN_REQUIRED_PARTS_COUNT: usize = 3;

impl std::convert::TryFrom<&str> for PermissionToken {
    type Error = PermissionTokenParseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        trace!("converting {} into PermissionToken", s);

        let parts: Vec<&str> = s.split('&').collect();

        if parts.len() < MIN_REQUIRED_PARTS_COUNT {
            return Err(PermissionTokenParseError::InsufficientParts {
                token_string: s.to_owned(),
                found: parts.len() as u32,
            });
        }
        let version = try_get_item(s, &parts, VERSION_PREFIX)?;
        if version != "1.0" && version != "1" {
            return Err(PermissionTokenParseError::UnrecognizedVersionNumber {
                provided_version: version.to_owned(),
            });
        }

        let permission_type = try_get_item(s, &parts, PERMISSION_TYPE_PREFIX)?;
        let signature = try_get_item(s, &parts, SIGNATURE_PREFIX)?.to_owned();
        let token = match permission_type {
            "master" => AuthorizationToken::Primary(base64::decode(signature)?),
            "resource" => AuthorizationToken::Resource(signature),
            _ => {
                return Err(PermissionTokenParseError::UnrecognizedPermissionType {
                    provided_type: permission_type.to_owned(),
                })
            }
        };
        Ok(Self { token })
    }
}

fn try_get_item<'a>(
    token_string: &'a str,
    parts: &[&'a str],
    name: &str,
) -> Result<&'a str, PermissionTokenParseError> {
    let mut tokens = parts.iter().filter(|t| t.starts_with(name));

    match tokens.next() {
        Some(_t) if tokens.next().is_some() => Err(PermissionTokenParseError::DuplicatePart {
            token_string: token_string.to_owned(),
            part: name.to_owned(),
            // Add 2 since we've already called `next` twice
            occurrences: 2 + tokens.count() as u32,
        }),
        Some(t) => {
            // we checked for < 1 and > 1 so this is == 1
            Ok(&t[name.len()..])
        }
        None => Err(PermissionTokenParseError::MissingPart {
            token_string: token_string.to_owned(),
            missing_part: name.to_owned(),
        }),
    }
}

impl std::convert::From<AuthorizationToken> for PermissionToken {
    fn from(token: AuthorizationToken) -> Self {
        Self { token }
    }
}

#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum PermissionTokenParseError {
    #[error(
        "Permission token string has an insufficient number of ';' separated parts. Required number is {}, but found {}. Full string: \"{}\"",
        MIN_REQUIRED_PARTS_COUNT,
        found,
        token_string
    )]
    InsufficientParts { token_string: String, found: u32 },
    #[error(
        "A required part of permission token is missing: {}. Full string: \"{}\"",
        missing_part,
        token_string
    )]
    MissingPart {
        token_string: String,
        missing_part: String,
    },
    #[error(
        "Duplicate part found in permission token. Part: {}. Occurrences: {}. Full string: \"{}\"",
        part,
        occurrences,
        token_string
    )]
    DuplicatePart {
        token_string: String,
        part: String,
        occurrences: u32,
    },
    #[error(
        "Unrecognized version number provided in permission token: {}",
        provided_version
    )]
    UnrecognizedVersionNumber { provided_version: String },
    #[error(
        "Unrecognized permission type provided in permission token: {}",
        provided_type
    )]
    UnrecognizedPermissionType { provided_type: String },
    #[error("the authorization token was not properly base64 encoded: {0}")]
    InvalidBase64Encoding(#[from] base64::DecodeError),
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
