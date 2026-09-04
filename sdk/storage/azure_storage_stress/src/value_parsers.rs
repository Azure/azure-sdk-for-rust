// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::time::Duration;

/// Parses an integer and converts it into a [Duration] of that many seconds.
pub fn duration_from_seconds(s: &str) -> Result<Duration, String> {
    Ok(Duration::from_secs(
        s.parse::<u64>().map_err(map_int_parse)?,
    ))
}

fn map_int_parse(e: std::num::ParseIntError) -> String {
    e.to_string()
}
