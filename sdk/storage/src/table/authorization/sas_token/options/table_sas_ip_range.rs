use azure_core::{SasError, TableSasParsingError};
use std::{net::IpAddr, str::FromStr};

/// Represents a range of allowed IP addresses for constructing a Shared Access Signature.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct TableSasIpRange {
    /// The start of the IP range.
    start: IpAddr,

    /// The optional end of the IP range.
    end: IpAddr,
}

impl TableSasIpRange {
    pub fn new(start: impl Into<IpAddr>, end: impl Into<IpAddr>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    /// Get a reference to the table sas ip range's start.
    pub fn start(&self) -> IpAddr {
        self.start
    }

    /// Get a reference to the table sas ip range's end.
    pub fn end(&self) -> IpAddr {
        self.end
    }
}

impl From<TableSasIpRange> for String {
    fn from(ip_range: TableSasIpRange) -> Self {
        format!(
            "{}-{}",
            ip_range.start.to_string(),
            ip_range.end.to_string()
        )
    }
}

impl FromStr for TableSasIpRange {
    type Err = SasError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once('-') {
            Ok(TableSasIpRange::new(
                IpAddr::from_str(start).map_err(|_| SasError::SasParsingError {
                    field: "".into(),
                    input: "".into(),
                })?,
                IpAddr::from_str(end).map_err(|_| SasError::SasParsingError {
                    field: "".into(),
                    input: "".into(),
                })?,
            ))
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::authorization::sas_token::options::table_sas_ip_range::TableSasIpRange;
    use std::str::FromStr;

    #[test]
    fn creation_test() {
        let ip_range_from_str = TableSasIpRange::from_str("127.0.0.1-127.0.0.15");
        let ip_range_new = TableSasIpRange::new([127, 0, 0, 1], [127, 0, 0, 15]);
        assert_eq!(Ok(ip_range_new), ip_range_from_str);
    }
}
