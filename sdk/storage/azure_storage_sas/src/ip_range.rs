// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{fmt, net::IpAddr};

/// An IP address or range allowed for a SAS request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SasIpRange {
    /// A single IP address.
    Address(IpAddr),
    /// An inclusive range of IP addresses.
    Range { start: IpAddr, end: IpAddr },
}

impl fmt::Display for SasIpRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(addr) => write!(f, "{addr}"),
            Self::Range { start, end } => write!(f, "{start}-{end}"),
        }
    }
}
