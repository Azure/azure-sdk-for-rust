// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;

use azure_core::{credentials::Secret, http::Model, Error};
use serde::Deserialize;
use std::fmt::Debug;

/// Represents a Cosmos DB connection string.
///
/// The [`Debug`] implementation will not print the account key.
#[derive(Model, Clone, Deserialize, PartialEq, Eq)]
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

impl Debug for ConnectionString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!(
            "AccountEndpoint={};AccountKey=Secret;",
            self.account_endpoint
        );

        f.write_str(&debug_string)
    }
}
