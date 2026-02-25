// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Account endpoint types for Azure Cosmos DB.

use azure_core::http::Url;

/// The endpoint URL for a Cosmos DB account.
///
/// This is a newtype wrapper around [`Url`] that provides a strongly-typed representation
/// of a Cosmos DB account endpoint, such as `https://myaccount.documents.azure.com/`.
///
/// # Examples
///
/// Parsing from a string:
///
/// ```rust
/// use azure_data_cosmos::CosmosAccountEndpoint;
///
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// ```
///
/// Converting from a [`Url`](azure_core::http::Url):
///
/// ```rust
/// use azure_data_cosmos::CosmosAccountEndpoint;
/// use azure_core::http::Url;
///
/// let url: Url = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let endpoint = CosmosAccountEndpoint::from(url);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CosmosAccountEndpoint(Url);

impl CosmosAccountEndpoint {
    /// Returns a reference to the underlying [`Url`].
    pub fn url(&self) -> &Url {
        &self.0
    }

    /// Consumes this endpoint and returns the underlying [`Url`].
    pub fn into_url(self) -> Url {
        self.0
    }
}

impl std::str::FromStr for CosmosAccountEndpoint {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url: Url = s.parse().map_err(|e: url::ParseError| {
            azure_core::Error::new(azure_core::error::ErrorKind::Other, e)
        })?;
        Ok(Self(url))
    }
}

impl From<Url> for CosmosAccountEndpoint {
    fn from(url: Url) -> Self {
        Self(url)
    }
}

impl std::fmt::Display for CosmosAccountEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
