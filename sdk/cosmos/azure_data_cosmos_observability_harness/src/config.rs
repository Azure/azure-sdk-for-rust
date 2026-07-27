// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Command-line / environment configuration for the observability soak harness.

use clap::{Parser, ValueEnum};

/// The Cosmos DB emulator's well-known master key. Used as the default account
/// key so the harness runs against a locally started emulator with no extra
/// configuration.
pub const EMULATOR_MASTER_KEY: &str =
    "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";

/// Default emulator endpoint.
pub const DEFAULT_EMULATOR_ENDPOINT: &str = "https://localhost:8081";

/// Soak/load harness for the Azure Cosmos DB Rust SDK observability layer.
///
/// Registers the built-in metrics, distributed-tracing, and sampled-logging
/// diagnostics handlers, exports OpenTelemetry to stdout or an OTLP collector,
/// and drives a configurable read/write/query workload — optionally with fault
/// injection — so the diagnostics layer can be validated end-to-end.
#[derive(Debug, Parser)]
#[command(name = "cosmos-observability-harness", version, about)]
pub struct Config {
    // --- Target account ---------------------------------------------------
    /// Cosmos DB account endpoint. Defaults to the local emulator.
    #[arg(long, env = "AZURE_COSMOS_ENDPOINT", default_value = DEFAULT_EMULATOR_ENDPOINT)]
    pub endpoint: String,

    /// Account key (key auth). Defaults to the emulator's well-known key when
    /// unset and `--auth key` is used.
    #[arg(long, env = "AZURE_COSMOS_KEY")]
    pub key: Option<String>,

    /// Full connection string (`AccountEndpoint=...;AccountKey=...;`). When set,
    /// it overrides `--endpoint`/`--key`. The literal value `emulator` expands
    /// to the local emulator endpoint + well-known key.
    #[arg(long, env = "AZURE_COSMOS_CONNECTION_STRING")]
    pub connection_string: Option<String>,

    /// Authentication method.
    #[arg(long, value_enum, default_value_t = AuthMethod::Key)]
    pub auth: AuthMethod,

    /// Application region used for proximity-based routing.
    #[arg(long, env = "AZURE_COSMOS_REGION", default_value = "West US")]
    pub region: String,

    /// Treat the target as the emulator (relaxes TLS certificate validation).
    /// Auto-enabled when the endpoint host is `localhost`/`127.0.0.1`. For a
    /// custom (non-localhost) emulator host the harness exports
    /// `AZURE_COSMOS_EMULATOR_HOST` so the SDK actually relaxes validation for
    /// that host.
    #[arg(long, default_value_t = false)]
    pub emulator: bool,

    /// Database name (created if missing).
    #[arg(long, default_value = "observability_soak")]
    pub database: String,

    /// Container name (created if missing).
    #[arg(long, default_value = "items")]
    pub container: String,

    /// Provisioned throughput (RU/s) for the container when it is created.
    #[arg(long, default_value_t = 400)]
    pub throughput: usize,

    // --- Workload ---------------------------------------------------------
    /// Number of documents to seed before the load loop starts.
    #[arg(long, default_value_t = 100)]
    pub seed_count: usize,

    /// Number of concurrent worker tasks.
    #[arg(long, default_value_t = 8)]
    pub concurrency: usize,

    /// Target aggregate requests per second across all workers. `0` (default)
    /// means run as fast as possible (closed loop).
    #[arg(long, default_value_t = 0.0)]
    pub rps: f64,

    /// How long to run, in seconds. `0` (default) runs until Ctrl+C.
    #[arg(long, default_value_t = 0)]
    pub duration_secs: u64,

    /// Relative weight of point reads in the operation mix.
    #[arg(long, default_value_t = 70)]
    pub read_weight: u32,

    /// Relative weight of writes (upserts) in the operation mix.
    #[arg(long, default_value_t = 20)]
    pub write_weight: u32,

    /// Relative weight of single-partition queries in the operation mix.
    #[arg(long, default_value_t = 10)]
    pub query_weight: u32,

    /// Interval, in seconds, between console progress reports.
    #[arg(long, default_value_t = 10)]
    pub report_interval_secs: u64,

    // --- Telemetry --------------------------------------------------------
    /// OpenTelemetry exporter to install.
    #[arg(long, value_enum, default_value_t = Exporter::Stdout)]
    pub exporter: Exporter,

