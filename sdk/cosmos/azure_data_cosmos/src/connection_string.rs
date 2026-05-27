// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::str::FromStr;

use azure_core::{credentials::Secret, fmt::SafeDebug};

/// Represents a Cosmos DB connection string.
#[derive(Clone, PartialEq, Eq, SafeDebug)]
pub struct ConnectionString {
    pub account_endpoint: String,
    pub account_key: Secret,
}

impl TryFrom<&Secret> for ConnectionString {
    type Error = crate::CosmosError;
    fn try_from(secret: &Secret) -> Result<Self, Self::Error> {
        secret.secret().parse()
    }
}

impl FromStr for ConnectionString {
    type Err = crate::CosmosError;
    fn from_str(connection_string: &str) -> Result<Self, Self::Err> {
        if connection_string.is_empty() {
            return Err(crate::DriverCosmosError::builder()
                .with_status(crate::CosmosStatus::CLIENT_CONNECTION_STRING_EMPTY)
                .with_message("connection string cannot be empty")
                .build()
                .into());
        }

        let splat = connection_string.split(';');
        let mut account_endpoint = None;
        let mut account_key = None;
        for part in splat {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            let (key, value) = part.split_once('=').ok_or_else(|| {
                crate::DriverCosmosError::builder()
                    .with_status(crate::CosmosStatus::CLIENT_CONNECTION_STRING_MALFORMED_PART)
                    .with_message("invalid connection string")
                    .build()
            })?;

            if key.eq_ignore_ascii_case("AccountEndpoint") {
                account_endpoint = Some(value.to_string())
            }

            if key.eq_ignore_ascii_case("AccountKey") {
                account_key = Some(Secret::new(value.to_string()))
            }
        }

        let Some(endpoint) = account_endpoint else {
            return Err(crate::DriverCosmosError::builder()
                .with_status(crate::CosmosStatus::CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_ENDPOINT)
                .with_message("invalid connection string, missing 'AccountEndpoint'")
                .build()
                .into());
        };

        let Some(key) = account_key else {
            return Err(crate::DriverCosmosError::builder()
                .with_status(crate::CosmosStatus::CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_KEY)
                .with_message("invalid connection string, missing 'AccountKey'")
                .build()
                .into());
        };

        Ok(Self {
            account_endpoint: endpoint,
            account_key: key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionString;
    use azure_core::credentials::Secret;

    #[test]
    pub fn test_valid_connection_string() {
        let connection_string =
            "AccountEndpoint=https://accountname.documents.azure.com:443/;AccountKey=key";
        let secret = Secret::new(connection_string);
        let connection_str = ConnectionString::try_from(&secret).unwrap();

        assert_eq!(
            "https://accountname.documents.azure.com:443/",
            connection_str.account_endpoint
        );

        assert_eq!("key", connection_str.account_key.secret());
    }

    #[test]
    pub fn test_valid_connection_string_mismatched_case() {
        let connection_string =
            "accountendpoint=https://accountname.documents.azure.com:443/;accountkey=key";
        let secret = Secret::new(connection_string);
        let connection_str = ConnectionString::try_from(&secret).unwrap();

        // should pass, we don't expect connection string keys to be case sensitive
        assert_eq!(
            "https://accountname.documents.azure.com:443/",
            connection_str.account_endpoint
        );

        assert_eq!("key", connection_str.account_key.secret());
    }

    #[test]
    pub fn test_empty_connection_string() {
        test_bad_connection_string(
            "",
            "400/20104 (ClientConnectionStringEmpty)",
            "connection string cannot be empty",
        )
    }

    #[test]
    pub fn test_malformed_connection_string() {
        test_bad_connection_string(
            "AccountEndpointhttps://accountname.documents.azure.com:443AccountKeyaccountkey",
            "400/20105 (ClientConnectionStringMalformedPart)",
            "invalid connection string",
        );
    }

    #[test]
    pub fn test_partially_malformed_connection_string() {
        test_bad_connection_string(
            "AccountEndpointhttps://accountname.documents.azure.com:443/AccountKey=accountkey",
            "400/20106 (ClientConnectionStringMissingAccountEndpoint)",
            "invalid connection string, missing 'AccountEndpoint'",
        );
    }

    #[test]
    pub fn test_connection_string_missing_account_endpoint() {
        test_bad_connection_string(
            "AccountKey=key",
            "400/20106 (ClientConnectionStringMissingAccountEndpoint)",
            "invalid connection string, missing 'AccountEndpoint'",
        );
    }

    #[test]
    pub fn test_connection_string_missing_account_key() {
        test_bad_connection_string(
            "AccountEndpoint=https://accountname.documents.azure.com:443/;",
            "400/20107 (ClientConnectionStringMissingAccountKey)",
            "invalid connection string, missing 'AccountKey'",
        );
    }

    fn test_bad_connection_string(
        connection_string: &str,
        expected_status: &str,
        expected_error_message: &str,
    ) {
        let secret = Secret::new(connection_string.to_owned());
        let connection_str = ConnectionString::try_from(&secret);
        let err = connection_str.unwrap_err();
        let actual_error_message = err.to_string();
        assert_eq!(
            actual_error_message,
            format!("{expected_status}: {expected_error_message}")
        )
    }
}
