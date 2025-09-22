// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::CertificateOperation;
use azure_core::{error::ErrorKind, http::Url, Result};
use std::str::FromStr;

/// Information about the resource.
///
/// Call [`ResourceExt::resource_id()`] on supported models e.g., [`Certificate`](crate::models::Certificate) to get this information.
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
        deconstruct(url, true)
    }
}

/// Extension methods to get a [`ResourceId`] from models in this crate.
pub trait ResourceExt {
    /// Gets the [`ResourceId`] from this model.
    ///
    /// You can parse the name and version to pass to subsequent [`CertificateClient`](crate::CertificateClient) method calls.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_security_keyvault_certificates::{models::Certificate, ResourceExt as _};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // CertificateClient::get_certificate() will return a Certificate.
    /// let mut certificate = Certificate::default();
    /// certificate.id = Some("https://my-vault.vault.azure.net/certificates/my-certificate/abcd1234?api-version=7.5".into());
    ///
    /// let id = certificate.resource_id()?;
    /// assert_eq!(id.vault_url, "https://my-vault.vault.azure.net");
    /// assert_eq!(id.name, "my-certificate");
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
        deconstruct(&url, true)
    }
}

impl ResourceExt for CertificateOperation {
    fn resource_id(&self) -> Result<ResourceId> {
        let Some(id) = self.id.as_ref() else {
            return Err(azure_core::Error::with_message(
                ErrorKind::DataConversion,
                "missing resource id",
            ));
        };

        let url: Url = id.parse()?;
        deconstruct(&url, false)
    }
}

fn deconstruct(url: &Url, version: bool) -> Result<ResourceId> {
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
            if col != "certificates" {
                return Err(azure_core::Error::with_message(
                    ErrorKind::DataConversion,
                    "not in certificates collection",
                ));
            }
            Ok(col)
        })?;
    let name = segments
        .next()
        .ok_or_else(|| azure_core::Error::with_message(ErrorKind::DataConversion, "missing name"))
        .map(String::from)?;

    let mut resource_id = ResourceId {
        source_id: url.as_str().into(),
        vault_url,
        name,
        version: None,
    };
    if version {
        resource_id.version = segments.next().map(String::from);
    }

    Ok(resource_id)
}

mod private {
    use crate::models::{
        Certificate, CertificateProperties, DeletedCertificate, DeletedCertificateProperties,
    };

    pub trait AsId {
        fn as_id(&self) -> Option<&String>;
    }

    impl AsId for Certificate {
        fn as_id(&self) -> Option<&String> {
            self.id.as_ref()
        }
    }

    impl AsId for CertificateProperties {
        fn as_id(&self) -> Option<&String> {
            self.id.as_ref()
        }
    }

    impl AsId for DeletedCertificate {
        fn as_id(&self) -> Option<&String> {
            self.id.as_ref()
        }
    }

    impl AsId for DeletedCertificateProperties {
        fn as_id(&self) -> Option<&String> {
            self.id.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Certificate;

    use super::*;

    #[test]
    fn try_from_str() {
        assert_eq!(
            "https://vault.azure.net/certificates/name/version"
                .parse::<ResourceId>()
                .unwrap(),
            ResourceId {
                source_id: "https://vault.azure.net/certificates/name/version".to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }

    #[test]
    fn try_from_url() {
        let url: Url = "https://vault.azure.net/certificates/name/version"
            .parse()
            .unwrap();
        let resource: ResourceId = url.try_into().unwrap();
        assert_eq!(
            resource,
            ResourceId {
                source_id: "https://vault.azure.net/certificates/name/version".to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }

    #[test]
    fn test_deconstruct() {
        deconstruct(&"file:///tmp".parse().unwrap(), true).expect_err("cannot-be-base url");
        deconstruct(&"https://vault.azure.net/".parse().unwrap(), true)
            .expect_err("missing collection");
        deconstruct(
            &"https://vault.azure.net/collection/".parse().unwrap(),
            true,
        )
        .expect_err("invalid collection");
        deconstruct(
            &"https://vault.azure.net/certificates/".parse().unwrap(),
            true,
        )
        .expect_err("missing name");

        let url: Url = "https://vault.azure.net/certificates/name".parse().unwrap();
        assert_eq!(
            deconstruct(&url, true).unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: None
            }
        );

        let url: Url = "https://vault.azure.net/certificates/name/version"
            .parse()
            .unwrap();
        assert_eq!(
            deconstruct(&url, true).unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );

        let url: Url = "https://vault.azure.net:443/certificates/name/version"
            .parse()
            .unwrap();
        assert_eq!(
            deconstruct(&url, true).unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );

        let url: Url = "https://vault.azure.net:8443/certificates/name/version"
            .parse()
            .unwrap();
        assert_eq!(
            deconstruct(&url, true).unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net:8443".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }

    #[test]
    fn from_certificate_bundle() {
        let mut certificate = Certificate {
            id: None,
            ..Default::default()
        };
        certificate.resource_id().expect_err("missing resource id");

        let url: Url = "https://vault.azure.net/certificates/name/version"
            .parse()
            .unwrap();
        certificate.id = Some(url.clone().into());
        assert_eq!(
            certificate.resource_id().unwrap(),
            ResourceId {
                source_id: url.to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }

    #[test]
    fn from_certificate_operation() {
        use crate::models::CertificateOperation;

        let mut operation = CertificateOperation {
            id: None,
            ..Default::default()
        };
        operation.resource_id().expect_err("missing resource id");

        let url: Url = "https://vault.azure.net/certificates/name/pending"
            .parse()
            .unwrap();
        operation.id = Some(url.clone().into());

        // CertificateOperation should not include version in the ResourceId
        let resource_id = operation.resource_id().unwrap();
        assert_eq!(resource_id.source_id, url.to_string());
        assert_eq!(resource_id.vault_url, "https://vault.azure.net");
        assert_eq!(resource_id.name, "name");
        assert_eq!(resource_id.version, None);

        // Test with a URL that has a port
        let url_with_port: Url = "https://vault.azure.net:8443/certificates/cert-name/pending"
            .parse()
            .unwrap();
        operation.id = Some(url_with_port.clone().into());

        let resource_id = operation.resource_id().unwrap();
        assert_eq!(resource_id.source_id, url_with_port.to_string());
        assert_eq!(resource_id.vault_url, "https://vault.azure.net:8443");
        assert_eq!(resource_id.name, "cert-name");
        assert_eq!(resource_id.version, None);
    }

    #[test]
    fn canonicalizes() {
        assert_eq!(
            "https://vault.azure.net//certificates/name/version"
                .parse::<ResourceId>()
                .unwrap(),
            ResourceId {
                source_id: "https://vault.azure.net//certificates/name/version".to_string(),
                vault_url: "https://vault.azure.net".into(),
                name: "name".into(),
                version: Some("version".into()),
            }
        );
    }
}
