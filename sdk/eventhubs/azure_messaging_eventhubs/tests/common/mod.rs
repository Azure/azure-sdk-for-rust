// Copyright (c) Microsoft Corp. All Rights Reserved.

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

pub fn setup() {
    INIT_LOGGING.call_once(|| {
        println!("Setting up test logger...");

        tracing_subscriber::fmt::init();
    });
}
