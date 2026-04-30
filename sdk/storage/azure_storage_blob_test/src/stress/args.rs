// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::time::Duration;

use clap::{Args, Parser};

use crate::{
    fault_injection::FaultInjectionProbabilities,
    stress::{value_parsers::simple_duration, StressTest, StressTestFactory},
};

#[derive(Debug, Clone, Parser)]
pub struct StressRunnerOptions<T: StressTestFactory> {
    /// Parallel operations to run.
    #[arg(long, default_value_t = 1)]
    pub parallel: usize,

    /// Duration of the stress test, excluding setup and cleanup.
    #[arg(long, default_value = "10", value_parser = simple_duration, value_name = "SECONDS")]
    pub duration: Duration,

    /// Optional timeout for one-time test setup.
    #[arg(long, value_parser = simple_duration, value_name = "SECONDS")]
    pub setup_timeout: Option<Duration>,

    /// Optional timeout for individual operations during the test.
    #[arg(long, value_parser = simple_duration, value_name = "SECONDS")]
    pub operation_timeout: Option<Duration>,

    /// Optional timeout for one-time test cleanup.
    #[arg(long, value_parser = simple_duration, value_name = "SECONDS")]
    pub cleanup_timeout: Option<Duration>,

    /// Path to a json config file for fault injection.
    #[arg(long = "fault-config", value_name = "FILE", group = "fault injection")]
    pub fault_injection_file: Option<String>,

    /// Use a default configuration for fault injection.
    #[arg(long = "fault-standard", group = "fault injection")]
    pub use_default_fault_injection: bool,

    #[command(flatten)]
    pub fault_injection_inline_overrides: FaultInjectionOverrideOptions,

    #[command(subcommand)]
    pub command: T,
}

impl<T: StressTestFactory> StressRunnerOptions<T> {
    pub fn build_test(&self) -> azure_core::Result<Box<dyn StressTest>> {
        T::build_test(self)
    }

    pub fn fault_options(&self) -> &FaultInjectionProbabilities {
        todo!()
    }
}

impl<T: StressTestFactory> std::fmt::Display for StressRunnerOptions<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Stress Runner Configuration ===")?;
        writeln!(f, "duration: {}", self.duration.as_secs())?;
        writeln!(f, "parallel: {}", self.parallel)?;
        writeln!(
            f,
            "operation_timeout: {:?}",
            self.operation_timeout.map(|t| t.as_secs())
        )?;
        writeln!(
            f,
            "setup_timeout: {:?}",
            self.setup_timeout.map(|t| t.as_secs())
        )?;
        writeln!(
            f,
            "cleanup_timeout: {:?}",
            self.cleanup_timeout.map(|t| t.as_secs())
        )?;
        std::fmt::Display::fmt(&self.command, f)
    }
}

#[derive(Args, serde::Deserialize, Clone, Debug)]
#[group(required = false, multiple = true)]
pub struct FaultInjectionOverrideOptions {
    /// Override probability for a partial response then hang.
    #[arg(long = "fault-p", value_name = "PROBABILITY")]
    pub partial_response_hang: Option<f32>,

    /// Override probability for a partial response then close (TCP FIN).
    #[arg(long = "fault-pc", value_name = "PROBABILITY")]
    pub partial_response_close: Option<f32>,

    /// Override probability for a partial response then abort (TCP RST).
    #[arg(long = "fault-pa", value_name = "PROBABILITY")]
    pub partial_response_abort: Option<f32>,

    /// Override probability for a partial response then graceful finish.
    #[arg(long = "fault-pn", value_name = "PROBABILITY")]
    pub partial_response_normal: Option<f32>,

    /// Override probability for no response then hang.
    #[arg(long = "fault-n", value_name = "PROBABILITY")]
    pub no_response_hang: Option<f32>,

    /// Override probability for no response close (TCP FIN)
    #[arg(long = "fault-nc", value_name = "PROBABILITY")]
    pub no_response_close: Option<f32>,

    /// Override probability for no response then abort (TCP RST).
    #[arg(long = "fault-na", value_name = "PROBABILITY")]
    pub no_response_abort: Option<f32>,
}

impl From<FaultInjectionOverrideOptions> for crate::fault_injection::FaultInjectionProbabilities {
    fn from(value: FaultInjectionOverrideOptions) -> Self {
        Self {
            partial_response_hang: value.partial_response_hang.unwrap_or_default(),
            partial_response_close: value.partial_response_close.unwrap_or_default(),
            partial_response_abort: value.partial_response_abort.unwrap_or_default(),
            partial_response_normal: value.partial_response_normal.unwrap_or_default(),
            no_response_hang: value.no_response_hang.unwrap_or_default(),
            no_response_close: value.no_response_close.unwrap_or_default(),
            no_response_abort: value.no_response_abort.unwrap_or_default(),
        }
    }
}
