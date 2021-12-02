use super::{table_sas_ip_range::TableSasIpRange, table_sas_protocol::TableSasProtocol};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct TableAccountSasOptions {
    /// Gets the optional IP address or a range of IP addresses from which to accept requests.
    /// When specifying a range, note that the range is inclusive.
    pub ip_range: Option<TableSasIpRange>,

    /// Optional. Specifies the protocol permitted for a request made with
    /// the shared access signature.
    pub protocol: Option<TableSasProtocol>,

    /// Gets the optional time at which the shared access signature becomes valid.
    /// If omitted, start time for this call is assumed to be the time when the storage service receives the request.
    pub start_time: Option<DateTime<Utc>>,

    /// Gets the optional unique value up to 64 characters in length that
    /// correlates to an access policy specified for the blob container, queue, or share.
    pub identifier: Option<String>,
}

impl Default for TableAccountSasOptions {
    fn default() -> Self {
        Self {
            start_time: Some(Utc::now()),
            ip_range: Default::default(),
            identifier: Default::default(),
            protocol: Some(TableSasProtocol::HttpsAndHttp),
        }
    }
}

impl TableAccountSasOptions {
    setters! {
        ip_range: TableSasIpRange => Some(ip_range),
        protocol: TableSasProtocol  => Some(protocol),
        start_time:  DateTime<Utc> => Some(start_time),
        identifier: String => Some(identifier),
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, Utc};

    use super::TableAccountSasOptions;
    use crate::authorization::sas_token::options::{
        table_sas_ip_range::TableSasIpRange, table_sas_protocol::TableSasProtocol,
    };

    #[test]
    fn creation_test() {
        let sas_options = TableAccountSasOptions::default()
            .ip_range(TableSasIpRange::new([127, 0, 0, 1], [127, 0, 0, 15]))
            .start_time(Utc::now() + Duration::hours(1))
            .protocol(TableSasProtocol::Https)
            .identifier("some_identifier");
    }
}
// /// Gets the time at which the shared access signature becomes invalid.
// pub(crate) expiry_time: DateTime<Utc>,

// /// Gets the permissions associated with the shared access signature.
// /// The user is restricted to operations allowed by the permissions.
// /// This field must be omitted if it has been specified in an
// /// associated stored access policy.
// pub(crate) permissions_builder: TableAccountSasPermissions,

// /// Gets which resources are accessible via the shared access signature.
// pub(crate) resource_types_builder: TableAccountSasResourceTypes,
