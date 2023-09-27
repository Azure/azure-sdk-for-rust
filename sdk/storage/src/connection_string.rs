use crate::StorageCredentials;
use azure_core::error::{Error, ErrorKind};

// Key names.
pub const ACCOUNT_KEY_KEY_NAME: &str = "AccountKey";
pub const ACCOUNT_NAME_KEY_NAME: &str = "AccountName";
pub const SAS_KEY_NAME: &str = "SharedAccessSignature";
pub const ENDPOINT_SUFFIX_KEY_NAME: &str = "EndpointSuffix";
pub const DEFAULT_ENDPOINTS_PROTOCOL_KEY_NAME: &str = "DefaultEndpointsProtocol";
pub const USE_DEVELOPMENT_STORAGE_KEY_NAME: &str = "UseDevelopmentStorage";
pub const DEVELOPMENT_STORAGE_PROXY_URI_KEY_NAME: &str = "DevelopmentStorageProxyUri";
pub const BLOB_ENDPOINT_KEY_NAME: &str = "BlobEndpoint";
pub const BLOB_SECONDARY_ENDPOINT_KEY_NAME: &str = "BlobSecondaryEndpoint";
pub const TABLE_ENDPOINT_KEY_NAME: &str = "TableEndpoint";
pub const TABLE_SECONDARY_ENDPOINT_KEY_NAME: &str = "TableSecondaryEndpoint";
pub const QUEUE_ENDPOINT_KEY_NAME: &str = "QueueEndpoint";
pub const QUEUE_SECONDARY_ENDPOINT_KEY_NAME: &str = "QueueSecondaryEndpoint";
pub const FILE_ENDPOINT_KEY_NAME: &str = "FileEndpoint";
pub const FILE_SECONDARY_ENDPOINT_KEY_NAME: &str = "FileSecondaryEndpoint";

#[derive(Debug, PartialEq, Eq)]
pub enum EndpointProtocol {
    Http,
    Https,
}

impl std::fmt::Display for EndpointProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            EndpointProtocol::Https => "https",
            EndpointProtocol::Http => "http",
        };
        write!(f, "{s}")
    }
}

/// A storage connection string.
///
/// The key are a subset of what is defined in the
/// [.NET SDK](https://github.com/Azure/azure-storage-net/blob/ed89733dfb170707d65b7b8b0be36cb5bd6512e4/Lib/Common/CloudStorageAccount.cs#L63-L261).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ConnectionString<'a> {
    /// Name of the storage account.
    pub account_name: Option<&'a str>,
    /// Account key of the storage account.
    pub account_key: Option<&'a str>,
    /// SAS (Shared Access Signature) key of the storage account.
    pub sas: Option<&'a str>,
    /// Default protocol to use for storage endpoints.
    pub default_endpoints_protocol: Option<EndpointProtocol>,
    /// Whether to use development storage.
    pub endpoint_suffix: Option<&'a str>,
    /// The development storage proxy URI.
    pub use_development_storage: Option<bool>,
    /// Custom storage endpoint suffix.
    pub development_storage_proxy_uri: Option<&'a str>,
    /// Custom blob storage endpoint.
    pub blob_endpoint: Option<&'a str>,
    /// Custom blob storage secondary endpoint.
    pub blob_secondary_endpoint: Option<&'a str>,
    /// Custom table storage endpoint.
    pub table_endpoint: Option<&'a str>,
    /// Custom table storage secondary endpoint.
    pub table_secondary_endpoint: Option<&'a str>,
    /// Custom queue storage endpoint.
    pub queue_endpoint: Option<&'a str>,
    /// Custom queue storage secondary endpoint.
    pub queue_secondary_endpoint: Option<&'a str>,
    /// Custom file storage endpoint.
    pub file_endpoint: Option<&'a str>,
    /// Custom file storage secondary endpoint.
    pub file_secondary_endpoint: Option<&'a str>,
}

