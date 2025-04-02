// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

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
