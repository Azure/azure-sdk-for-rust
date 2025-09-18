// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{error::ErrorKind, http::Url, Result};
use std::str::FromStr;

/// Information about the resource.
///
/// Call [`ResourceExt::resource_id()`] on supported models e.g., [`Secret`](crate::models::Secret) to get this information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceId {
    /// The source URL of the resource.
    pub source_id: String,

    /// The vault URL containing the resource.
    pub vault_url: String,

    /// The name of the resource.
    pub name: String,

    /// The optional version of the resource.
    pub version: Option<String>,
}

impl FromStr for ResourceId {
    type Err = azure_core::Error;
    fn from_str(s: &str) -> Result<Self> {
        s.parse::<Url>()?.try_into()
    }
}

impl TryFrom<Url> for ResourceId {
    type Error = azure_core::Error;
    fn try_from(url: Url) -> Result<Self> {
        ResourceId::try_from(&url)
    }
}

impl TryFrom<&Url> for ResourceId {
    type Error = azure_core::Error;
    fn try_from(url: &Url) -> Result<Self> {
        deconstruct(url)
    }
}

/// Extension methods to get a [`ResourceId`] from models in this crate.
pub trait ResourceExt {
    /// Gets the [`ResourceId`] from this model.
    ///
    /// You can parse the name and version to pass to subsequent [`SecretClient`](crate::SecretClient) method calls.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_security_keyvault_secrets::{models::Secret, ResourceExt as _};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // SecretClient::get_secret() will return a Secret.
    /// let mut secret = Secret::default();
    /// secret.id = Some("https://my-vault.vault.azure.net/secrets/my-secret/abcd1234?api-version=7.5".into());
    ///
    /// let id = secret.resource_id()?;
    /// assert_eq!(id.vault_url, "https://my-vault.vault.azure.net");
    /// assert_eq!(id.name, "my-secret");
    /// assert_eq!(id.version, Some("abcd1234".into()));
    /// # Ok(())
    /// # }
    /// ```
    fn resource_id(&self) -> Result<ResourceId>;
}

impl<T> ResourceExt for T
where
    T: private::AsId,
{
    fn resource_id(&self) -> Result<ResourceId> {
        let Some(id) = self.as_id() else {
            return Err(azure_core::Error::with_message(
                ErrorKind::DataConversion,
                "missing resource id",
            ));
        };

        let url: Url = id.parse()?;
        deconstruct(&url)
    }
}

fn deconstruct(url: &Url) -> Result<ResourceId> {
    let vault_url = format!("{}://{}", url.scheme(), url.authority(),);
    let mut segments = url
        .path_segments()
        .ok_or_else(|| azure_core::Error::with_message(ErrorKind::DataConversion, "invalid url"))?
        .filter(|s| !s.is_empty());
    segments
        .next()
        .ok_or_else(|| {
            azure_core::Error::with_message(ErrorKind::DataConversion, "missing collection")
        })
        .and_then(|col| {
            if col != "secrets" {
                return Err(azure_core::Error::with_message(
                    ErrorKind::DataConversion,
                    "not in secrets collection",
                ));
            }
            Ok(col)
        })?;
    let name = segments
        .next()
        .ok_or_else(|| azure_core::Error::with_message(ErrorKind::DataConversion, "missing name"))
        .map(String::from)?;
    let version = segments.next().map(String::from);

    Ok(ResourceId {
        source_id: url.as_str().into(),
        vault_url,
        name,
        version,
    })
}

mod private {
    use crate::models::{DeletedSecret, DeletedSecretProperties, Secret, SecretProperties};

    pub trait AsId {
        fn as_id(&self) -> Option<&String>;
    }

    impl AsId for Secret {
        fn as_id(&self) -> Option<&String> {
            self.id.as_ref()
        }
    }

    impl AsId for SecretProperties {
        fn as_id(&self) -> Option<&String> {
            self.id.as_ref()
        }
    }

    impl AsId for DeletedSecret {
        fn as_id(&self) -> Option<&String> {
            self.id.as_ref()
        }
    }

    impl AsId for DeletedSecretProperties {
        fn as_id(&self) -> Option<&String> {
            self.id.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Secret;

    use super::*;

    #[test]
    fn try_from_str() {
        assert_eq!(
            "https://vault.azure.net/secrets/name/version"
                .parse::<ResourceId>()
                .unwrap(),
            ResourceId {
                source_id: "https://vault.azure.net/secrets/name/version".to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }

    #[test]
    fn try_from_url() {
        let url: Url = "https://vault.azure.net/secrets/name/version"
            .parse()
            .unwrap();
        let resource: ResourceId = url.try_into().unwrap();
        assert_eq!(
            resource,
            ResourceId {
                source_id: "https://vault.azure.net/secrets/name/version".to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }

    #[test]
    fn test_deconstruct() {
        deconstruct(&"file:///tmp".parse().unwrap()).expect_err("cannot-be-base url");
        deconstruct(&"https://vault.azure.net/".parse().unwrap()).expect_err("missing collection");
        deconstruct(&"https://vault.azure.net/collection/".parse().unwrap())
            .expect_err("invalid collection");
        deconstruct(&"https://vault.azure.net/secrets/".parse().unwrap())
            .expect_err("missing name");

        let url: Url = "https://vault.azure.net/secrets/name".parse().unwrap();
        assert_eq!(
            deconstruct(&url).unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: None
            }
        );

        let url: Url = "https://vault.azure.net/secrets/name/version"
            .parse()
            .unwrap();
        assert_eq!(
            deconstruct(&url).unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );

        let url: Url = "https://vault.azure.net:443/secrets/name/version"
            .parse()
            .unwrap();
        assert_eq!(
            deconstruct(&url).unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );

        let url: Url = "https://vault.azure.net:8443/secrets/name/version"
            .parse()
            .unwrap();
        assert_eq!(
            deconstruct(&url).unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net:8443".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }

    #[test]
    fn from_secret_bundle() {
        let mut secret = Secret {
            id: None,
            ..Default::default()
        };
        secret.resource_id().expect_err("missing resource id");

        let url: Url = "https://vault.azure.net/secrets/name/version"
            .parse()
            .unwrap();
        secret.id = Some(url.clone().into());
        assert_eq!(
            secret.resource_id().unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }

    #[test]
    fn canonicalizes() {
        assert_eq!(
            "https://vault.azure.net//secrets/name/version"
                .parse::<ResourceId>()
                .unwrap(),
            ResourceId {
                source_id: "https://vault.azure.net//secrets/name/version".to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }
}
