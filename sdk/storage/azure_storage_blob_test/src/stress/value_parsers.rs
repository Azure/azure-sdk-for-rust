// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::time::Duration;

const POSITIVE_VALUE: &str = "Value must be positive.";

pub fn non_zero_usize(s: &str) -> Result<usize, String> {
    let num: usize = s.parse().map_err(|_| POSITIVE_VALUE)?;
    if num == 0 {
        Err(POSITIVE_VALUE.to_string())
    } else {
        Ok(num)
    }
}

pub fn non_zero_u32(s: &str) -> Result<u32, String> {
    let num: u32 = s.parse().map_err(|_| POSITIVE_VALUE)?;
    if num == 0 {
        Err(POSITIVE_VALUE.to_string())
    } else {
        Ok(num)
    }
}

pub fn non_zero_u64(s: &str) -> Result<u64, String> {
    let num: u64 = s.parse().map_err(|_| POSITIVE_VALUE)?;
    if num == 0 {
        Err(POSITIVE_VALUE.to_string())
    } else {
        Ok(num)
    }
}

/// Parses a decimal number with an optional suffix to indicate the time unit.
///
/// # Suffixes
/// - `s` - seconds. (Default)
/// - `m` - minutes.
/// - `h` - hours.
///
/// # Examples
/// - "10.5" - Duration of 10.5 seconds.
/// - "10s" - Duration of 10 seconds.
/// - "10.1m" - Duration of 10.1 minutes.
/// - "10ss" - Error.
/// - "10.2a" - Error.
pub fn simple_duration(s: &str) -> Result<Duration, String> {
    if let Some(s) = s.strip_suffix("s") {
        Ok(Duration::from_secs_f64(s.parse().map_err(map_float_parse)?))
    } else if let Some(s) = s.strip_suffix("m") {
        Ok(Duration::from_secs_f64(s.parse().map_err(map_float_parse)?))
    } else if let Some(s) = s.strip_suffix("h") {
        Ok(Duration::from_secs_f64(s.parse().map_err(map_float_parse)?))
    } else {
        Ok(Duration::from_secs_f64(s.parse().map_err(map_float_parse)?))
    }
}

/// Parses an integer with an optional suffix to indicate the binary SI unit.
///
/// # Suffixes
/// - `B` - bytes. (Default)
/// - `KiB` - kibibytes.
/// - `MiB` - mebibytes.
/// - `GiB` - gibibytes.
///
/// # Examples
/// - "10" - 10 bytes.
/// - "10B" - 10 bytes.
/// - "10KiB" - 10240 bytes.
/// - "10KB" - Error.
/// - "1.5GiB" - Error.
pub fn simple_len_u64(s: &str) -> Result<u64, String> {
    if let Some(s) = s.to_lowercase().strip_suffix("gib") {
        Ok(s.parse::<u64>().map_err(map_int_parse)? << 30)
    } else if let Some(s) = s.to_lowercase().strip_suffix("mib") {
        Ok(s.parse::<u64>().map_err(map_int_parse)? << 20)
    } else if let Some(s) = s.to_lowercase().strip_suffix("kib") {
        Ok(s.parse::<u64>().map_err(map_int_parse)? << 10)
    } else if let Some(s) = s.to_lowercase().strip_suffix("b") {
        Ok(s.parse::<u64>().map_err(map_int_parse)?)
    } else {
        Ok(s.parse::<u64>().map_err(map_int_parse)?)
    }
}

/// [simple_len_u64] but with usize.
pub fn simple_len_usize(s: &str) -> Result<usize, String> {
    if let Some(s) = s.to_lowercase().strip_suffix("gib") {
        Ok(s.parse::<usize>().map_err(map_int_parse)? << 30)
    } else if let Some(s) = s.to_lowercase().strip_suffix("mib") {
        Ok(s.parse::<usize>().map_err(map_int_parse)? << 20)
    } else if let Some(s) = s.to_lowercase().strip_suffix("kib") {
        Ok(s.parse::<usize>().map_err(map_int_parse)? << 10)
    } else if let Some(s) = s.to_lowercase().strip_suffix("b") {
        Ok(s.parse::<usize>().map_err(map_int_parse)?)
    } else {
        Ok(s.parse::<usize>().map_err(map_int_parse)?)
    }
}

/// [simple_len_u64] but rejects a zero value.
pub fn simple_non_zero_len_u64(s: &str) -> Result<u64, String> {
    let num = simple_len_u64(s)?;
    if num == 0 {
        Err(POSITIVE_VALUE.to_string())
    } else {
        Ok(num)
    }
}

/// [simple_len_usize] but rejects a zero value.
pub fn simple_non_zero_len_usize(s: &str) -> Result<usize, String> {
    let num = simple_len_usize(s)?;
    if num == 0 {
        Err(POSITIVE_VALUE.to_string())
    } else {
        Ok(num)
    }
}

fn map_float_parse(e: std::num::ParseFloatError) -> String {
    e.to_string()
}

fn map_int_parse(e: std::num::ParseIntError) -> String {
    e.to_string()
}
