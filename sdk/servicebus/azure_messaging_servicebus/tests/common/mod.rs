// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use std::{env, error::Error};

#[allow(dead_code)]
static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

#[allow(dead_code)]
pub fn setup() {
    INIT_LOGGING.call_once(|| {
        println!("Setting up test logger...");

        use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_ansi(std::env::var("NO_COLOR").map_or(true, |v| v.is_empty()))
            .with_writer(std::io::stderr)
            .init();
    });
}

#[allow(dead_code)]
pub fn get_connection_string() -> Result<String, Box<dyn Error>> {
    env::var("SERVICEBUS_CONNECTION_STRING")
        .map_err(|_| "SERVICEBUS_CONNECTION_STRING environment variable not set".into())
}

#[allow(dead_code)]
pub fn get_queue_name() -> Result<String, Box<dyn Error>> {
    env::var("SERVICEBUS_QUEUE_NAME")
        .map_err(|_| "SERVICEBUS_QUEUE_NAME environment variable not set".into())
}

#[allow(dead_code)]
pub fn get_servicebus_namespace() -> Result<String, Box<dyn Error>> {
    env::var("SERVICEBUS_NAMESPACE")
        .map_err(|_| "SERVICEBUS_NAMESPACE environment variable not set".into())
}

#[allow(dead_code)]
pub fn get_topic_name() -> Result<String, Box<dyn Error>> {
    env::var("SERVICEBUS_TOPIC_NAME")
        .map_err(|_| "SERVICEBUS_TOPIC_NAME environment variable not set".into())
}

#[allow(dead_code)]
pub fn get_subscription_name() -> Result<String, Box<dyn Error>> {
    env::var("SERVICEBUS_SUBSCRIPTION_NAME")
        .map_err(|_| "SERVICEBUS_SUBSCRIPTION_NAME environment variable not set".into())
}
