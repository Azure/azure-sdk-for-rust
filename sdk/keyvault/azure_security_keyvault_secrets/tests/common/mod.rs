// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use rand::distributions::{Alphanumeric, DistString};

pub fn setup() {
    // Setup code here
}

pub fn create_random_name(prefix: Option<dyn Into<String>>, length: usize) -> String {
    let mut random_string: String = prefix.unwrap_or_default();

    random_string = random_string + &Alphanumeric.sample_string(&mut rand::thread_rng(), length);

    return random_string;
}
