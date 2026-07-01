// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::fmt;

/// The protocol permitted for a SAS request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SasProtocol {
    /// Only HTTPS is permitted.
    Https,
    /// Both HTTPS and HTTP are permitted.
    HttpsAndHttp,
}

impl fmt::Display for SasProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Https => f.write_str("https"),
            Self::HttpsAndHttp => f.write_str("https,http"),
        }
    }
}
