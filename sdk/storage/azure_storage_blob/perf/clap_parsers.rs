// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::num::NonZero;

pub fn non_zero_usize(s: &str) -> Result<NonZero<usize>, String> {
    s.parse::<usize>()
        .map_err(|e| format!("Failed to parse '{}' as usize: {}", s, e))
        .and_then(|n| {
            NonZero::new(n).ok_or_else(|| {
                format!("Value must be a non-zero positive integer, but got '{}'", s)
            })
        })
}
