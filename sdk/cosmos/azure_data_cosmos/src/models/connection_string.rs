// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;

use azure_core::{credentials::Secret, fmt::SafeDebug, Error};
use serde::Deserialize;

/// Represents a Cosmos DB connection string.
///
/// The [`Debug`] implementation will not print the account key.
#[derive(Clone, Deserialize, PartialEq, Eq, SafeDebug)]
pub struct ConnectionString {
    pub account_endpoint: Cow<'static, str>,
    pub account_key: Secret,
}

impl ConnectionString {
    pub fn from_connection_string(connection_string: &str) -> Result<Self, Error> {
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
            match key {
                "AccountEndpoint" => account_endpoint = Some(value.to_string()),
                "AccountKey" => account_key = Some(Secret::new(value.to_string())),
                _ => {}
            }
        }

        let Some(endpoint) = account_endpoint else {
            return Err(Error::new(
                azure_core::error::ErrorKind::Other,
                "invalid connection string, missing 'AccountEndpoint",
            ));
        };

        let Some(key) = account_key else {
            return Err(Error::new(
                azure_core::error::ErrorKind::Other,
                "invalid connection string, missing 'AccountKey",
            ));
        };

        Ok(Self {
            account_endpoint: Cow::Owned(endpoint),
            account_key: key,
        })
    }

    pub fn from_secret(secret: &Secret) -> azure_core::Result<Self> {
        ConnectionString::from_connection_string(secret.secret())
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionString;
    use azure_core::credentials::Secret;

    #[test]
    pub fn test_regular_connection_string() {
        let connection_string =
            "AccountEndpoint=https://accountname.documents.azure.com:443/;AccountKey=accountkey";
        let secret = Secret::new(connection_string);
        let connection_str = ConnectionString::from_secret(&secret).unwrap();

        assert_eq!(
            "https://accountname.documents.azure.com:443/",
            connection_str.account_endpoint
        );

        assert_eq!("accountkey", connection_str.account_key.secret());
    }

    #[test]
    pub fn test_empty_connection_string() {
        test_bad_connection_string("")
    }

    #[test]
    pub fn test_malformed_connection_string() {
        test_bad_connection_string(
            "AccountEndpointhttps://accountname.documents.azure.com:443/AccountKey=accountkey",
        );
    }

    #[test]
    pub fn test_connection_string_missing_account_endpoint() {
        test_bad_connection_string("AccountKey=accountkey");
    }

    #[test]
    pub fn test_connection_string_missing_account_key() {
        test_bad_connection_string("AccountEndpoint=https://accountname.documents.azure.com:443/;");
    }

    fn test_bad_connection_string(connection_string: &str) {
        let secret = Secret::new(connection_string.to_owned());
        let connection_str = ConnectionString::from_secret(&secret);
        assert!(connection_str.is_err());
    }
}
