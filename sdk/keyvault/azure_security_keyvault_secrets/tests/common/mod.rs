// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use rand::distributions::{Alphanumeric, DistString};

pub fn setup() {
    // Setup code here
}

pub fn create_random_name(prefix: String, length: usize) -> String {
    let random_string = prefix + &Alphanumeric.sample_string(&mut rand::thread_rng(), length);

    return random_string;
}
