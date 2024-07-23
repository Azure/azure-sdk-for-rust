// Copyright (c) Microsoft Corp. All Rights Reserved.

pub fn setup() {
    println!("Setting up tests...");
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
    if subscriber.is_err() {
        println!("Failed to set up tracing: {:?}", subscriber.err());
    }
}
