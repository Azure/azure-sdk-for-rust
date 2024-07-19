// Copyright (c) Microsoft Corp. All Rights Reserved.

pub fn setup() {
    println!("Setting up tests...");
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}