    /// OTLP gRPC endpoint (used when `--exporter otlp`).
    #[arg(long, default_value = "http://localhost:4317")]
    pub otlp_endpoint: String,

    /// Interval, in seconds, at which metrics are exported.
    #[arg(long, default_value_t = 15)]
    pub metric_export_interval_secs: u64,

    /// Emit the optional development-tier metrics (request charge, returned
    /// rows) and the extended attribute set from the metrics handler.
    #[arg(long, default_value_t = false)]
    pub extended_metrics: bool,

    // --- Fault injection --------------------------------------------------
    /// Probability (0.0–1.0) that a matching request is faulted. `0` disables
    /// fault injection. Requires the `fault_injection` feature.
    #[arg(long, default_value_t = 0.0)]
    pub fault_probability: f64,

    /// Extra server-side delay, in milliseconds, applied to faulted requests
    /// (produces slow operations that breach latency thresholds).
    #[arg(long, default_value_t = 0)]
    pub fault_delay_ms: u64,

    /// Error injected when a request is faulted.
    #[arg(long, value_enum, default_value_t = FaultError::ServiceUnavailable)]
    pub fault_error: FaultError,

    /// Which operations fault injection applies to.
    #[arg(long, value_enum, default_value_t = FaultOp::All)]
    pub fault_operation: FaultOp,

    /// Delay, in seconds, from the start of the load loop before fault injection
    /// becomes active (setup and seeding always run fault-free). `0` (default)
    /// faults from the first load operation. Combine with `--fault-duration-secs`
    /// to inject a bounded fault window partway through a long steady-state soak.
    /// Requires the `fault_injection` feature.
    #[arg(long, default_value_t = 0)]
    pub fault_start_secs: u64,

    /// How long, in seconds, the fault window stays active once it begins. `0`
    /// (default) leaves faults active for the remainder of the run. Requires the
    /// `fault_injection` feature.
    #[arg(long, default_value_t = 0)]
    pub fault_duration_secs: u64,
}

/// Authentication method for the Cosmos client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum AuthMethod {
    /// Shared account key.
    Key,
    /// Microsoft Entra ID (developer tools credential chain).
    Aad,
}

/// Selectable OpenTelemetry exporter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Exporter {
    /// Print spans and metrics to stdout (no infrastructure required).
    Stdout,
    /// Export via OTLP/gRPC to a local collector (requires the `otlp` feature).
    Otlp,
    /// Install no exporter; only the sampled `tracing` logs are emitted.
    None,
}

/// Error type to inject under fault injection. Mirrors the driver's
/// `FaultInjectionErrorType` variants the harness exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FaultError {
    /// 503 Service Unavailable.
    ServiceUnavailable,
    /// 429 Too Many Requests (throttling).
    TooManyRequests,
    /// 500 Internal Server Error.
    InternalServerError,
    /// 408 Request Timeout.
    Timeout,
    /// 449 Retry With.
    RetryWith,
    /// Transport-level connection failure.
    ConnectionError,
}

/// Which operation types fault injection targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FaultOp {
    /// Apply to every operation type.
    All,
    /// Apply only to point reads.
    Read,
    /// Apply only to writes (upserts).
    Write,
    /// Apply only to queries.
    Query,
}

impl Config {
    /// Resolves the effective endpoint and (for key auth) account key, taking
    /// the connection string into account when supplied.
    pub fn resolve_endpoint_and_key(&self) -> Result<(String, Option<String>), String> {
        if let Some(cs) = &self.connection_string {
            let cs = if cs.eq_ignore_ascii_case("emulator") {
                format!(
                    "AccountEndpoint={DEFAULT_EMULATOR_ENDPOINT};AccountKey={EMULATOR_MASTER_KEY};"
                )
            } else {
                cs.clone()
            };
            let (endpoint, key) = parse_connection_string(&cs)?;
            return Ok((endpoint, Some(key)));
        }

        let key = match self.auth {
            AuthMethod::Key => match &self.key {
                Some(k) => Some(k.clone()),
                // Only fall back to the emulator's well-known key for an actual
                // emulator endpoint; for a real account, surface a clear
                // missing-key error rather than a downstream 401.
                None if self.is_emulator(&self.endpoint) => Some(EMULATOR_MASTER_KEY.to_string()),
                None => {
                    return Err(
                        "key authentication requires --key (or AZURE_COSMOS_KEY) for a \
                                non-emulator endpoint; use --auth aad or --connection-string \
                                otherwise"
                            .into(),
                    );
                }
            },
            AuthMethod::Aad => None,
        };
        Ok((self.endpoint.clone(), key))
    }

