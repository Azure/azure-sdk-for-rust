// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use clap::{ArgAction, Parser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use azure_core_test::proxy;

    // cspell:ignore ECANCELED ECHILD
    const ECANCELED: i32 = 4;
    const ECHILD: i32 = 5;

    let args = Args::parse();

    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(args.trace_level().into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let mut proxy = proxy::start(None, env!("CARGO_MANIFEST_DIR"), Some(args.into())).await?;

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
    /// Bind to any available port.
    ///
    /// If not set, only port `5000` will be tried.
    /// You can start the `test-proxy` service this way
    /// and pass `PROXY_MANUAL_START=true` when running `cargo test` to easily view trace information.
    #[arg(long)]
    auto: bool,

    /// Allow insecure upstream SSL certs.
    #[arg(long)]
    insecure: bool,

    /// Number of seconds to automatically shut down when no activity.
    #[arg(long, default_value_t = 300)]
    auto_shutdown_in_seconds: u32,

    /// Enable verbose logging.
    ///
    /// Trace level `INFO` is used by default.
    /// Pass `-v` to enable `DEBUG` level tracing,
    /// or `-vv` to enable `TRACE` level tracing.
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
}

impl Args {
    fn trace_level(&self) -> tracing::level_filters::LevelFilter {
        use tracing::level_filters::LevelFilter;
        match self.verbose {
            0 => LevelFilter::INFO,
            1 => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE,
        }
    }
}

impl From<Args> for azure_core_test::proxy::ProxyOptions {
    fn from(args: Args) -> Self {
        Self {
            auto: args.auto,
            insecure: args.insecure,
            auto_shutdown_in_seconds: args.auto_shutdown_in_seconds,
        }
    }
}
