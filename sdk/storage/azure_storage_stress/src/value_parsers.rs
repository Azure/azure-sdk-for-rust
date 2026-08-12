// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{num::NonZero, time::Duration};

const POSITIVE_VALUE: &str = "Value must be positive.";

pub fn non_zero_usize(s: &str) -> Result<NonZero<usize>, String> {
    let num: usize = s.parse().map_err(map_int_parse)?;
    NonZero::new(num).ok_or_else(|| POSITIVE_VALUE.to_string())
}

pub fn non_zero_u32(s: &str) -> Result<NonZero<u32>, String> {
    let num: u32 = s.parse().map_err(map_int_parse)?;
    NonZero::new(num).ok_or_else(|| POSITIVE_VALUE.to_string())
}

pub fn non_zero_u64(s: &str) -> Result<NonZero<u64>, String> {
    let num: u64 = s.parse().map_err(map_int_parse)?;
    NonZero::new(num).ok_or_else(|| POSITIVE_VALUE.to_string())
}

/// Parses an integer and converts it into a [Duration] of that many seconds.
pub fn duration_from_seconds(s: &str) -> Result<Duration, String> {
    Ok(Duration::from_secs(
        s.parse::<u64>().map_err(map_int_parse)?,
    ))
}

fn map_int_parse(e: std::num::ParseIntError) -> String {
    e.to_string()
}