    /// Whether the target should be treated as the emulator (relaxed TLS).
    pub fn is_emulator(&self, endpoint: &str) -> bool {
        self.emulator || host_is_local(endpoint)
    }

    /// Validates cross-field invariants, returning a human-readable error.
    pub fn validate(&self) -> Result<(), String> {
        if self.concurrency == 0 {
            return Err("--concurrency must be at least 1".into());
        }
        // Cosmos DB's manual-throughput floor is 400 RU/s; a lower value passes
        // here but fails only when the container is created.
        if self.throughput < 400 {
            return Err("--throughput must be at least 400 RU/s (Cosmos DB manual minimum)".into());
        }
        // Sum in `u64` so extreme (abuse-case) weights cannot overflow `u32`.
        if u64::from(self.read_weight) + u64::from(self.write_weight) + u64::from(self.query_weight)
            == 0
        {
            return Err("at least one operation weight must be greater than 0".into());
        }
        if self.seed_count == 0 {
            return Err("--seed-count must be at least 1".into());
        }
        if !(0.0..=1.0).contains(&self.fault_probability) {
            return Err("--fault-probability must be between 0.0 and 1.0".into());
        }
        if self.rps < 0.0 || !self.rps.is_finite() {
            return Err("--rps must be a finite, non-negative number".into());
        }

        // Reject configurations that request behavior this build cannot provide,
        // rather than silently accepting flags that are compiled out.
        if !cfg!(feature = "fault_injection")
            && (self.fault_probability > 0.0
                || self.fault_start_secs > 0
                || self.fault_duration_secs > 0
                || self.fault_delay_ms > 0
                || self.fault_operation != FaultOp::All)
        {
            return Err(
                "fault injection was requested (--fault-* flags) but this build was \
                        compiled without the `fault_injection` feature; rebuild with \
                        `--features fault_injection` (on by default)"
                    .into(),
            );
        }
        if !cfg!(feature = "metrics") && self.extended_metrics {
            return Err(
                "--extended-metrics was requested but this build was compiled without \
                        the `metrics` feature; rebuild with `--features metrics` (on by default)"
                    .into(),
            );
        }

        // Fault window / delay / operation knobs only take effect when a fault
        // probability is set; reject the combination instead of silently doing
        // nothing. Only meaningful when the feature is present — otherwise the
        // check above already rejects the flags.
        if cfg!(feature = "fault_injection")
            && self.fault_probability == 0.0
            && (self.fault_start_secs > 0
                || self.fault_duration_secs > 0
                || self.fault_delay_ms > 0
                || self.fault_operation != FaultOp::All)
        {
            return Err(
                "fault options (--fault-start-secs / --fault-duration-secs / --fault-delay-ms \
                        / --fault-operation) require a non-zero --fault-probability"
                    .into(),
            );
        }

        // When faults *are* requested, reject configurations whose window can
        // never open or whose target operation is never issued — both would run
        // a fault soak that silently injects nothing.
        if cfg!(feature = "fault_injection") && self.fault_probability > 0.0 {
            if self.duration_secs > 0 && self.fault_start_secs >= self.duration_secs {
                return Err(
                    "--fault-start-secs must be less than --duration-secs, otherwise the fault \
                            window never opens before the run ends"
                        .into(),
                );
            }
            let target_weight_zero = match self.fault_operation {
                FaultOp::All => false,
                FaultOp::Read => self.read_weight == 0,
                FaultOp::Write => self.write_weight == 0,
                FaultOp::Query => self.query_weight == 0,
            };
            if target_weight_zero {
                return Err(
                    "--fault-operation targets an operation whose mix weight is 0, so no request \
                            will ever match the fault rule; raise the corresponding \
                            --read/--write/--query-weight or change --fault-operation"
                        .into(),
                );
            }
        }
        Ok(())
    }
}

/// Parses `AccountEndpoint=...;AccountKey=...;` into `(endpoint, key)`.
fn parse_connection_string(cs: &str) -> Result<(String, String), String> {
    let mut endpoint = None;
    let mut key = None;
    for part in cs.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let Some((name, value)) = part.split_once('=') else {
            continue;
        };
        match name.trim().to_ascii_lowercase().as_str() {
            "accountendpoint" => endpoint = Some(value.trim().to_string()),
            // AccountKey values contain base64 `=` padding, so re-join the tail.
            "accountkey" => key = Some(value.trim().to_string()),
            _ => {}
        }
    }
    match (endpoint, key) {
        (Some(e), Some(k)) => Ok((e, k)),
        _ => Err("connection string must contain AccountEndpoint and AccountKey".into()),
    }
}

