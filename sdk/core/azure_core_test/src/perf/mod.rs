// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::TestContext;
use azure_core::{
    error::{ErrorKind, ResultExt},
    time::Duration,
    Error, Result,
};
use clap::ArgMatches;
use serde::Serialize;
use std::{
    fmt::Display,
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

pub type CreatePerfTestReturn =
    Pin<Box<dyn Future<Output = azure_core::Result<Box<dyn PerfTest>>>>>;

/// Type alias for an async function that creates a PerfTest instance.
/// Takes a PerfRunner reference and returns a future that resolves to a PerfTest trait object.
pub type CreatePerfTestFn = fn(PerfRunner) -> CreatePerfTestReturn;

/// Metadata about a performance test.
#[derive(Debug, Clone)]
pub struct PerfTestMetadata {
    /// The name of the test suite.
    pub name: &'static str,
    /// A brief description of the test suite.
    pub description: &'static str,
    /// The set of test options supported by this test.
    pub options: Vec<PerfTestOption>,

    /// An async function used to create the performance test.
    /// Takes a PerfRunner reference and returns a future that resolves to a PerfTest trait object.
    pub create_test: CreatePerfTestFn,
}

/// The expected type of a test option, used in [`PerfRunner::try_get_test_arg`] and [`PerfRunner::try_get_global_arg`]
///
/// This allows a test author to declare the expected numeric type of the option value, which
/// simplifies the work involved in processing a test option value and reduces the chance of errors in that processing.
///
/// The default option type is `String`,
#[derive(Debug, Clone, Default)]
pub enum PerfTestOptionKind {
    // Note: We need this type because `clap` requires us to specify the expected type of each argument
    // for proper parsing, and this allows us to leverage that parsing in `try_get_test_arg` and
    // `try_get_global_arg` to get typed arguments without needing to do any additional parsing
    // or error handling in the test code. See also get_command_for_metadata which specifies the
    // clap parser for each option based on this type.
    #[default]
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Usize,
    Boolean,
}

/// A `PerfTestOptions` defines a set of options for the test which will be merged with the common test inputs to define the command line for the performance test.
#[derive(Debug, Default, Clone)]
pub struct PerfTestOption {
    /// The name of the test option. This is used as the key in the `TestArguments` map.
    pub name: &'static str,

    /// The short form activator for this argument e.g., `-t`. Does not include the hyphen.
    pub short_activator: Option<char>,

    /// The long form activator for this argument e.g., `--test-option`. Does not include the hyphens.
    pub long_activator: &'static str,

    /// Display message - displayed in the --help message.
    pub display_message: &'static str,

    /// Expected argument count
    pub expected_args_len: usize,

    /// Required
    pub mandatory: bool,

    /// Argument value is sensitive and should be sanitized.
    pub sensitive: bool,

    /// The expected type of the argument value.
    pub option_type: PerfTestOptionKind,
}

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

#[derive(Debug, Clone)]
struct PerfRunnerOptions {
    no_cleanup: bool,
    iterations: u32,
    parallel: u32,
    duration: Duration,
    warmup: Duration,
    disable_progress: bool,
    latency: bool,
    test_results_filename: String,
    results_file: String,
}

impl Display for PerfRunnerOptions {
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

impl From<&ArgMatches> for PerfRunnerOptions {
    fn from(matches: &ArgMatches) -> Self {
        Self {
            no_cleanup: matches.get_flag("no-cleanup"),
            iterations: *matches
                .get_one::<u32>("iterations")
                .expect("defaulted by clap"),
            parallel: *matches
                .get_one::<u32>("parallel")
                .expect("defaulted by clap"),
            disable_progress: matches.get_flag("no-progress"),
            latency: matches.get_flag("latency"),
            duration: Duration::seconds(
                *matches
                    .get_one::<i64>("duration")
                    .expect("defaulted by clap"),
            ),
            warmup: Duration::seconds(
                *matches.get_one::<i64>("warmup").expect("defaulted by clap"),
            ),
            test_results_filename: matches
                .get_one::<String>("test-results")
                .expect("defaulted by clap")
                .to_string(),
            results_file: matches
                .get_one::<String>("results-file")
                .cloned()
                .unwrap_or_default(),
        }
    }
}

/// Context information required by performance tests.
#[derive(Debug, Clone)]
pub struct PerfRunner {
    options: PerfRunnerOptions,
    tests: Vec<PerfTestMetadata>,
    arguments: ArgMatches,
    package_dir: &'static str,
    module_name: &'static str,
    progress: Arc<AtomicU64>,
}

impl PerfRunner {
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
        tests: Vec<PerfTestMetadata>,
    ) -> Result<Self> {
        let command = Self::get_command_from_metadata(&tests);
        let arguments = command.try_get_matches();
        let arguments = match arguments {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        Ok(Self {
            options: PerfRunnerOptions::from(&arguments),
            tests,
            arguments,
            package_dir,
            module_name,
            progress: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Run the performance tests in `tests` with the command line specified in `args`
    pub fn with_command_line(
        package_dir: &'static str,
        module_name: &'static str,
        tests: Vec<PerfTestMetadata>,
        args: Vec<&str>,
    ) -> azure_core::Result<Self> {
        let command = Self::get_command_from_metadata(&tests);
        let arguments = command
            .try_get_matches_from(args)
            .with_context(ErrorKind::Other, "Failed to parse command line arguments.")?;

        Ok(Self {
            options: PerfRunnerOptions::from(&arguments),
            tests,
            arguments,
            package_dir,
            module_name,
            progress: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Gets a typed argument by its id.
    pub fn try_get_global_arg<T>(&self, id: &str) -> Result<Option<T>>
    where
        T: Clone + Send + Sync + 'static,
    {
        self.arguments
            .try_get_one::<T>(id)
            .with_context(
                ErrorKind::Other,
                format!("Failed to get argument '{}'.", id),
            )
            .map(|arg| arg.cloned())
    }

    /// Gets a typed argument for the selected test by its id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the argument to get.
    ///
    /// # Returns
    ///
    /// The argument if it exists, or None.
    pub fn try_get_test_arg<T>(&self, id: &str) -> Result<Option<T>>
    where
        T: Clone + Send + Sync + 'static,
    {
        if let Some((_, args)) = self.arguments.subcommand() {
            args.try_get_one::<T>(id)
                .with_context(
                    ErrorKind::Other,
                    format!("Failed to get argument '{}' for test.", id),
                )
                .map(|arg| arg.cloned())
        } else {
            Ok(None)
        }
    }

    /// Gets the name of the selected test.
    pub fn get_selected_test_name(&self) -> Result<&str> {
        match self.arguments.subcommand_name() {
            Some(name) => Ok(name),
            None => Err(Error::with_message(
                azure_core::error::ErrorKind::Other,
                "No test was selected.",
            )),
        }
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
        // We can only run tests if there was a test selected.
        let test_name = match self.get_selected_test_name() {
            Ok(name) => name,
            Err(e) => {
                eprintln!("Error getting selected test name: {}", e);
                return Ok(());
            }
        };

        let test = self
            .tests
            .iter()
            .find(|t| t.name == test_name)
            .ok_or_else(|| {
                Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("Test '{}' not found.", test_name),
                )
            })?;

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
                let instance = (test.create_test)(self.clone()).await?;
                let instance: Arc<dyn PerfTest> = Arc::from(instance);
                let context = Arc::new(
                    crate::recorded::start(
                        test_mode,
                        self.package_dir,
                        self.module_name,
                        test.name,
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
                Self::print_latencies("Latency Distribution", &mut latencies);

                // Still useful to print the latencies above even if we're not writing them to a file.
                if !self.options.results_file.is_empty() {
                    // Detect size from the selected test's subcommand args, defaulting to -1.
                    let size: i64 = self
                        .try_get_test_arg::<usize>("size")
                        .ok()
                        .flatten()
                        .map(|s| s as i64)
                        .unwrap_or(-1);

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
                    test_name: test.name.to_string(),
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

    fn print_latencies(header: &str, latencies: &mut [tokio::time::Duration]) {
        if latencies.is_empty() {
            return;
        }
        latencies.sort();
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

    // Future command line switches:
    // * Test Proxy servers.
    // * TLS
    //   * Allow untrusted TLS certificates
    // * Advanced options
    //   * Print job statistics (?)
    //   * Target throughput (operations/second) (?)
    // * Language specific options
    //   * Max I/O completion threads
    //   * Minimum number of asynchronous I/O threads in the thread pool
    //   * Minimum number of worker threads the thread pool creates on demand
    //   * Sync - run a synchronous version of the test

    /// Constructs a `clap::Command` from the provided test metadata.
    fn get_command_from_metadata(tests: &[PerfTestMetadata]) -> clap::Command {
        let mut command = clap::Command::new("perf-tests")
            .about("Run performance tests for the Azure SDK for Rust")
            .arg(
                clap::arg!(--iterations <COUNT> "The number of iterations to run each test")
                    .required(false)
                    .default_value("1")
                    .value_parser(clap::value_parser!(u32))
                    .global(false),
            )
            .arg(clap::arg!(--sync "Run synchronous tests (ignored)")
                .global(true)
                .required(false))
            .arg(clap::arg!(--"test-proxy" <URL> "The URL of the test proxy, ignored.")
                .global(true)
                .value_parser(clap::value_parser!(String))
                .required(false))
            .arg(
                clap::arg!(--parallel <COUNT> "The number of concurrent tasks to use when running each test")
                    .required(false)
                    .short('p')
                    .default_value("1")
                    .value_parser(clap::value_parser!(u32))
                    .global(true),
            )
            .arg(clap::arg!(--"no-progress" "Disable progress reporting").required(false).global(false))
            .arg(
                clap::arg!(--duration <SECONDS> "The duration of each test in seconds")
                    .required(false)
                    .short('d')
                    .default_value("30")
                    .value_parser(clap::value_parser!(i64))
                    .global(true),
            )
            .arg(
                clap::arg!(--warmup <SECONDS> "The duration of the warmup period in seconds")
                    .required(false)
                    .default_value("5")
                    .value_parser(clap::value_parser!(i64))
                    .global(true),
            )
            // Cargo bench passes --bench to the test binary to instruct it to run benchmarks only.
            .arg(clap::arg!(--bench).required(false).global(true))
            .arg(
                clap::arg!(--"test-results" <FILE> "The file to write test results to")
                    .required(false)
                    .default_value("./results.json")
                    .global(false),
            )
            .arg(clap::arg!(--"no-cleanup" "Disable test cleanup")
            .required(false).global(true))
            .arg(clap::arg!(-l --latency "Track and print per-operation latency statistics")
            .required(false).global(true))
            .arg(
                clap::arg!(--"results-file" <FILE> "File path to store per-operation latency results (requires --latency)")
                    .required(false)
                    .global(true),
            )
        ;
        for test in tests {
            let mut subcommand = clap::Command::new(test.name).about(test.description);
            for option in test.options.iter() {
                let mut arg = clap::Arg::new(option.name)
                    .help(option.display_message)
                    .long(option.long_activator)
                    .num_args(option.expected_args_len..=option.expected_args_len)
                    .required(option.mandatory)
                    .global(false);
                arg = match option.option_type {
                    PerfTestOptionKind::String => arg.value_parser(clap::value_parser!(String)),
                    PerfTestOptionKind::Usize => arg.value_parser(clap::value_parser!(usize)),
                    PerfTestOptionKind::Int8 => arg.value_parser(clap::value_parser!(i8)),
                    PerfTestOptionKind::Int16 => arg.value_parser(clap::value_parser!(i16)),
                    PerfTestOptionKind::Int32 => arg.value_parser(clap::value_parser!(i32)),
                    PerfTestOptionKind::Int64 => arg.value_parser(clap::value_parser!(i64)),
                    PerfTestOptionKind::Uint8 => arg.value_parser(clap::value_parser!(u8)),
                    PerfTestOptionKind::Uint16 => arg.value_parser(clap::value_parser!(u16)),
                    PerfTestOptionKind::Uint32 => arg.value_parser(clap::value_parser!(u32)),
                    PerfTestOptionKind::Uint64 => arg.value_parser(clap::value_parser!(u64)),
                    PerfTestOptionKind::Boolean => {
                        // For boolean options, we can use the presence of the flag to indicate true, and absence to indicate false.
                        // Therefore, we don't need to specify a value parser or expect any arguments for this type.
                        arg.action(clap::ArgAction::SetTrue).num_args(0)
                    }
                };
                if let Some(short_activator) = option.short_activator {
                    arg = arg.short(short_activator);
                }
                if option.sensitive {
                    arg = arg.hide(true);
                }
                subcommand = subcommand.arg(arg);
            }
            command = command.subcommand(subcommand);
        }

        command
    }
}

#[cfg(test)]
mod config_tests;

#[cfg(test)]
mod framework_tests;
