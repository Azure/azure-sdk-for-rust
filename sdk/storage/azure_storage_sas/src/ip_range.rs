// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{fmt, net::Ipv4Addr};

/// An IPv4 address or range allowed for a SAS request.
///
/// Azure Storage only supports IPv4 addresses for the `sip` field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SasIpRange {
    /// A single IPv4 address.
    Address(Ipv4Addr),
    /// An inclusive range of IPv4 addresses.
    InclusiveRange { start: Ipv4Addr, end: Ipv4Addr },
}

impl SasIpRange {
    /// Serializes the range to the value expected by the SAS `sip` field.
    ///
    /// A range is encoded as `start-end` (hyphen-separated), which is the
    /// format Azure Storage requires.
    pub(crate) fn sip_value(&self) -> String {
        match self {
            Self::Address(addr) => addr.to_string(),
            Self::InclusiveRange { start, end } => format!("{start}-{end}"),
        }
    }
}

impl fmt::Display for SasIpRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(addr) => write!(f, "{addr}"),
            Self::InclusiveRange { start, end } => write!(f, "{start}..={end}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sip_value_uses_hyphen_for_range() {
        let range = SasIpRange::InclusiveRange {
            start: Ipv4Addr::new(10, 0, 0, 1),
            end: Ipv4Addr::new(10, 0, 0, 255),
        };
        assert_eq!(range.sip_value(), "10.0.0.1-10.0.0.255");
    }

    #[test]
    fn sip_value_single_address() {
        let addr = SasIpRange::Address(Ipv4Addr::new(1, 2, 3, 4));
        assert_eq!(addr.sip_value(), "1.2.3.4");
    }

    #[test]
    fn display_uses_idiomatic_range_syntax() {
        let range = SasIpRange::InclusiveRange {
            start: Ipv4Addr::new(10, 0, 0, 1),
            end: Ipv4Addr::new(10, 0, 0, 255),
        };
        assert_eq!(range.to_string(), "10.0.0.1..=10.0.0.255");
        assert_eq!(
            SasIpRange::Address(Ipv4Addr::new(1, 2, 3, 4)).to_string(),
            "1.2.3.4"
        );
    }
}
