use crate::{clients::ServiceType, StorageCredentials};
use once_cell::sync::Lazy;
use std::convert::TryFrom;
use url::Url;

/// The cloud with which you want to interact.
// TODO: Other govt clouds?
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
    /// Use the well-known emulator
    Emulator { address: String, port: u16 },
    /// A custom base URL
    Custom {
        uri: String,
        credentials: StorageCredentials,
    },
}

impl CloudLocation {
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
            CloudLocation::Custom { uri, .. } => uri.clone(),
            CloudLocation::Emulator { address, port } => {
                format!("http://{address}:{port}/{EMULATOR_ACCOUNT}")
            }
        };
        Ok(url::Url::parse(&url)?)
    }

    pub fn credentials(&self) -> &StorageCredentials {
        match self {
            CloudLocation::Public { credentials, .. } => credentials,
            CloudLocation::China { credentials, .. } => credentials,
            CloudLocation::Emulator { .. } => &EMULATOR_CREDENTIALS,
            CloudLocation::Custom { credentials, .. } => credentials,
        }
    }
}

impl TryFrom<&Url> for CloudLocation {
    type Error = azure_core::Error;

    // TODO: This only works for Public and China clouds
    fn try_from(url: &Url) -> azure_core::Result<Self> {
        let token = url.query().ok_or_else(|| {
            azure_core::Error::with_message(azure_core::error::ErrorKind::DataConversion, || {
                "missing token"
            })
        })?;
        let credentials = StorageCredentials::sas_token(token)?;

        let host = url.host_str().ok_or_else(|| {
            azure_core::Error::with_message(azure_core::error::ErrorKind::DataConversion, || {
                "unable to find host from url"
            })
        })?;

        let mut domain = host.split_terminator('.').collect::<Vec<_>>();
        if domain.len() < 2 {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                || "unable to find host from url",
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
                || format!("CloudLocation conversion failed: {}", rest),
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
