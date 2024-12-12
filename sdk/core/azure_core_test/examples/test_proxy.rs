// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use clap::Parser;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use azure_core_test::proxy;

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

    let code = tokio::select! {
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

#[cfg(not(target_arch = "wasm32"))]
impl Args {
    fn trace_level(&self) -> tracing::level_filters::LevelFilter {
        if self.verbose {
            return tracing::level_filters::LevelFilter::DEBUG;
        }
        tracing::level_filters::LevelFilter::INFO
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Args> for azure_core_test::proxy::ProxyOptions {
    fn from(args: Args) -> Self {
        Self {
            insecure: args.insecure,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let _ = Args::parse();
    println!("wasm32 target architecture not supported");
}
