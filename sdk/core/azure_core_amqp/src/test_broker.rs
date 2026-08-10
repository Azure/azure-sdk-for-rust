// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Helpers for the tests that need the local AMQP test broker.
//!
//! `Test-Setup.ps1` builds the broker, starts it, and sets
//! `TEST_BROKER_ADDRESS`. A test that needs the broker calls
//! [`test_broker_address`].
//!
//! Two environment variables control the behavior:
//!
//! * `TEST_BROKER_ADDRESS` holds the broker address, for example
//!   `amqp://127.0.0.1:25672`.
//! * `TEST_BROKER_REQUIRED` makes a missing broker an error instead of a skip.
//!   The pipeline sets it, so that a broker that stops running makes the build
//!   red. A developer who does not set it can still run the other tests.
//!
//! `TEST_BROKER_REQUIRED` is on when it holds a value other than an empty
//! string, `0`, or `false`.

/// Name of the variable that holds the address of the test broker.
const TEST_BROKER_ADDRESS: &str = "TEST_BROKER_ADDRESS";

/// Name of the variable that turns a skipped broker test into a failure.
const TEST_BROKER_REQUIRED: &str = "TEST_BROKER_REQUIRED";

/// Returns the address of the test broker, or `None` when the caller must skip
/// the test.
///
/// # Panics
///
/// Panics when `TEST_BROKER_REQUIRED` is on and the broker address is absent or
/// empty.
pub(crate) fn test_broker_address() -> Option<String> {
    let address = std::env::var(TEST_BROKER_ADDRESS)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    if address.is_some() {
        return address;
    }

    assert!(
        !is_env_flag_set(TEST_BROKER_REQUIRED),
        "{TEST_BROKER_REQUIRED} is set, but {TEST_BROKER_ADDRESS} is absent or empty. \
         Start the broker with sdk/core/azure_core_amqp/Test-Setup.ps1 and run the tests in \
         the same shell, or clear {TEST_BROKER_REQUIRED} to skip the broker tests."
    );

    println!("{TEST_BROKER_ADDRESS} is not set. Skipping test.");
    None
}

/// Returns `true` when the variable holds a value other than an empty string,
/// `0`, or `false`.
fn is_env_flag_set(name: &str) -> bool {
    match std::env::var(name) {
        Ok(value) => {
            let value = value.trim();
            !value.is_empty() && value != "0" && !value.eq_ignore_ascii_case("false")
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_env_flag_set;

    #[test]
    fn env_flag_is_off_when_absent() {
        assert!(!is_env_flag_set("AZURE_CORE_AMQP_FLAG_THAT_IS_NEVER_SET"));
    }
}