impl<'a> ConnectionString<'a> {
    pub fn new(connection_string: &'a str) -> azure_core::Result<Self> {
        let mut account_name = None;
        let mut account_key = None;
        let mut sas = None;
        let mut endpoint_suffix = None;
        let mut default_endpoints_protocol = None;
        let mut use_development_storage = None;
        let mut development_storage_proxy_uri = None;
        let mut blob_endpoint = None;
        let mut blob_secondary_endpoint = None;
        let mut table_endpoint = None;
        let mut table_secondary_endpoint = None;
        let mut queue_endpoint = None;
        let mut queue_secondary_endpoint = None;
        let mut file_endpoint = None;
        let mut file_secondary_endpoint = None;

        let kv_str_pairs = connection_string
            .split(';')
            .filter(|s| !s.chars().all(char::is_whitespace));

        for kv_pair_str in kv_str_pairs {
            let kv = kv_pair_str.trim().split_once('=');

            let (k, v) = match kv {
                Some((k, _)) if (k.chars().all(char::is_whitespace) || k.trim() == "") => {
                    return Err(Error::with_message(ErrorKind::Other, || {
                        format!("no key found in connection string: {connection_string}")
                    }))
                }
                Some((k, v)) if (v.chars().all(char::is_whitespace) || v.trim() == "") => {
                    return Err(Error::with_message(ErrorKind::Other, || {
                        format!("missing value in connection string: {connection_string} key: {k}")
                    }))
                }
                Some((k, v)) => (k.trim(), v.trim()),
                None => {
                    return Err(Error::with_message(ErrorKind::Other, || {
                        format!("no key/value found in connection string: {connection_string}")
                    }))
                }
            };

            match k {
                ACCOUNT_NAME_KEY_NAME => account_name = Some(v),
                ACCOUNT_KEY_KEY_NAME => account_key = Some(v),
                SAS_KEY_NAME => sas = Some(v),
                ENDPOINT_SUFFIX_KEY_NAME => endpoint_suffix = Some(v),
                DEFAULT_ENDPOINTS_PROTOCOL_KEY_NAME => {
                    let protocol = match v {
                        "http" => EndpointProtocol::Http,
                        "https" => EndpointProtocol::Https,
                        _ => {
                            return Err(Error::with_message(ErrorKind::Other, || {
                                format!("connection string unsupported protocol: {v}")
                            }));
                        }
                    };
                    default_endpoints_protocol = Some(protocol);
                }
                USE_DEVELOPMENT_STORAGE_KEY_NAME => match v {
                    "true" => use_development_storage = Some(true),
                    "false" => use_development_storage = Some(false),
                    _ => {
                        return Err(Error::with_message(ErrorKind::Other, || {
                            format!("connection string unexpected value. {USE_DEVELOPMENT_STORAGE_KEY_NAME}: {v}. Please specify true or false.")
                        }));
                    }
                },
                DEVELOPMENT_STORAGE_PROXY_URI_KEY_NAME => development_storage_proxy_uri = Some(v),
                BLOB_ENDPOINT_KEY_NAME => blob_endpoint = Some(v),
                BLOB_SECONDARY_ENDPOINT_KEY_NAME => blob_secondary_endpoint = Some(v),
                TABLE_ENDPOINT_KEY_NAME => table_endpoint = Some(v),
                TABLE_SECONDARY_ENDPOINT_KEY_NAME => table_secondary_endpoint = Some(v),
                QUEUE_ENDPOINT_KEY_NAME => queue_endpoint = Some(v),
                QUEUE_SECONDARY_ENDPOINT_KEY_NAME => queue_secondary_endpoint = Some(v),
                FILE_ENDPOINT_KEY_NAME => file_endpoint = Some(v),
                FILE_SECONDARY_ENDPOINT_KEY_NAME => file_secondary_endpoint = Some(v),
                k => {
                    return Err(Error::with_message(ErrorKind::Other, || {
                        format!("connection string unexpected key: {k}")
                    }))
                }
            }
        }

        Ok(Self {
            account_name,
            account_key,
            sas,
            default_endpoints_protocol,
            endpoint_suffix,
            use_development_storage,
            development_storage_proxy_uri,
            blob_endpoint,
            blob_secondary_endpoint,
            table_endpoint,
            table_secondary_endpoint,
            queue_endpoint,
            queue_secondary_endpoint,
            file_endpoint,
            file_secondary_endpoint,
        })
    }

