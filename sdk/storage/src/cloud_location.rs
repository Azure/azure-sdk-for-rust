use crate::clients::ServiceType;
use std::convert::TryFrom;
use url::Url;

/// The cloud with which you want to interact.
// TODO: Other govt clouds?
#[derive(Debug, Clone)]
pub enum CloudLocation {
    /// Azure public cloud
    Public { account: String },
    /// Azure China cloud
    China { account: String },
    /// Use the well-known emulator
    Emulator { address: String, port: u16 },
    /// A custom base URL
    Custom { uri: String },
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
}

impl TryFrom<&Url> for CloudLocation {
    type Error = azure_core::Error;

    // TODO: This only works for Public and China clouds.
    // ref: https://github.com/Azure/azure-sdk-for-rust/issues/502
    fn try_from(url: &Url) -> azure_core::Result<Self> {
        let host = url.host_str().ok_or_else(|| {
            azure_core::Error::with_message(azure_core::error::ErrorKind::DataConversion, || {
                "unable to find the target host in the URL"
            })
        })?;

        let mut domain = host.split_terminator('.').collect::<Vec<_>>();
        if domain.len() < 2 {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                || format!("URL refers to a domain that is not a Public or China domain: {host}"),
            ));
        }

        let account = domain.remove(0).to_string();
        domain.remove(0);
        let rest = domain.join(".");

        match rest.as_str() {
            "core.windows.net" => Ok(CloudLocation::Public { account }),
            "core.chinacloudapi.cn" => Ok(CloudLocation::China { account }),
            _ => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                || format!("URL refers to a domain that is not a Public or China domain: {host}"),
            )),
        }
    }
}

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
}
