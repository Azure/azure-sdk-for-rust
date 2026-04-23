// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub mod data;
pub mod value_parsers;

use azure_core::{
    async_runtime::get_async_runtime,
    error::{ErrorKind, ResultExt},
    Error,
};
use clap::{Parser, Subcommand};
use futures::{
    channel::mpsc::{self, UnboundedSender},
    future,
};
use serde::Serialize;
use std::{fmt::Debug, mem};

pub type Result<T> = std::result::Result<T, Error>;

/// A [Subcommand] specifier capable of instancing the code for the selected subcommand.
pub trait StressTestFactory: Subcommand + Debug + std::fmt::Display {
    fn build_test(&self) -> Result<Box<dyn StressTest>>;
}

#[async_trait::async_trait]
pub trait StressTest: Send + Sync {
    async fn global_setup(&self) -> Result<()>;
    async fn get_operation(&self) -> Result<Box<dyn StressTestOperation>>;
    async fn global_cleanup(&self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait StressTestOperation: Send + Sync {
    async fn run(&mut self, result_sender: UnboundedSender<StressRunOutput>);
}

#[derive(Debug, Clone, Parser)]
struct StressRunnerOptions<T: StressTestFactory> {
    /// Parallel operations to run.
    #[arg(long, default_value_t = 1)]
    parallel: usize,

    /// Run duration in seconds.
    #[arg(long, value_name = "SECONDS", default_value_t = 60)]
    duration: u64,

    /// Optional timeout in seconds for individual operations.
    #[arg(long, value_name = "SECONDS")]
    timeout: Option<u64>,

    // /// Results output destination.
    // #[arg(short = 'o', long = "output", value_name = "FILE")]
    // results_file: String,
    #[command(subcommand)]
    command: T,
}

impl<T: StressTestFactory> std::fmt::Display for StressRunnerOptions<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StressRunnerOptions: {{ parallel: {}, duration: {}, timeout: {:?} }}",
            self.parallel, self.duration, self.timeout,
        )
    }
}

#[derive(Debug, Clone, Default, Serialize)]
struct StressRunCounts {
    pub total_loops: usize,
    pub loops_success: usize,
    pub loops_graceful_error: usize,
    pub loops_timeout: usize,
    pub loops_panic: usize,
    pub loops_data_corruption: usize,
}

pub enum StressRunOutput {
    Success,
    GracefulError(Error),
    Timeout,
    Panic(String),
    DataCorruption,
}

/// Context information required by performance tests.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StressRunner<T: StressTestFactory> {
    options: StressRunnerOptions<T>,
    package_dir: &'static str,
    module_name: &'static str,
}

impl<T: StressTestFactory> StressRunner<T> {
    /// Construct a stress test runner using one of the provided `tests` based on [`env::args_os`].
    ///
    ///
    /// # Arguments
    ///
    /// * package_dir - The directory containing the package with the tests. Typically `env!("CARGO_PACKAGE_DIR")`
    /// * module_name - The name of the module containing the test, typically `file!()`
    /// * tests - A set of test definitions.
    ///
    pub fn new(package_dir: &'static str, module_name: &'static str) -> Self {
        let options = StressRunnerOptions::<T>::parse();
        Self {
            options,
            package_dir,
            module_name,
        }
    }

    /// Construct a stress test runner using one of the provided `tests` based on provided `args`.
    pub fn from_args(
        package_dir: &'static str,
        module_name: &'static str,
        args: Vec<&str>,
    ) -> azure_core::Result<Self> {
        let options = StressRunnerOptions::<T>::try_parse_from(args)
            .with_context(ErrorKind::Other, "Failed to parse command line arguments.")?;
        Ok(Self {
            options,
            package_dir,
            module_name,
        })
    }

    pub async fn run(&self) -> Result<()> {
        let stress_test = self.options.command.build_test()?;

        println!("Test Configuration: {:#}", self.options);

        let stress_run_result: Result<()> = async {
            println!("========== Starting global setup ==========");
            stress_test.global_setup().await?;

            let mut totals = StressRunCounts::default();
            let mut join_handles = Vec::with_capacity(self.options.parallel);
            let (tx, mut rx) = mpsc::unbounded();
            for iteration in 1.. {
                println!("========== Starting test run {} ==========", iteration);

                let mut operation = stress_test.get_operation().await?;
                let tx_clone = tx.clone();
                join_handles.push(get_async_runtime().spawn(Box::pin(async move {
                    operation.run(tx_clone).await;
                })));

                // block until free parallel slot
                while join_handles.len() >= self.options.parallel {
                    let join_result;
                    (join_result, _, join_handles) =
                        future::select_all(mem::take(&mut join_handles)).await;
                    if let Err(_join_error) = join_result {
                        todo!("Handle error joining task")
                    }
                }

                // non-blocking process run result(s)
                while let Ok(msg) = rx.try_recv() {
                    totals.total_loops += 1;
                    match msg {
                        StressRunOutput::Success => totals.loops_success += 1,
                        StressRunOutput::GracefulError(_error) => totals.loops_graceful_error += 1,
                        StressRunOutput::Timeout => totals.loops_timeout += 1,
                        StressRunOutput::Panic(_panic_msg) => totals.loops_panic += 1,
                        StressRunOutput::DataCorruption => totals.loops_data_corruption += 1,
                    }
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&totals).with_context(
                            ErrorKind::DataConversion,
                            "Failed to serialize test results to JSON.",
                        )?
                    );
                }
            }
            Ok(())
        }
        .await;
        if let Err(e) = stress_run_result {
            eprintln!("Stress runner failure. {:#}", e);
        }

        println!("========== Starting test cleanup ==========");
        stress_test.global_cleanup().await
    }
}

#[cfg(test)]
mod framework_tests;
