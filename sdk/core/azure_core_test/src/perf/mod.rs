// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::TestContext;
use azure_core::{
    error::{ErrorKind, ResultExt},
    http::Url,
    time::Duration,
    Result,
};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::{
    fmt::{Debug, Display},
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::{select, task::JoinSet};

/// A trait representing a performance test.
///
/// Performance tests have three phases:
/// 1. `setup`: Prepare the test environment.
/// 2. `run`: Execute the performance test. This is called repeatedly for the duration of the test.
/// 3. `cleanup`: Clean up the test environment.
///
/// One instance of the test is created per parallel task. Each instance owns its own resources
/// (e.g., clients, containers, blobs) and is not shared across tasks. This allows each task to
/// set up dedicated resources in `setup` without worrying about concurrent access.
#[async_trait::async_trait]
pub trait PerfTest: Send + Sync {
    /// Set up the test environment.
    ///
    /// Performs whatever steps are needed to set up the test environment.
    /// Called once per instance (i.e., once per parallel task) per iteration.
    ///
    /// # Arguments
    /// - `context`: An `Arc` to a `TestContext` that provides context information for the test.
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()>;
    async fn run(&self, context: Arc<TestContext>) -> azure_core::Result<()>;
    async fn cleanup(&self, context: Arc<TestContext>) -> azure_core::Result<()>;
}

pub trait PerfTestFactory: Subcommand + Clone + Debug {
    fn name(&self) -> &'static str;
    fn create_test(&self) -> CreatePerfTestReturn;
    fn peek(&self) -> PeekSubcommandInfo {
        Default::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct PeekSubcommandInfo {
    // Resource size
    pub size: Option<i64>,
}

pub type CreatePerfTestReturn =
    Pin<Box<dyn Future<Output = azure_core::Result<Box<dyn PerfTest>>>>>;

#[derive(Debug, Clone, Default, Serialize)]
#[allow(dead_code)]
struct PerfTestOutputs {
    pub test_name: String,
    pub operations_per_second: f64,
    pub average_cpu_use: Option<f64>,
    pub average_memory_use: Option<f64>,
}

/// Per-operation latency result matching the PerfAutomation JSON format.
#[derive(Serialize)]
struct OperationResult {
    #[serde(rename = "Time")]
    time: f64,
    #[serde(rename = "Size")]
    size: i64,
}

#[derive(Parser, Debug, Clone)]
struct PerfRunnerOptions<T: PerfTestFactory> {
    // Satisfies the cargo bench command.
    #[arg(long, global = true, default_value_t = true, hide = true)]
    bench: bool,

    // Disable test cleanup.
    #[arg(long, global = true)]
    no_cleanup: bool,

    // The number of iterations to run each test.
    #[arg(long, default_value_t = 1, value_name = "COUNT")]
    iterations: u32,

    // The number of concurrent tasks to use when running each test.
    #[arg(short, long, default_value_t = 1, global = true, value_name = "COUNT")]
    parallel: u32,

    // The duration of each test in seconds.
    #[arg(short, long, default_value = "30", global = true, value_parser = duration_seconds, value_name = "SECONDS")]
    duration: Duration,

    // The duration of the warmup period in seconds.
    #[arg(long, default_value = "5", global = true, value_parser = duration_seconds, value_name = "SECONDS")]
    warmup: Duration,

    // Disable progress reporting.
    #[arg(long = "no-progress")]
    disable_progress: bool,

    // Track and print per-operation latency statistics.
    #[arg(short, long, global = true)]
    latency: bool,

    // The file to write test results to.
    #[arg(
        long = "test-results",
        default_value = "./results.json",
        value_name = "FILE"
    )]
    test_results_filename: String,

    // File path to store per-operation latency results (requires --latency)
    #[arg(long, default_value = "", global = true, value_name = "FILE")]
    results_file: String,

    // Run synchronous tests (ignored).
    #[arg(long, global = true)]
    sync: bool,

    // URL of test-proxy (ignored).
    #[arg(long, global = true, value_parser = url, value_name = "URL")]
    test_proxy: Option<Url>,

    #[command(subcommand)]
    pub subcommand: T,
}

fn duration_seconds(s: &str) -> std::result::Result<Duration, String> {
    s.parse::<i64>()
        .map(Duration::seconds)
        .map_err(|e| format!("Failed to parse duration '{}': {}", s, e))
}

fn url(s: &str) -> std::result::Result<Url, String> {
    Url::parse(s).map_err(|e| format!("Failed to parse URL '{}': {}", s, e))
}

impl<T: PerfTestFactory> Display for PerfRunnerOptions<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PerfRunnerOptions {{ no_cleanup: {}, iterations: {}, parallel: {}, duration: {}, warmup: {}, disable_progress: {}, latency: {}, test_results_filename: '{}', results_file: '{}' }}",
            self.no_cleanup,
            self.iterations,
            self.parallel,
            self.duration,
            self.warmup,
            self.disable_progress,
            self.latency,
            self.test_results_filename,
            self.results_file
        )
    }
}

