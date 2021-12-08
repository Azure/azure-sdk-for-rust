use super::{table_sas_ip_option::TableSasIpOption, table_sas_protocol::TableSasProtocol};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TableAccountSasOptionalOptions {
    /// Gets the optional IP address or a range of IP addresses from which to accept requests.
    /// When specifying a range, note that the range is inclusive.
    pub ip: Option<TableSasIpOption>,

    /// Optional. Specifies the protocol permitted for a request made with
    /// the shared access signature.
    pub protocol: Option<TableSasProtocol>,

    /// The time at which the SAS becomes valid.
    /// If omitted, the start time is assumed to be the time when the storage service receives the request.
    pub start_time: Option<DateTime<Utc>>,
}

impl TableAccountSasOptionalOptions {
    setters! {
        ip: TableSasIpOption => Some(ip),
        protocol: TableSasProtocol  => Some(protocol),
        start_time:  DateTime<Utc> => Some(start_time),
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, Utc};

    use super::TableAccountSasOptionalOptions;
    use crate::authorization::sas_token::options::{
        table_sas_ip_option::TableSasIpOption, table_sas_protocol::TableSasProtocol,
    };

    #[test]
    fn creation_test() {
        let sas_options = TableAccountSasOptionalOptions::default()
            .ip(TableSasIpOption::new_single([127, 0, 0, 1]))
            .start_time(Utc::now() + Duration::hours(1))
            .protocol(TableSasProtocol::Https);
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
