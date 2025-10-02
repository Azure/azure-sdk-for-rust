// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("README.md")]

use crate::TestContext;
use azure_core::{time::Duration, Error, Result};
use clap::ArgMatches;
use std::{
    any::Any,
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::{select, task::JoinSet};

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait PerfTest: Send + Sync {
    async fn setup(&self, context: &TestContext) -> azure_core::Result<()>;
    async fn run(&self /*, context: &TestContext*/) -> azure_core::Result<()>;
    async fn cleanup(&self, context: &TestContext) -> azure_core::Result<()>;
}

pub type CreatePerfTestReturn =
    Pin<Box<dyn Future<Output = azure_core::Result<Box<dyn PerfTest>>>>>;

/// Metadata about a performance test.
#[derive(Debug, Clone)]
pub struct TestMetadata {
    /// The name of the test suite.
    pub name: &'static str,
    /// A brief description of the test suite.
    pub description: &'static str,
    /// The set of test options supported by this test.
    pub options: Vec<TestOption>,

    /// A function used to create the performance test.
    pub create_test: fn(&PerfRunner) -> CreatePerfTestReturn,
}

/// #A `TestOptions` defines a set of options for the test which will be merged with the common test inputs to define the command line for the performance test.
#[derive(Debug, Default, Clone)]
pub struct TestOption {
    /// The name of the test option. This is used as the key in the `TestArguments` map.
    pub name: &'static str,

    /// The short form activator for this argument e.g., `-t`. Does not include the hyphen.
    pub short_activator: char,

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
}

#[derive(Debug)]
#[allow(dead_code)]
struct PerfRunnerOptions {
    no_cleanup: bool,
    iterations: u32,
    parallel: usize,
    duration: Duration,
    warmup: Duration,
    disable_progress: bool,
    test_results_filename: String,
}

impl PerfRunnerOptions {}

impl From<&ArgMatches> for PerfRunnerOptions {
    fn from(matches: &ArgMatches) -> Self {
        Self {
            no_cleanup: matches.get_flag("no-cleanup"),
            iterations: *matches
                .get_one::<u32>("iterations")
                .expect("defaulted by clap"),
            parallel: *matches
                .get_one::<usize>("parallel")
                .expect("defaulted by clap"),
            disable_progress: matches.get_flag("no-progress"),
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
        }
    }
}

/// Context information required by performance tests.
#[derive(Debug)]
pub struct PerfRunner {
    options: PerfRunnerOptions,
    #[allow(dead_code)]
    tests: Vec<TestMetadata>,
    arguments: ArgMatches,
    package_dir: &'static str,
    module_name: &'static str,
    progress: Arc<AtomicU64>,
}

impl PerfRunner {
    pub fn new(
        package_dir: &'static str,
        module_name: &'static str,
        tests: Vec<TestMetadata>,
    ) -> azure_core::Result<Self> {
        let command = Self::get_command_from_metadata(&tests);
        let arguments = command.get_matches();
        Ok(Self {
            options: PerfRunnerOptions::from(&arguments),
            tests,
            arguments,
            package_dir,
            module_name,
            progress: Arc::new(AtomicU64::new(0)),
        })
    }

    #[cfg(test)]
    pub fn with_command_line(
        package_dir: &'static str,
        module_name: &'static str,
        tests: Vec<TestMetadata>,
        args: Vec<&str>,
    ) -> azure_core::Result<Self> {
        let command = Self::get_command_from_metadata(&tests);
        let arguments = command.try_get_matches_from(args).map_err(|e| {
            azure_core::error::Error::with_error(
                azure_core::error::ErrorKind::Other,
                e,
                "Failed to parse command line arguments.",
            )
        })?;
        Ok(Self {
            options: PerfRunnerOptions::from(&arguments),
            tests,
            arguments,
            package_dir,
            module_name,
            progress: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Gets a reference to a typed argument by its id.
    pub fn try_get_global_arg<T>(&self, id: &str) -> Result<Option<&T>>
    where
        T: Any + Clone + Send + Sync + 'static,
    {
        self.arguments.try_get_one::<T>(id).map_err(|e| {
            Error::with_error(
                azure_core::error::ErrorKind::Other,
                e,
                format!("Failed to get argument '{}'.", id),
            )
        })
    }

    pub fn try_get_test_arg<T>(&self, id: &str) -> Result<Option<&T>>
    where
        T: Any + Clone + Send + Sync + 'static,
    {
        if let Some((_, args)) = self.arguments.subcommand() {
            args.try_get_one::<T>(id).map_err(|e| {
                Error::with_error(
                    azure_core::error::ErrorKind::Other,
                    e,
                    format!("Failed to get argument '{}' for test.", id),
                )
            })
        } else {
            Ok(None)
        }
    }

    pub fn get_selected_test_name(&self) -> Result<&str> {
        match self.arguments.subcommand_name() {
            Some(name) => Ok(name),
            None => Err(Error::with_message(
                azure_core::error::ErrorKind::Other,
                "No test was selected.",
            )),
        }
    }

    pub async fn run(&self) -> azure_core::Result<()> {
        // We can only run tests if there was a test selected.
        let test_name = self.get_selected_test_name()?;

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
        let test_instance = (test.create_test)(self).await?;
        let test_instance: Arc<dyn PerfTest> = Arc::from(test_instance);

        let context = TestContext::new(self.package_dir, self.module_name, test.name)?;

        for iteration in 0..self.options.iterations {
            println!(
                "Running test iteration {}/{}",
                iteration + 1,
                self.options.iterations
            );

            println!("========== Starting test setup ==========");
            test_instance.setup(&context).await?;

            println!(
                "========== Starting test warmup for {} ==========",
                self.options.warmup
            );

            self.run_test_for(Arc::clone(&test_instance), test.name, self.options.warmup)
                .await?;

            println!(
                "========== Starting test run for {} ==========",
                self.options.duration
            );
            self.run_test_for(Arc::clone(&test_instance), test.name, self.options.duration)
                .await?;
            if !self.options.no_cleanup {
                println!("========== Starting test cleanup ==========");
                test_instance.cleanup(&context).await?;
            }

            let iteration_count = self.progress.load(Ordering::SeqCst);
            println!(
                "Completed test iteration {}/{} - {} iterations run in {} seconds - {} seconds/iteration",
                iteration + 1,
                self.options.iterations,
                iteration_count,
                self.options.duration.as_seconds_f64(),
                self.options.duration.as_seconds_f64() / iteration_count as f64
            );
            let operations_per_second =
                self.options.duration.as_seconds_f64() / iteration_count as f64;
            let duration_per_operation = Duration::seconds_f64(operations_per_second);
            println!("{} seconds/operation", duration_per_operation);
        }
        Ok(())
    }
    pub async fn run_test_for(
        &self,
        test_instance: Arc<dyn PerfTest>,
        _test_name: &str,
        duration: Duration,
    ) -> azure_core::Result<()> {
        let mut tasks: JoinSet<Result<()>> = JoinSet::new();
        for _ in 0..self.options.parallel {
            let test_instance_clone = Arc::clone(&test_instance);
            let progress = self.progress.clone();
            // let package_dir = self.package_dir;
            // let module_name = self.module_name;
            tasks.spawn(async move {
                //                let context =
                //                    TestContext::new(package_dir, module_name, " test_name_copy.as_str()")?;

                loop {
                    test_instance_clone.run(/*&context*/).await?;
                    progress.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
        let start = tokio::time::Instant::now();
        let timeout = tokio::time::Duration::from_secs_f64(duration.as_seconds_f64());
        select!(
                _ = tokio::time::sleep(timeout) => {println!("Timeout reached, stopping test tasks: {:?}", start.elapsed());},
                _ = tasks.join_all() =>  {println!("All test tasks completed: {:?}", start.elapsed());},
                _ = async {
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            println!("{:?} elapsed: {} op/sec,  {} sec/ operation.",
                                start.elapsed(),
                                self.progress.load(Ordering::SeqCst) as f64 / start.elapsed().as_secs_f64(),
                                Duration::seconds_f64( start.elapsed().as_secs_f64() / self.progress.load(Ordering::SeqCst) as f64 ));
                        }
                    }, if !self.options.disable_progress => {},
        );
        println!("Task time elapsed: {:?}", start.elapsed());
        Ok(())
    }

    // * Disable test cleanup
    // * Test Proxy servers.
    // * TLS
    //   * Allow untrusted TLS certificates
    // * Advanced options
    //   * Print job statistics (?)
    //   * Track latency and print per-operation latency statistics
    //   * Target throughput (operations/second) (?)
    // * Language specific options
    //   * Max I/O completion threads
    //   * Minimum number of asynchronous I/O threads in the thread pool
    //   * Minimum number of worker threads the thread pool creates on demand
    //   * Sync - run a synchronous version of the test

    /// Constructs a `clap::Command` from the provided test metadata.
    fn get_command_from_metadata(tests: &[TestMetadata]) -> clap::Command {
        let mut command = clap::Command::new("perf-tests")
            .about("Run performance tests for the Azure SDK for Rust")
            .arg(
                clap::arg!(--iterations <COUNT> "The number of iterations to run each test")
                    .required(false)
                    .default_value("1")
                    .value_parser(clap::value_parser!(u32))
                    .global(false),
            )
            .arg(
                clap::arg!(--parallel <COUNT> "The number of concurrent tasks to use when running each test")
                    .required(false)
                    .default_value("1")
                    .value_parser(clap::value_parser!(usize))
                    .global(false),
            )
            .arg(clap::arg!(--"no-progress" "Disable progress reporting").required(false).global(false))
            .arg(
                clap::arg!(--duration <SECONDS> "The duration of each test in seconds")
                    .required(false)
                    .default_value("30")
                    .value_parser(clap::value_parser!(i64))
                    .global(false),
            )
            .arg(
                clap::arg!(--warmup <SECONDS> "The duration of the warmup period in seconds")
                    .required(false)
                    .default_value("5")
                    .value_parser(clap::value_parser!(i64))
                    .global(false),
            )
            .arg(
                clap::arg!(--"test-results" <FILE> "The file to write test results to")
                    .required(false)
                    .default_value("./tests/results.json")
                    .global(false),
            )
            .arg(clap::arg!(--"no-cleanup" "Disable test cleanup")
            .required(false).global(true))
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
                if option.short_activator != '\0' {
                    arg = arg.short(option.short_activator);
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
