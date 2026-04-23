// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

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
