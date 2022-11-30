use crate::{clients::ServiceType, StorageCredentials};
use once_cell::sync::Lazy;
use std::{
    convert::TryFrom,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use url::Url;

const AZURE_CLOUD: &str = "AzureCloud";
const AZURE_PUBLIC_CLOUD: &str = "AzurePublicCloud";
const AZURE_CHINA_CLOUD: &str = "AzureChinaCloud";
const AZURE_US_GOV: &str = "AzureUSGovernment";
const AZURE_GERMAN_CLOUD: &str = "AzureGermanCloud";

/// The cloud with which you want to interact.
#[derive(Debug, Clone)]
pub enum CloudLocation {
    /// Azure public cloud
    Public {
        account: String,
        credentials: StorageCredentials,
    },
    /// Azure China cloud
    China {
        account: String,
        credentials: StorageCredentials,
    },
    /// Azure US Government
    USGov {
        account: String,
        credentials: StorageCredentials,
    },
    /// Azure German Government
    /// Note: This seems like it will be deprecated, but still shows up in `az cloud list --output table`, so adding it for
    /// completeness
    GermanCloud {
        account: String,
        credentials: StorageCredentials,
    },
    /// Use the well-known emulator
    Emulator { address: String, port: u16 },
    /// Auto-detect location based on `AZURE_CLOUD_NAME` variable or `$HOME/.azure/config`
    AutoDetect {
        account: String,
        credentials: StorageCredentials,
        /// Optional fallback cloud-location, for example in test-environments
        ///
        fallback: Option<Box<Self>>,
    },
    /// A custom base URL
    Custom {
        uri: String,
        credentials: StorageCredentials,
    },
}

impl CloudLocation {
    /// Returns a cloud location that will auto-detect the current cloud,
    ///
    /// This will auto detect first by checking the `AZURE_CLOUD_NAME` env variable, and then next
    /// by parsing the current user's `$HOME/.azure/config` file.
    ///
    /// If either of these methods fail, this will default to public azure.
    ///
    /// Cloud names can be listed with, `az cloud list --output table`. Current public values are:
    /// - AzureCloud - Public azure, in some cases may be AzurePublicCloud in env var form
    /// - AzureChinaCloud
    /// - AzureUSGovernment
    /// - AzureGermanCloud - Might be deprecating based on public documentation but still shows up in `az cloud list`
    ///
    pub fn auto_detect(account: impl AsRef<str>, credentials: StorageCredentials) -> CloudLocation {
        Self::auto_detect_with_fallback(account, credentials, None)
    }

    /// Returns a cloud location that will auto-detect the current cloud,
    ///
    /// See CloudLocation::auto_detect(..) for more info. This fn is for more advanced scenarios.
    ///
    /// If a fallback option is set, and if no cloud location could be explicitly detected, the fallback location will be used instead.
    ///
    pub fn auto_detect_with_fallback(
        account: impl AsRef<str>,
        credentials: StorageCredentials,
        fallback: Option<CloudLocation>,
    ) -> CloudLocation {
        CloudLocation::AutoDetect {
            account: account.as_ref().to_string(),
            credentials,
            fallback: fallback.map(Box::new),
        }
    }
    /// the base URL for a given cloud location
    pub fn url(&self, service_type: ServiceType) -> azure_core::Result<Url> {
        let url = match self {
            CloudLocation::Public { account, .. } => {
                format!(
                    "https://{}.{}.core.windows.net",
                    account,
                    service_type.subdomain()
                )
            }
            CloudLocation::China { account, .. } => {
                format!(
                    "https://{}.{}.core.chinacloudapi.cn",
                    account,
                    service_type.subdomain()
                )
            }
            CloudLocation::USGov { account, .. } => {
                format!(
                    "https://{}.{}.core.usgovcloudapi.net",
                    account,
                    service_type.subdomain()
                )
            }
            CloudLocation::GermanCloud { account, .. } => {
                format!(
                    "https://{}.{}.core.cloudapi.de",
                    account,
                    service_type.subdomain()
                )
            }
            CloudLocation::Custom { uri, .. } => uri.clone(),
            CloudLocation::Emulator { address, port } => {
                format!("http://{address}:{port}/{EMULATOR_ACCOUNT}")
            }
            CloudLocation::AutoDetect {
                account,
                credentials,
                fallback,
            } => {
                if let Some(name) = Self::find_cloud_name() {
                    // These names are from
                    // `az cloud list --output table`
                    return match name.as_str() {
                        // Seems like "AzurePublicCloud" is used in some environments
                        AZURE_CLOUD | AZURE_PUBLIC_CLOUD => CloudLocation::Public {
                            account: account.clone(),
                            credentials: credentials.clone(),
                        }
                        .url(service_type),
                        AZURE_US_GOV => CloudLocation::USGov {
                            account: account.clone(),
                            credentials: credentials.clone(),
                        }
                        .url(service_type),
                        AZURE_CHINA_CLOUD => CloudLocation::China {
                            account: account.clone(),
                            credentials: credentials.clone(),
                        }
                        .url(service_type),
                        AZURE_GERMAN_CLOUD => CloudLocation::GermanCloud {
                            account: account.clone(),
                            credentials: credentials.clone(),
                        }
                        .url(service_type),
                        _ => {
                            todo!()
                        }
                    };
                } else if let Some(fallback) = fallback {
                    return fallback.url(service_type);
                } else {
                    // Default to PROD
                    return CloudLocation::Public {
                        account: account.clone(),
                        credentials: credentials.clone(),
                    }
                    .url(service_type);
                }
            }
        };
        Ok(url::Url::parse(&url)?)
    }

    /// Returns the storage credentials for this cloud location,
    ///
    pub fn credentials(&self) -> &StorageCredentials {
        match self {
            CloudLocation::Public { credentials, .. }
            | CloudLocation::China { credentials, .. }
            | CloudLocation::USGov { credentials, .. }
            | CloudLocation::GermanCloud { credentials, .. }
            | CloudLocation::Custom { credentials, .. }
            | CloudLocation::AutoDetect { credentials, .. } => credentials,
            CloudLocation::Emulator { .. } => &EMULATOR_CREDENTIALS,
        }
    }

    /// Finds the cloud name, first by environment variable, then by parsing the current user's $HOME/.azure/config file
    ///
    fn find_cloud_name() -> Option<String> {
        if let Ok(name) = std::env::var("AZURE_CLOUD_NAME") {
            Some(name)
        } else if let Ok(home_dir) = std::env::var("HOME") {
            if let Some(config) = PathBuf::from(home_dir)
                .join(".azure/config")
                .canonicalize()
                .ok()
                .and_then(|config| File::open(config).ok())
            {
                let mut lines = BufReader::new(config).lines();

                while let Some(Ok(line)) = lines.next() {
                    // Alternatively, import serde_toml for parsing this file, but probably better off doing
                    // that by creating a struct dedicated to managing the config file
                    if line.trim() == "[cloud]" {
                        if let Some(Ok(name)) = lines.next() {
                            if let Some((name, value)) = name.split_once('=') {
                                if name.trim() == "name" {
                                    return Some(value.trim().to_string());
                                }
                            }
                        }
                    }
                }
            }
            None
        } else {
            None
        }
    }
}

impl TryFrom<&Url> for CloudLocation {
    type Error = azure_core::Error;

    // TODO: This only works for Public and China clouds.
    // ref: https://github.com/Azure/azure-sdk-for-rust/issues/502
    fn try_from(url: &Url) -> azure_core::Result<Self> {
        let token = url.query().ok_or_else(|| {
            azure_core::Error::with_message(azure_core::error::ErrorKind::DataConversion, || {
                "unable to find SAS token in URL"
            })
        })?;
        let credentials = StorageCredentials::sas_token(token)?;

        let host = url.host_str().ok_or_else(|| {
            azure_core::Error::with_message(azure_core::error::ErrorKind::DataConversion, || {
                "unable to find the target host in the URL"
            })
        })?;

        let mut domain = host.split_terminator('.').collect::<Vec<_>>();
        if domain.len() < 2 {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                || {
                    format!(
                        "URL refers to a domain that is not a Public or China domain: {}",
                        host
                    )
                },
            ));
        }

        let account = domain.remove(0).to_string();
        domain.remove(0);
        let rest = domain.join(".");

        match rest.as_str() {
            "core.windows.net" => Ok(CloudLocation::Public {
                account,
                credentials,
            }),
            "core.chinacloudapi.cn" => Ok(CloudLocation::China {
                account,
                credentials,
            }),
            _ => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                || {
                    format!(
                        "URL refers to a domain that is not a Public or China domain: {}",
                        host
                    )
                },
            )),
        }
    }
}