    pub fn storage_credentials(&self) -> azure_core::Result<StorageCredentials> {
        match self {
            ConnectionString {
                sas: Some(sas_token),
                ..
            } => {
                if self.account_key.is_some() {
                    log::warn!("Both account key and SAS defined in connection string. Using only the provided SAS.");
                }
                StorageCredentials::sas_token(*sas_token)
            }
            ConnectionString {
                account_name: Some(account),
                account_key: Some(key),
                ..
            } =>  Ok(StorageCredentials::access_key(*account, *key)),
           _ => {
                Err(Error::message(ErrorKind::Credential,
                    "Could not create a `StorageCredentail` from the provided connection string. Please validate that you have specified a means of authentication (key, SAS, etc.)."
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_returns_expected_errors() {
        assert!(ConnectionString::new("AccountName=").is_err());
        assert!(ConnectionString::new("AccountName    =").is_err());
        assert!(ConnectionString::new("MissingEquals").is_err());
        assert!(ConnectionString::new("=").is_err());
        assert!(ConnectionString::new("x=123;").is_err());
    }

    #[test]
    fn it_parses_empty_connection_string() {
        assert_eq!(
            ConnectionString::new("").unwrap(),
            ConnectionString::default()
        );
    }

    #[test]
    fn it_parses_basic_cases() {
        assert!(matches!(
            ConnectionString::new("AccountName=guy"),
            Ok(ConnectionString {
                account_name: Some("guy"),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("AccountName=guy;"),
            Ok(ConnectionString {
                account_name: Some("guy"),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("AccountName=guywald;AccountKey=1234"),
            Ok(ConnectionString {
                account_name: Some("guywald"),
                account_key: Some("1234"),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("AccountName=guywald;SharedAccessSignature=s"),
            Ok(ConnectionString {
                account_name: Some("guywald"),
                sas: Some("s"),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("AccountName=guywald;SharedAccessSignature=se=2036-01-01&sp=acw&sv=2018-11-09&sr=c&sig=c2lnbmF0dXJlCg%3D%3D"),
            Ok(ConnectionString {
                account_name: Some("guywald"),
                sas: Some("se=2036-01-01&sp=acw&sv=2018-11-09&sr=c&sig=c2lnbmF0dXJlCg%3D%3D"),
                ..
            })
        ));

        assert!(matches!(
            ConnectionString::new("AccountName = guywald;SharedAccessSignature = se=2036-01-01&sp=acw&sv=2018-11-09&sr=c&sig=c2lnbmF0dXJlCg%3D%3D"),
            Ok(ConnectionString {
                account_name: Some("guywald"),
                sas: Some("se=2036-01-01&sp=acw&sv=2018-11-09&sr=c&sig=c2lnbmF0dXJlCg%3D%3D"),
                ..
            })
        ));
    }

    #[test]
    fn it_parses_all_properties() {
        assert!(matches!(
                ConnectionString::new("AccountName=a;AccountKey=b;DefaultEndpointsProtocol=https;UseDevelopmentStorage=true;DevelopmentStorageProxyUri=c;BlobEndpoint=d;TableEndpoint=e;QueueEndpoint=f;SharedAccessSignature=g;"),
                Ok(ConnectionString {
                    account_name: Some("a"),
                    account_key: Some("b"),
                    default_endpoints_protocol: Some(EndpointProtocol::Https),
                    use_development_storage: Some(true),
                    development_storage_proxy_uri: Some("c"),
                    blob_endpoint: Some("d"),
                    table_endpoint: Some("e"),
                    sas: Some("g"),
                    ..
                })
            ));
        assert!(matches!(
                ConnectionString::new("BlobEndpoint=b1;BlobSecondaryEndpoint=b2;TableEndpoint=t1;TableSecondaryEndpoint=t2;QueueEndpoint=q1;QueueSecondaryEndpoint=q2;FileEndpoint=f1;FileSecondaryEndpoint=f2;"),
                Ok(ConnectionString {
                    blob_endpoint: Some("b1"),
                    blob_secondary_endpoint: Some("b2"),
                    table_endpoint: Some("t1"),
                    table_secondary_endpoint: Some("t2"),
                    queue_endpoint: Some("q1"),
                    queue_secondary_endpoint: Some("q2"),
                    file_endpoint: Some("f1"),
                    file_secondary_endpoint: Some("f2"),
                    ..
                })
            ));
    }

    #[test]
    fn it_parses_correct_endpoint_protocols() {
        assert!(matches!(
            ConnectionString::new("DefaultEndpointsProtocol=https"),
            Ok(ConnectionString {
                default_endpoints_protocol: Some(EndpointProtocol::Https),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("DefaultEndpointsProtocol=http"),
            Ok(ConnectionString {
                default_endpoints_protocol: Some(EndpointProtocol::Http),
                ..
            })
        ));
    }
}
