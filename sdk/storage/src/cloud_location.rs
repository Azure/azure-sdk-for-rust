use once_cell::sync::Lazy;

use crate::clients::ServiceType;
use crate::StorageCredentials;

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
    pub fn url(&self, service_type: ServiceType) -> azure_core::Result<url::Url> {
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
