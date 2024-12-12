// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::proxy::{self, ProxyOptions};
use clap::Parser;
use tokio::select;
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cspell:ignore ECANCELED ECHILD
    const ECANCELED: i32 = 4;
    const ECHILD: i32 = 5;

    let args = Args::parse();

    tracing_subscriber::fmt()
        // Default trace level based on command line arguments.
        .with_max_level(args.trace_level())
        // RUST_LOG environment variable can override trace level.
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let mut proxy = proxy::start(env!("CARGO_MANIFEST_DIR"), Some(args.into())).await?;

    let code = select! {
        _ = tokio::signal::ctrl_c() => {
            // Try to shutdown the test-proxy.
            proxy.stop().await?;

            ECANCELED
        },
        v = proxy.wait() => {
            let code = v.map_or_else(|_| ECHILD, |v| v.code().unwrap_or_default());
            println!("test-proxy exited with status code {code}");

            code
        },
    };

    if code != 0 {
        std::process::exit(code);
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(about = "Starts the Test-Proxy service", version)]
struct Args {
    /// Allow insecure upstream SSL certs.
    #[arg(long)]
    insecure: bool,

    /// Enable verbose logging.
    #[arg(short, long)]
    verbose: bool,
}

impl Args {
    fn trace_level(&self) -> LevelFilter {
        if self.verbose {
            return LevelFilter::DEBUG;
        }
        LevelFilter::INFO
    }
}

impl From<Args> for ProxyOptions {
    fn from(args: Args) -> Self {
        Self {
            insecure: args.insecure,
        }
    }
}
