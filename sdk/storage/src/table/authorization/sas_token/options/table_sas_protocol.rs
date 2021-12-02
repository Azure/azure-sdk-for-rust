use azure_core::TableSasParsingError;
use std::str::FromStr;

/// Defines the protocols permitted for Storage requests made with a shared access signature.
/// If no value is specified, the service will default to HttpsAndHttp.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TableSasProtocol {
    /// Only requests issued over HTTPS or HTTP will be permitted.
    HttpsAndHttp = 1,

    /// Only requests issued over HTTPS will be permitted.
    Https = 2,
}

impl From<TableSasProtocol> for String {
    fn from(protocol: TableSasProtocol) -> Self {
        match protocol {
            TableSasProtocol::HttpsAndHttp => "https,http",
            TableSasProtocol::Https => "https",
        }
        .into()
    }
}

impl FromStr for TableSasProtocol {
    type Err = TableSasParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "https" => Ok(TableSasProtocol::Https),
            "https,http" => Ok(TableSasProtocol::HttpsAndHttp),
            _ => Err(TableSasParsingError::ProtocolParsingError(s.into())),
        }
    }
}
