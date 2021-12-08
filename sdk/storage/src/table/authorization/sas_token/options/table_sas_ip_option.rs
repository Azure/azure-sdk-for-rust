use azure_core::SasError;
use std::{net::IpAddr, str::FromStr};

/// Specifies an IP address or a range of IP addresses from which to accept requests.
/// When specifying a range, note that the range is inclusive.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum TableSasIpOption {
    Single(IpAddr),
    Range {
        /// The start of the IP range.
        start: IpAddr,
        /// The optional end of the IP range.
        end: IpAddr,
    },
}

impl TableSasIpOption {
    pub fn new_single(ip: impl Into<IpAddr>) -> Self {
        TableSasIpOption::Single(ip.into())
    }

    pub fn new_range(start: impl Into<IpAddr>, end: impl Into<IpAddr>) -> Self {
        TableSasIpOption::Range {
            start: start.into(),
            end: end.into(),
        }
    }
}

impl FromStr for TableSasIpOption {
    type Err = SasError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once('-') {
            Ok(TableSasIpOption::new_range(
                IpAddr::from_str(start).map_err(|_| SasError::SasParsingError {
                    field: "start".into(),
                    input: start.into(),
                })?,
                IpAddr::from_str(end).map_err(|_| SasError::SasParsingError {
                    field: "end".into(),
                    input: end.into(),
                })?,
            ))
        } else {
            Ok(TableSasIpOption::Single(IpAddr::from_str(s).map_err(
                |_| SasError::SasParsingError {
                    field: "single ip".into(),
                    input: s.into(),
                },
            )?))
        }
    }
}

impl From<TableSasIpOption> for String {
    fn from(ip_option: TableSasIpOption) -> Self {
        match ip_option {
            TableSasIpOption::Single(ip) => ip.to_string(),
            TableSasIpOption::Range { start, end } => {
                format!("{}-{}", start.to_string(), end.to_string())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::TableSasIpOption;
    use std::str::FromStr;
    #[test]
    fn creation_test() {
        let ip = TableSasIpOption::new_range([127, 0, 0, 1], [127, 0, 0, 15]);
        let ip_from_str = TableSasIpOption::from_str("127.0.0.1-127.0.0.15").unwrap();
        assert_eq!(ip, ip_from_str);

        let ip_from_str = TableSasIpOption::from_str("127.0.0.1").unwrap();
        let ip = TableSasIpOption::new_single([127, 0, 0, 1]);
        assert_eq!(ip, ip_from_str);
    }
}
