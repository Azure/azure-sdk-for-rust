// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("README.md")]

use clap::{parser::MatchesError, ArgMatches};
use std::any::Any;

/// Metadata about a performance test.
#[derive(Debug, Clone)]
pub struct TestMetadata {
    /// The name of the test.
    pub name: &'static str,
    /// A brief description of the test.
    pub description: &'static str,
    /// The set of test options supported by this test.
    pub options: &'static [&'static TestOption],
}

/// #A `TestOptions` defines a set of options for the test which will be merged with the common test inputs to define the command line for the performance test.
#[derive(Debug, Default)]
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
    parallel: u32,
    test: Option<String>,
    duration: u32,
    warmup: u32,
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
                .get_one::<u32>("parallel")
                .expect("defaulted by clap"),
            test: matches.get_one::<String>("test").cloned(),
            duration: *matches
                .get_one::<u32>("duration")
                .expect("defaulted by clap"),
            warmup: *matches.get_one::<u32>("warmup").expect("defaulted by clap"),
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
    arguments: ArgMatches,
}

impl PerfRunner {
    pub fn new(tests: Vec<TestMetadata>) -> azure_core::Result<Self> {
        let command = Self::get_command_from_metadata(tests);
        let arguments = command.get_matches();
        Ok(Self {
            options: PerfRunnerOptions::from(&arguments),
            arguments,
        })
    }

    #[cfg(test)]
    pub fn with_command_line(
        tests: Vec<TestMetadata>,
        args: Vec<&str>,
    ) -> azure_core::Result<Self> {
        let command = Self::get_command_from_metadata(tests);
        let arguments = command.try_get_matches_from(args).map_err(|e| {
            azure_core::error::Error::with_error(
                azure_core::error::ErrorKind::Other,
                e,
                "Failed to parse command line arguments.",
            )
        })?;
        Ok(Self {
            options: PerfRunnerOptions::from(&arguments),
            arguments,
        })
    }

    /// Gets a reference to a typed argument by its id.
    pub fn try_get_one<T>(&self, id: &str) -> Result<Option<&T>, MatchesError>
    where
        T: Any + Clone + Send + Sync + 'static,
    {
        self.arguments.try_get_one::<T>(id)
    }

    pub fn try_get_one_subcommand<T>(
        &self,
        subcommand: &str,
        id: &str,
    ) -> Result<Option<&T>, MatchesError>
    where
        T: Any + Clone + Send + Sync + 'static,
    {
        let subcommand = self.arguments.subcommand_matches(subcommand);
        if let Some(subcommand) = subcommand {
            subcommand.try_get_one::<T>(id)
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    async fn run_test<F, Fut>(&self, test: F) -> azure_core::Result<()>
    where
        F: Fn(u32, u32) -> Fut,
        Fut: std::future::Future<Output = azure_core::Result<()>>,
    {
        test(self.options.iterations, self.options.parallel).await
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
    fn get_command_from_metadata(tests: Vec<TestMetadata>) -> clap::Command {
        let mut command = clap::Command::new("perf-tests")
            .about("Run performance tests for the Azure SDK for Rust")
            .arg(
                clap::arg!(--iterations <COUNT> "The number of iterations to run each test")
                    .required(false)
                    .default_value("1")
                    .value_parser(clap::value_parser!(u32))
                    .global(true),
            )
            .arg(
                clap::arg!(--parallel <COUNT> "The number of concurrent tasks to use when running each test")
                    .required(false)
                    .default_value("1")
                    .value_parser(clap::value_parser!(u32))
                    .global(true),
            )
            .arg(
                clap::arg!(--test <TEST_NAME> "The name of the test to run. If not specified, all tests will be run.")
                    .required(false)
                    .global(true),
            )
            .arg(
                clap::arg!(--duration <SECONDS> "The duration of each test in seconds")
                    .required(false)
                    .default_value("30")
                    .value_parser(clap::value_parser!(u32))
                    .global(true),
            )
            .arg(
                clap::arg!(--warmup <SECONDS> "The duration of the warmup period in seconds")
                    .required(false)
                    .default_value("5")
                    .value_parser(clap::value_parser!(u32))
                    .global(true),
            ).arg(
                clap::arg!(--"test-results" <FILE> "The file to write test results to")
                    .required(false)
                    .default_value("./tests/results.json")
                    .global(true),
            )
            .arg(clap::arg!(--"no-cleanup" "Disable test cleanup")
            .required(false).global(true))
        ;
        for test in &tests {
            let mut subcommand = clap::Command::new(test.name).about(test.description);
            for option in test.options {
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
mod tests;