/// Context information required by performance tests.
#[derive(Debug, Clone)]
pub struct PerfRunner<T: PerfTestFactory> {
    options: PerfRunnerOptions<T>,
    package_dir: &'static str,
    module_name: &'static str,
    progress: Arc<AtomicU64>,
}

impl<T: PerfTestFactory> PerfRunner<T> {
    /// Run the performance tests in `tests` using the current process command line.
    ///
    /// # Arguments
    ///
    /// * package_dir - The directory containing the package with the tests. Typically `env!("CARGO_PACKAGE_DIR")`
    /// * module_name - the name of the module containing the test, typically `file!()`
    /// * tests - the set of tests to configure.
    ///
    pub fn new(
        package_dir: &'static str,
        module_name: &'static str,
    ) -> std::result::Result<Self, clap::Error> {
        Ok(Self {
            options: PerfRunnerOptions::<T>::try_parse()?,
            package_dir,
            module_name,
            progress: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Run the performance tests in `tests` with the command line specified in `args`
    pub fn with_command_line(
        package_dir: &'static str,
        module_name: &'static str,
        args: Vec<&str>,
    ) -> std::result::Result<Self, clap::Error> {
        Ok(Self {
            options: PerfRunnerOptions::<T>::try_parse_from(args.iter())?,
            package_dir,
            module_name,
            progress: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Runs the selected performance test.
    ///
    /// This will run the selected test for the configured number of iterations, parallel tasks, and duration.
    ///
    /// If no test has been selected, this will print an error message and return Ok(()).
    ///
    /// # Returns
    ///
    /// A result indicating the success or failure of the test run.
    ///
    pub async fn run(&self) -> azure_core::Result<()> {
        let test = &self.options.subcommand;
        let test_mode = crate::TestMode::current_opt()?.unwrap_or(crate::TestMode::Live);

        println!("Test Configuration: {:#}", self.options);

        for iteration in 0..self.options.iterations {
            println!(
                "Running test iteration {}/{}",
                iteration + 1,
                self.options.iterations
            );

            // Create one test instance and context per parallel task.
            let mut test_instances: Vec<Arc<dyn PerfTest>> = Vec::new();
            let mut test_contexts: Vec<Arc<TestContext>> = Vec::new();

            println!("========== Starting test setup ==========");
            for i in 0..self.options.parallel {
                let instance = test.create_test().await?;
                let instance: Arc<dyn PerfTest> = Arc::from(instance);
                let context = Arc::new(
                    crate::recorded::start(
                        test_mode,
                        self.package_dir,
                        self.module_name,
                        test.name(),
                        None,
                    )
                    .await?,
                );
                instance.setup(context.clone()).await?;
                println!(
                    "Setup complete for parallel task {}/{}",
                    i + 1,
                    self.options.parallel
                );
                test_instances.push(instance);
                test_contexts.push(context);
            }

            println!(
                "========== Starting test warmup for {} ==========",
                self.options.warmup
            );

            self.run_test_for(&test_instances, &test_contexts, self.options.warmup, false)
                .await?;

            println!(
                "========== Starting test run for {} ==========",
                self.options.duration
            );

            let (operations_per_second, mut latencies) = self
                .run_test_for(
                    &test_instances,
                    &test_contexts,
                    self.options.duration,
                    self.options.latency,
                )
                .await?;
            if self.options.latency {
                latencies.sort();
                Self::print_latencies("Latency Distribution", &latencies);

                // Still useful to print the latencies above even if we're not writing them to a file.
                if !self.options.results_file.is_empty() {
                    // Detect size from the selected test's subcommand args, defaulting to -1.
                    let size: i64 = self.options.subcommand.peek().size.unwrap_or(-1);

                    let results: Vec<OperationResult> = latencies
                        .iter()
                        .map(|l| OperationResult {
                            time: l.as_secs_f64() * 1000.0,
                            size,
                        })
                        .collect();

                    let json = serde_json::to_string_pretty(&results).with_context(
                        ErrorKind::DataConversion,
                        "Failed to serialize latency results to JSON.",
                    )?;

                    std::fs::write(&self.options.results_file, json)
                        .with_context(ErrorKind::Io, "Failed to write latency results to file.")?;
                }
            }
            if !self.options.no_cleanup {
                println!("========== Starting test cleanup ==========");
                for (instance, context) in test_instances.iter().zip(test_contexts.iter()) {
                    instance.cleanup(context.clone()).await?;
                }
            }

            println!(
                "Completed test iteration {}/{} - {} operations/second",
                iteration + 1,
                self.options.iterations,
                operations_per_second,
            );

            if !self.options.test_results_filename.is_empty() {
                // Write out the results to a file.
                println!(
                    "Writing test results to {}",
                    self.options.test_results_filename
                );
                let results = PerfTestOutputs {
                    test_name: test.name().to_string(),
                    operations_per_second,
                    average_cpu_use: None,
                    average_memory_use: None,
                };

                let json = serde_json::to_string_pretty(&results).with_context(
                    ErrorKind::DataConversion,
                    "Failed to serialize test results to JSON.",
                )?;

                println!("Test results: {}", json);
                std::fs::write(&self.options.test_results_filename, json)
                    .with_context(ErrorKind::Io, "Failed to write test results to file.")?;
            }
        }
        Ok(())
    }

    /// Runs the provided test instances for the specified duration using the provided test contexts.
    ///
    /// Each parallel task runs its own dedicated test instance, allowing per-task resource isolation.
    ///
    /// # Arguments
    /// * `test_instances` - One test instance per parallel task.
    /// * `test_contexts` - The test contexts to use for each parallel task.
    /// * `duration` - The duration to run the test for.
    /// * `track_latency` - Whether to track per-operation latency.
    ///
    /// # Returns
    /// A tuple of (operations per second, per-operation latencies). Latencies is empty if `track_latency` is false.
    pub async fn run_test_for(
        &self,
        test_instances: &[Arc<dyn PerfTest>],
        test_contexts: &[Arc<TestContext>],
        duration: Duration,
        track_latency: bool,
    ) -> azure_core::Result<(f64, Vec<tokio::time::Duration>)> {
        // Reset the performance measurements before starting the test.
        self.progress.store(0, Ordering::SeqCst);
        let mut tasks: JoinSet<Result<(i64, tokio::time::Duration, Vec<tokio::time::Duration>)>> =
            JoinSet::new();
        (0..self.options.parallel).for_each(|i| {
            let test_instance = Arc::clone(&test_instances[i as usize]);
            let progress = self.progress.clone();
            let test_context = test_contexts[i as usize].clone();
            tasks.spawn(async move {
                let start = tokio::time::Instant::now();
                let mut count = 0i64;
                let mut latencies = Vec::new();
                let timeout = tokio::time::Duration::from_secs_f64(duration.as_seconds_f64());
                loop {
                    let op_start = if track_latency {
                        Some(tokio::time::Instant::now())
                    } else {
                        None
                    };
                    test_instance.run(test_context.clone()).await?;
                    if let Some(op_start) = op_start {
                        latencies.push(op_start.elapsed());
                    }
                    progress.fetch_add(1, Ordering::SeqCst);
                    count += 1;
                    if start.elapsed() >= timeout {
                        break;
                    }
                }
                Ok((count, start.elapsed(), latencies))
            });
        });
        let start = tokio::time::Instant::now();

        let (operations_per_second, all_latencies) = select!(
                results = tasks.join_all() =>  {
                    println!("All test tasks completed: {:?}", start.elapsed());
                    let collected_results: Result<Vec<_>> = results.into_iter().collect();
                    let collected = collected_results?;

                    let total_ops:f64 = collected
                        .iter()
                        .map(|(count, duration, _)| {*count as f64 / duration.as_secs_f64()})
                        .sum();

                    let all_latencies: Vec<tokio::time::Duration> = collected
                        .into_iter()
                        .flat_map(|(_, _, latencies)| latencies)
                        .collect();

                    println!("Total operations per second: {total_ops}");
                    Ok((total_ops, all_latencies))
                },
                _ = async {
                        let mut last_count = 0;
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            let current_total = self.progress.load(Ordering::SeqCst);

                            if start.elapsed().as_secs_f64() != 0f64 && current_total != 0 {
                                println!("Current {:3}, Total {:5} {:4}", current_total - last_count, current_total, Duration::seconds_f64( start.elapsed().as_secs_f64() / current_total as f64 ));
                            }
                            else{
                                println!("Current {:3}, Total {:5} ---", current_total - last_count, current_total);
                            }

                            last_count = current_total;
                        }
                    }, if !self.options.disable_progress => {Err(azure_core::Error::with_message(
                        ErrorKind::Other,
                        "Progress reporting task exited unexpectedly.",
                    ))},
        )?;
        Ok((operations_per_second, all_latencies))
    }

    /// Print latency percentiles to the console. Requires the latencies to be pre-sorted.
    fn print_latencies(header: &str, latencies: &[tokio::time::Duration]) {
        if latencies.is_empty() {
            return;
        }
        println!("=== {} ===", header);
        let percentiles = [0.5, 0.75, 0.9, 0.99, 0.999, 0.9999, 0.99999, 1.0];
        for percentile in percentiles {
            let index = ((latencies.len() as f64 * percentile) as usize).saturating_sub(1);
            let latency = latencies[index];
            println!(
                "{:>9.3}%   {:>8.2}ms",
                percentile * 100.0,
                latency.as_secs_f64() * 1000.0
            );
        }
        println!();
    }
}

#[cfg(test)]
mod framework_tests;
