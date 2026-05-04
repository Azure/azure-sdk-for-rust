// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub mod args;
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
use std::{fmt::Debug, future::Future, mem, pin::Pin, time::Duration};

use crate::{stress::args::StressRunnerOptions, OptionalTimeoutFutureExt};

pub type Result<T> = std::result::Result<T, Error>;

/// A [Subcommand] specifier capable of instancing the code for the selected subcommand.
pub trait StressTestFactory: Subcommand + Debug + std::fmt::Display {
    fn build_test(options: &StressRunnerOptions<Self>) -> Result<Box<dyn StressTest>>;
}

#[async_trait::async_trait]
pub trait StressTest: Send + Sync {
    /// One-time setup.
    async fn global_setup(&self) -> Result<()>;
    /// Gets an operation to be ran. Many operations can be run in parallel.
    async fn get_operation(&self) -> Result<Box<dyn StressTestOperation>>;
    /// One-time cleanup.
    async fn global_cleanup(&self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait StressTestOperation: Send + Sync {
    async fn run(
        &mut self,
        timeout: Option<Duration>,
        result_sender: UnboundedSender<StressRunOutput>,
    );
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
    /// The operation completed successfully.
    Success,

    /// The operation failed, but communicated this through [Result::Err].
    GracefulError(Error),

    /// The operation did not complete within the provided timeout.
    ///
    /// # Notes
    ///
    /// Operation timeout and reporting must be the responsibility of the [StressTestOperation] implementor.
    /// Since all operations are run in spawned async workers, their work cannot be stopped by
    /// dropping the join handle, which would lead to the worker reporting a result post-timeout regardless
    /// of any runner-reported result.
    Timeout,

    /// The operation panicked.
    ///
    /// # Notes
    ///
    /// Panic unwinding and reporting must be the responsibility of the [StressTestOperation] implementor.
    /// Unwind safety is not dyn-compatible. Since all operations are dyn in practice, the runner
    /// can never successfully [std::panic::catch_unwind] the dynamically resolved future.
    Panic(String),

    /// The operation completed successfully, but data integrity checks failed.
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
    /// Construct a stress test runner with [Subcommand] test factory T, configured through parsed
    /// arguments from [`std::env::args_os`].
    ///
    /// # Arguments
    ///
    /// * package_dir - The directory containing the package with the tests. Typically `env!("CARGO_PACKAGE_DIR")`
    /// * module_name - The name of the module containing the test, typically `file!()`
    pub fn new(package_dir: &'static str, module_name: &'static str) -> Self {
        let options = StressRunnerOptions::<T>::parse();
        Self {
            options,
            package_dir,
            module_name,
        }
    }

    /// Construct a stress test runner with [Subcommand] test factory T, configured through parsed
    /// arguments provided by the caller.
    ///
    /// # Arguments
    ///
    /// * package_dir - The directory containing the package with the tests. Typically `env!("CARGO_PACKAGE_DIR")`
    /// * module_name - The name of the module containing the test, typically `file!()`
    /// * args - The arguments to use for configuring this test run, emulating arguments parsed from the command line.
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
        let stress_test = self.options.build_test()?;
        let mut totals = StressRunCounts::default();

        println!("{}", self.options);

        // Catch all Err returns of setup and test run, ensuring we always run cleanup.
        let setup_and_run_result: Result<()> = async {
            println!("=== Global Setup ===");
            stress_test
                .global_setup()
                .timeout(self.options.setup_timeout)
                .await??;

            println!("=== Begin Stress ===");
            // Race an infinite loop of parallel tests against a timeout.
            // Note that each individual test is spawned into a different worker, and therefore
            // will NOT cease when the stress loop future is dropped.
            // This is acceptable, as the next steps are to execute test cleanup and exit application.
            // If the runs absolutely must be stopped, the [StressTest] implementor can signal to
            // individual test runs in global cleanup.
            match infinite_stress_loop(stress_test.as_ref(), &mut totals, &self.options)
                .timeout(Some(self.options.duration))
                .await
            {
                // Test duration completed. This is the expected path.
                Err(_timeout_error) => {}

                // Infinite run loop exited due to an error managing tests.
                Ok(stress_result) => match stress_result {
                    Ok(()) => Err(Error::with_message(
                        ErrorKind::Other,
                        "Infinite stress loop exited with success. This is a bug.",
                    ))?,
                    Err(e) => Err(e)?,
                },
            }

            Ok(())
        }
        .await;
        if let Err(e) = setup_and_run_result {
            eprintln!("Stress runner failure.");
            eprintln!("{e}");
            if let Ok(inner) = e.into_inner() {
                eprintln!("{inner}");
            }
        }

        println!(
            "Final results: {}",
            serde_json::to_string_pretty(&totals).with_context(
                ErrorKind::DataConversion,
                "Failed to serialize test results to JSON.",
            )?
        );

        println!("=== Begin Cleanup ===");
        stress_test
            .global_cleanup()
            .timeout(self.options.cleanup_timeout)
            .await?
    }
}

async fn infinite_stress_loop<T: StressTestFactory>(
    stress_test: &dyn StressTest,
    totals: &mut StressRunCounts,
    options: &StressRunnerOptions<T>,
) -> Result<()> {
    let mut join_handles = Vec::with_capacity(options.parallel);
    let (tx, mut rx) = mpsc::unbounded();

    for iteration in 1usize.. {
        println!("Start operation {}", iteration);

        join_handles.push(get_async_runtime().spawn(operation_wrapper(
            stress_test.get_operation().await?,
            options.operation_timeout,
            tx.clone(),
        )));

        // block until free parallel slot
        while join_handles.len() >= options.parallel {
            let join_result;
            (join_result, _, join_handles) = future::select_all(mem::take(&mut join_handles)).await;
            if let Err(join_error) = join_result {
                totals.loops_panic += 1;
                eprintln!("{}", join_error);
            }
        }

        // non-blocking process run result(s)
        while let Ok(msg) = rx.try_recv() {
            totals.total_loops += 1;
            match &msg {
                StressRunOutput::Success => totals.loops_success += 1,
                StressRunOutput::GracefulError(_error) => totals.loops_graceful_error += 1,
                StressRunOutput::Timeout => totals.loops_timeout += 1,
                StressRunOutput::DataCorruption => totals.loops_data_corruption += 1,
                StressRunOutput::Panic(_panic_msg) => {}
            }
            match msg {
                StressRunOutput::Success | StressRunOutput::GracefulError(_) => {}
                _ => println!(
                    "{}",
                    serde_json::to_string_pretty(&totals).with_context(
                        ErrorKind::DataConversion,
                        "Failed to serialize test results to JSON.",
                    )?
                ),
            }
        }
    }

    Ok(())
}

fn operation_wrapper(
    mut operation: Box<dyn StressTestOperation>,
    timeout: Option<Duration>,
    tx: UnboundedSender<StressRunOutput>,
) -> Pin<Box<impl Future<Output = ()>>> {
    Box::pin(async move { operation.run(timeout, tx.clone()).await })
}

#[cfg(test)]
mod framework_tests;
