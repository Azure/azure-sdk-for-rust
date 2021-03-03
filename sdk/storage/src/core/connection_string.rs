use azure_core::errors::AzureError;

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

quick_error! {
    #[derive(Debug)]
    pub enum ConnectionStringError {
        MissingValue { key: String } {
            display("Missing value for key '{}'", key)
        }
        UnexpectedKey { key: String } {
            display("Unexpected for key '{}'", key)
        }
        ParsingError { msg: String } {
            display("{}", msg)
        }
        UnsupportedProtocol { protocol: String } {
            display("unsupported protocol {}", protocol)
        }
    }
}

impl From<ConnectionStringError> for AzureError {
    fn from(err: ConnectionStringError) -> Self {
        AzureError::GenericErrorWithText(err.to_string())
    }
}

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
        write!(f, "{}", s)
    }
}

/// A storage connection string.
///
/// The key are a subset of what is defined in the
/// [.NET SDK](https://github.com/Azure/azure-storage-net/blob/ed89733dfb170707d65b7b8b0be36cb5bd6512e4/Lib/Common/CloudStorageAccount.cs#L63-L261).
#[derive(Debug)]
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

impl<'a> Default for ConnectionString<'a> {
    fn default() -> Self {
        Self {
            account_name: None,
            account_key: None,
            sas: None,
            default_endpoints_protocol: None,
            endpoint_suffix: None,
            use_development_storage: None,
            development_storage_proxy_uri: None,
            blob_endpoint: None,
            blob_secondary_endpoint: None,
            table_endpoint: None,
            table_secondary_endpoint: None,
            queue_endpoint: None,
            queue_secondary_endpoint: None,
            file_endpoint: None,
            file_secondary_endpoint: None,
        }
    }
}

impl<'a> PartialEq for ConnectionString<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.account_name == other.account_name
            && self.account_key == other.account_key
            && self.sas == other.sas
            && self.default_endpoints_protocol == other.default_endpoints_protocol
            && self.endpoint_suffix == other.endpoint_suffix
            && self.use_development_storage == other.use_development_storage
            && self.development_storage_proxy_uri == other.development_storage_proxy_uri
            && self.blob_endpoint == other.blob_endpoint
            && self.blob_secondary_endpoint == other.blob_secondary_endpoint
            && self.table_endpoint == other.table_endpoint
            && self.table_secondary_endpoint == other.table_secondary_endpoint
            && self.queue_endpoint == other.queue_endpoint
            && self.queue_secondary_endpoint == other.queue_secondary_endpoint
            && self.file_endpoint == other.file_endpoint
            && self.file_secondary_endpoint == other.file_secondary_endpoint
    }
}

impl<'a> ConnectionString<'a> {
    pub fn new(connection_string: &'a str) -> Result<Self, ConnectionStringError> {
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
            let mut kv = kv_pair_str.trim().split('=');
            let k = match kv.next() {
                Some(k) if k.chars().all(char::is_whitespace) => {
                    return Err(ConnectionStringError::ParsingError {
                        msg: "No key found".to_owned(),
                    })
                }
                None => {
                    return Err(ConnectionStringError::ParsingError {
                        msg: "No key found".to_owned(),
                    })
                }
                Some(k) => k,
            };
            let v = match kv.next() {
                Some(v) if v.chars().all(char::is_whitespace) => {
                    return Err(ConnectionStringError::MissingValue { key: k.to_owned() })
                }
                None => return Err(ConnectionStringError::MissingValue { key: k.to_owned() }),
                Some(v) => v,
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
                            return Err(ConnectionStringError::UnsupportedProtocol {
                                protocol: v.to_owned(),
                            })
                        }
                    };
                    default_endpoints_protocol = Some(protocol);
                }
                USE_DEVELOPMENT_STORAGE_KEY_NAME => match v {
                    "true" => use_development_storage = Some(true),
                    "false" => use_development_storage = Some(false),
                    _ => {
                        return Err(ConnectionStringError::ParsingError {
                            msg: format!(
                        "Unexpected value for {}: {}. Please specify either 'true' or 'false'.",
                        USE_DEVELOPMENT_STORAGE_KEY_NAME, v),
                        })
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
                k => return Err(ConnectionStringError::UnexpectedKey { key: k.to_owned() }),
            }
        }

        Ok(Self {
            account_name,
            account_key,
            sas,
            endpoint_suffix,
            default_endpoints_protocol,
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
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_returns_expected_errors() {
        assert!(matches!(
            ConnectionString::new("AccountName="),
            Err(ConnectionStringError::MissingValue { key }) if key == "AccountName"
        ));
        assert!(matches!(
            ConnectionString::new("="),
            Err(ConnectionStringError::ParsingError { msg: _ })
        ));
        assert!(matches!(
            ConnectionString::new("x=123;"),
            Err(ConnectionStringError::UnexpectedKey { key }) if key == "x"
        ));
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
