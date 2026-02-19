// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Connection string parsing for Cosmos DB.

use std::str::FromStr;

use azure_core::{credentials::Secret, fmt::SafeDebug, Error};

/// Represents a Cosmos DB connection string.
///
/// A connection string contains the account endpoint and account key required
/// to authenticate with a Cosmos DB account using master key authentication.
///
/// # Format
///
/// The expected format is:
/// ```text
/// AccountEndpoint=https://<account>.documents.azure.com:443/;AccountKey=<key>
/// ```
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::models::ConnectionString;
///
/// let conn_str: ConnectionString = "AccountEndpoint=https://myaccount.documents.azure.com:443/;AccountKey=mykey".parse().unwrap();
/// assert_eq!(conn_str.account_endpoint(), "https://myaccount.documents.azure.com:443/");
/// ```
#[derive(Clone, PartialEq, Eq, SafeDebug)]
#[non_exhaustive]
pub struct ConnectionString {
    account_endpoint: String,
    account_key: Secret,
}

impl ConnectionString {
    /// Returns the account endpoint URL.
    pub fn account_endpoint(&self) -> &str {
        &self.account_endpoint
    }

    /// Returns the account key as a secret.
    pub fn account_key(&self) -> &Secret {
        &self.account_key
    }
}

impl TryFrom<&Secret> for ConnectionString {
    type Error = azure_core::Error;

    fn try_from(secret: &Secret) -> Result<Self, Self::Error> {
        secret.secret().parse()
    }
}

impl FromStr for ConnectionString {
    type Err = azure_core::Error;

    fn from_str(connection_string: &str) -> Result<Self, Self::Err> {
        if connection_string.is_empty() {
            return Err(Error::new(
                azure_core::error::ErrorKind::Other,
                "connection string cannot be empty",
            ));
        }

        let splat = connection_string.split(';');
        let mut account_endpoint = None;
        let mut account_key = None;

        for part in splat {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            let (key, value) = part.split_once('=').ok_or(Error::new(
                azure_core::error::ErrorKind::Other,
                "invalid connection string",
            ))?;

            if key.eq_ignore_ascii_case("AccountEndpoint") {
                account_endpoint = Some(value.to_string())
            }

            if key.eq_ignore_ascii_case("AccountKey") {
                account_key = Some(Secret::new(value.to_string()))
            }
        }

        let Some(endpoint) = account_endpoint else {
            return Err(Error::new(
                azure_core::error::ErrorKind::Other,
                "invalid connection string, missing 'AccountEndpoint'",
            ));
        };

        let Some(key) = account_key else {
            return Err(Error::new(
                azure_core::error::ErrorKind::Other,
                "invalid connection string, missing 'AccountKey'",
            ));
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
    fn valid_connection_string() {
        let connection_string =
            "AccountEndpoint=https://accountname.documents.azure.com:443/;AccountKey=key";
        let secret = Secret::new(connection_string);
        let connection_str = ConnectionString::try_from(&secret).unwrap();

        assert_eq!(
            "https://accountname.documents.azure.com:443/",
            connection_str.account_endpoint()
        );

        assert_eq!("key", connection_str.account_key().secret());
    }

    #[test]
    fn valid_connection_string_mismatched_case() {
        let connection_string =
            "accountendpoint=https://accountname.documents.azure.com:443/;accountkey=key";
        let secret = Secret::new(connection_string);
        let connection_str = ConnectionString::try_from(&secret).unwrap();

        // should pass, we don't expect connection string keys to be case sensitive
        assert_eq!(
            "https://accountname.documents.azure.com:443/",
            connection_str.account_endpoint()
        );

        assert_eq!("key", connection_str.account_key().secret());
    }

    #[test]
    fn empty_connection_string() {
        test_bad_connection_string("", "connection string cannot be empty")
    }

    #[test]
    fn malformed_connection_string() {
        test_bad_connection_string(
            "AccountEndpointhttps://accountname.documents.azure.com:443AccountKeyaccountkey",
            "invalid connection string",
        );
    }

    #[test]
    fn partially_malformed_connection_string() {
        test_bad_connection_string(
            "AccountEndpointhttps://accountname.documents.azure.com:443/AccountKey=accountkey",
            "invalid connection string, missing 'AccountEndpoint'",
        );
    }

    #[test]
    fn connection_string_missing_account_endpoint() {
        test_bad_connection_string(
            "AccountKey=key",
            "invalid connection string, missing 'AccountEndpoint'",
        );
    }

    #[test]
    fn connection_string_missing_account_key() {
        test_bad_connection_string(
            "AccountEndpoint=https://accountname.documents.azure.com:443/;",
            "invalid connection string, missing 'AccountKey'",
        );
    }

    fn test_bad_connection_string(connection_string: &str, expected_error_message: &str) {
        let secret = Secret::new(connection_string.to_owned());
        let error = ConnectionString::try_from(&secret)
            .expect_err("Expected error parsing connection string");
        assert!(
            error.to_string().contains(expected_error_message),
            "Expected error message to contain '{}', but got: {}",
            expected_error_message,
            error
        );
    }
}