pub static EMULATOR_CREDENTIALS: Lazy<StorageCredentials> = Lazy::new(|| {
    StorageCredentials::Key(EMULATOR_ACCOUNT.to_owned(), EMULATOR_ACCOUNT_KEY.to_owned())
});

/// The well-known account used by Azurite and the legacy Azure Storage Emulator.
/// <https://docs.microsoft.com/azure/storage/common/storage-use-azurite#well-known-storage-account-and-key>
pub const EMULATOR_ACCOUNT: &str = "devstoreaccount1";

/// The well-known account key used by Azurite and the legacy Azure Storage Emulator.
/// <https://docs.microsoft.com/azure/storage/common/storage-use-azurite#well-known-storage-account-and-key>
pub const EMULATOR_ACCOUNT_KEY: &str =
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_url() -> azure_core::Result<()> {
        let public_without_token = Url::parse("https://test.blob.core.windows.net")?;
        let public_with_token = Url::parse("https://test.blob.core.windows.net/?token=1")?;

        let cloud_location: CloudLocation = (&public_with_token).try_into()?;
        assert_eq!(public_without_token, cloud_location.url(ServiceType::Blob)?);

        let creds = cloud_location.credentials();
        assert!(matches!(creds, &StorageCredentials::SASToken(_)));

        let file_url = Url::parse("file://tmp/test.txt")?;
        let result: azure_core::Result<CloudLocation> = (&file_url).try_into();
        assert!(result.is_err());

        let missing_account = Url::parse("https://blob.core.windows.net?token=1")?;
        let result: azure_core::Result<CloudLocation> = (&missing_account).try_into();
        assert!(result.is_err());

        let missing_service_type = Url::parse("https://core.windows.net?token=1")?;
        let result: azure_core::Result<CloudLocation> = (&missing_service_type).try_into();
        assert!(result.is_err());

        let china_cloud = Url::parse("https://test.blob.core.chinacloudapi.cn/?token=1")?;
        let china_cloud_without_token = Url::parse("https://test.blob.core.chinacloudapi.cn")?;

        let cloud_location: CloudLocation = (&china_cloud).try_into()?;
        assert_eq!(
            china_cloud_without_token,
            cloud_location.url(ServiceType::Blob)?
        );

        Ok(())
    }

    #[test]
    fn test_auto_detect() {
        let cloud_location: CloudLocation =
            CloudLocation::auto_detect("test_account", StorageCredentials::Anonymous);

        std::env::set_var("AZURE_CLOUD_NAME", AZURE_US_GOV);
        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should return a url")
                .as_str(),
            "https://test_account.blob.core.usgovcloudapi.net/"
        );

        std::env::set_var("AZURE_CLOUD_NAME", AZURE_CHINA_CLOUD);
        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should return a url")
                .as_str(),
            "https://test_account.blob.core.chinacloudapi.cn/"
        );

        std::env::set_var("AZURE_CLOUD_NAME", AZURE_CLOUD);
        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should return a url")
                .as_str(),
            "https://test_account.blob.core.windows.net/"
        );

        std::env::set_var("AZURE_CLOUD_NAME", AZURE_PUBLIC_CLOUD);
        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should return a url")
                .as_str(),
            "https://test_account.blob.core.windows.net/"
        );

        std::env::set_var("AZURE_CLOUD_NAME", AZURE_GERMAN_CLOUD);
        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should return a url")
                .as_str(),
            "https://test_account.blob.core.cloudapi.de/"
        );

        std::env::remove_var("AZURE_CLOUD_NAME");

        std::env::set_var("HOME", ".test");
        std::fs::create_dir_all(".test/.azure").expect("should be able to make test dir");
        std::fs::write(
            ".test/.azure/config",
            r#"
[cloud]
name = AzureCloud
            "#
            .trim(),
        )
        .expect("should be able to write test config file");
        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should return a url")
                .as_str(),
            "https://test_account.blob.core.windows.net/"
        );

        std::fs::write(
            ".test/.azure/config",
            r#"
[cloud]
name = AzureChinaCloud
            "#
            .trim(),
        )
        .expect("should be able to write test config file");
        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should return a url")
                .as_str(),
            "https://test_account.blob.core.chinacloudapi.cn/"
        );

        std::fs::write(
            ".test/.azure/config",
            r#"
[cloud]
name = AzureUSGovernment
            "#
            .trim(),
        )
        .expect("should be able to write test config file");
        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should return a url")
                .as_str(),
            "https://test_account.blob.core.usgovcloudapi.net/"
        );

        std::fs::write(
            ".test/.azure/config",
            r#"
            "#
            .trim(),
        )
        .expect("should be able to write test config file");
        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should return a url")
                .as_str(),
            "https://test_account.blob.core.windows.net/"
        );

        // Clean-up test files
        std::fs::remove_dir_all(".test").expect("should be able to remove test dir");

        std::env::remove_var("HOME");
        std::env::remove_var("AZURE_CLOUD_NAME");

        // Test fallback
        let cloud_location: CloudLocation = CloudLocation::auto_detect_with_fallback(
            "test_account",
            StorageCredentials::Anonymous,
            Some(CloudLocation::Emulator {
                address: "test".to_string(),
                port: 0000,
            }),
        );

        assert_eq!(
            cloud_location
                .url(ServiceType::Blob)
                .expect("should be a url")
                .to_string(),
            format!("http://test:0/{EMULATOR_ACCOUNT}")
        );
    }
}
