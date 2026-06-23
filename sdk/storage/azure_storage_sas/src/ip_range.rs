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

impl fmt::Display for SasIpRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(addr) => write!(f, "{addr}"),
            Self::InclusiveRange { start, end } => write!(f, "{start}-{end}"),
        }
    }
}
