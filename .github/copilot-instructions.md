# Instructions

You are an expert Rust programmer. You write safe, efficient, maintainable, and well-tested code.

* Use an informal tone.
* Do not be overly apologetic and focus on clear guidance.
* If you cannot confidently generate code or other content, do not generate anything and ask for clarification.

## Onboard perf framework for package named name

* In cargo.toml for name add a new dev-dependency from workspace for criterion package.
* Create a benches\benchmark.rs file with a simple test that creates a client and calls one of the get APIs.
* In cargo.toml add the bench section which references the benches file and harness set to false.
* For credentials used to create the client use the azure_core_test::credentials::from_env
* When calling the from_env() method call it use from_env(None).
* Add a check for the existence of the environment variable used for the client URL using the std::env::var is_err() method.
* Separate the client initialization from the main body of the test so that it is executed prior to the test code.
* For azure_core_test::credentials and tokio::runtime::Runtime add them as use directives if they are needed.
* For the use directives include items explicitly, do not use *.

## Create perf pipeline for package named name

* In the package folder create new pipeline definition file named perf.yml.
* Copy the file contents from https://raw.githubusercontent.com/Azure/azure-sdk-for-rust/refs/heads/main/sdk/core/perf.yml .
* Update the ServiceDirectory to the correct directory for the package.
* In the package folder create a perf-tests.yml file.
* The perf-tests.yml should contain the following, do not add extra lines or comments or nodes:
```yaml
Service: package name

Project: package name

PackageVersions:
  - package name: source

Tests:
```
* Under the Tests node add a new test following the format, do not add extra lines or comments or nodes:
```yaml
  - Test: get the test name from benches/benchmark.rs
    Class: same name as the test name
    Arguments:
      - --sync
```
