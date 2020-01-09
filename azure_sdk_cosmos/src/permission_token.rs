use crate::errors::PermissionTokenParsingError;

const PERMISSION_TYPE_PREFIX: &str = "type=";
const VERSION_PREFIX: &str = "ver=";
const SIGNATURE_PREFIX: &str = "sig=";

#[derive(Debug, Clone, PartialEq)]
pub struct PermissionToken {
    pub permission_type: String,
    pub version: String,
    pub signature: String,
}

impl std::fmt::Display for PermissionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}&{}{}&{}{}",
            PERMISSION_TYPE_PREFIX,
            &self.permission_type,
            VERSION_PREFIX,
            &self.version,
            SIGNATURE_PREFIX,
            &self.signature
        )
    }
}

impl std::convert::TryFrom<&str> for PermissionToken {
    type Error = failure::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let tokens: Vec<&str> = s.split("&").collect();

        if tokens.len() < 3 {
            return Err(PermissionTokenParsingError::UnsufficientTokens {
                s: s.to_owned(),
                required: 3,
                found: tokens.len() as u32,
            }
            .into());
        }

        Ok(Self {
            permission_type: item_or_error(s, &tokens, PERMISSION_TYPE_PREFIX)?.to_owned(),
            version: item_or_error(s, &tokens, VERSION_PREFIX)?.to_owned(),
            signature: item_or_error(s, &tokens, SIGNATURE_PREFIX)?.to_owned(),
        })
    }
}

#[inline]
fn item_or_error<'a>(
    s: &'a str,
    tokens: &[&'a str],
    token: &'a str,
) -> Result<&'a str, PermissionTokenParsingError> {
    let tokens = tokens
        .iter()
        .filter(|t| t.starts_with(token))
        .collect::<Vec<_>>();

    if tokens.len() < 1 {
        return Err(PermissionTokenParsingError::MissingToken {
            s: s.to_owned(),
            missing_token: token.to_owned(),
        });
    }

    if tokens.len() > 1 {
        return Err(PermissionTokenParsingError::ReplicatedToken {
            s: s.to_owned(),
            token: token.to_owned(),
            occurrencies: tokens.len() as u32,
        });
    }

    // we checked for < 1 and > 1 so this is == 1
    // Unwrap is safe.
    Ok(&tokens.first().unwrap()[token.len()..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    const PERMISSION: &str = r#"type=resource&ver=1&sig=m32/00W65F8ADb3psljJ0g==;v0kQGihedau1pVGGQmuPgzlEcfsYDWSdfn2kyjDc1qF1aZfPHXzIS/BFMcuZQRUr6C5c5PgiyCSwhiAgZMJne2DorfMbE/GUHmxBLjOnykLARqwn3zpZpz9b2axWtL8+qQFX81nocdEDvBVzFuobyul6QimbmeZ7D6D1K4qJT9feuJkIBfczeAp/sKaSupXEgB3qyih0rej5N6Wv14Gufohh1QTlCRIzK3FqQv4xjcY={"#;

    #[test]
    fn parse_permission_token() {
        let permission_token: PermissionToken = PERMISSION.try_into().unwrap();
        assert_eq!(permission_token.version, "1");
        assert_eq!(permission_token.permission_type, "resource");
    }
}