/// Returns `true` when the endpoint host is a loopback address.
fn host_is_local(endpoint: &str) -> bool {
    endpoint.contains("localhost") || endpoint.contains("127.0.0.1") || endpoint.contains("[::1]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_connection_string_with_base64_padding() {
        let (endpoint, key) =
            parse_connection_string("AccountEndpoint=https://host:8081;AccountKey=abc==;").unwrap();
        assert_eq!(endpoint, "https://host:8081");
        // The trailing `==` padding must be preserved.
        assert_eq!(key, "abc==");
    }

    #[test]
    fn defaults_to_emulator_key_for_key_auth() {
        let config = Config::parse_from(["harness"]);
        let (endpoint, key) = config.resolve_endpoint_and_key().unwrap();
        assert_eq!(endpoint, DEFAULT_EMULATOR_ENDPOINT);
        assert_eq!(key.as_deref(), Some(EMULATOR_MASTER_KEY));
        assert!(config.is_emulator(&endpoint));
    }

    #[test]
    fn aad_auth_has_no_key() {
        let config = Config::parse_from(["harness", "--auth", "aad"]);
        let (_endpoint, key) = config.resolve_endpoint_and_key().unwrap();
        assert!(key.is_none());
    }

    #[test]
    fn rejects_zero_weight_mix() {
        let config = Config::parse_from([
            "harness",
            "--read-weight",
            "0",
            "--write-weight",
            "0",
            "--query-weight",
            "0",
        ]);
        assert!(config.validate().is_err());
    }

    #[test]
    fn accepts_extreme_weights_without_overflow() {
        let max = u32::MAX.to_string();
        let config = Config::parse_from([
            "harness",
            "--read-weight",
            &max,
            "--write-weight",
            &max,
            "--query-weight",
            &max,
        ]);
        // The weight sum is computed in u64, so the maximum u32 values on every
        // arm must validate cleanly rather than overflowing.
        assert!(config.validate().is_ok());
    }

    #[test]
    fn rejects_fault_window_without_probability() {
        // A fault window with the default (zero) probability would silently
        // inject nothing; reject it instead.
        let config = Config::parse_from(["harness", "--fault-start-secs", "60"]);
        assert!(config
            .validate()
            .unwrap_err()
            .contains("require a non-zero --fault-probability"));
    }

    #[test]
    fn rejects_fault_window_that_never_opens() {
        // fault-start-secs >= duration-secs means the window never opens.
        let config = Config::parse_from([
            "harness",
            "--fault-probability",
            "0.2",
            "--duration-secs",
            "60",
            "--fault-start-secs",
            "60",
        ]);
        assert!(config
            .validate()
            .unwrap_err()
            .contains("must be less than --duration-secs"));
    }

    #[test]
    fn rejects_fault_operation_with_zero_target_weight() {
        // Faulting writes while the write weight is 0 can never match a request.
        let config = Config::parse_from([
            "harness",
            "--fault-probability",
            "0.2",
            "--fault-operation",
            "write",
            "--write-weight",
            "0",
        ]);
        assert!(config.validate().unwrap_err().contains("mix weight is 0"));
    }

    #[test]
    fn accepts_valid_fault_window() {
        let config = Config::parse_from([
            "harness",
            "--fault-probability",
            "0.2",
            "--duration-secs",
            "120",
            "--fault-start-secs",
            "30",
            "--fault-duration-secs",
            "30",
        ]);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn key_auth_requires_key_for_non_emulator_endpoint() {
        let config = Config::parse_from([
            "harness",
            "--endpoint",
            "https://myacct.documents.azure.com:443/",
        ]);
        // No --key against a real endpoint must error rather than silently using
        // the emulator's public master key (which would yield a confusing 401).
        assert!(config.resolve_endpoint_and_key().is_err());
    }

    #[test]
    fn rejects_non_finite_rps() {
        let config = Config::parse_from(["harness", "--rps", "inf"]);
        assert!(config.validate().unwrap_err().contains("finite"));
    }

    #[test]
    fn rejects_throughput_below_minimum() {
        let config = Config::parse_from(["harness", "--throughput", "100"]);
        assert!(config.validate().unwrap_err().contains("400 RU/s"));
    }
}
