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
    resolve_broker_address(read_broker_address(), is_env_flag_set(TEST_BROKER_REQUIRED))
}

/// Reads and normalizes the broker address. An absent or blank value is `None`.
fn read_broker_address() -> Option<String> {
    std::env::var(TEST_BROKER_ADDRESS)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Decides what a broker test does, given the address and the strictness flag.
///
/// This holds the whole rule, and it reads no environment variable, so the tests
/// below can cover every branch without a shared global.
///
/// # Panics
///
/// Panics when `required` is true and `address` is `None`.
fn resolve_broker_address(address: Option<String>, required: bool) -> Option<String> {
    if address.is_some() {
        return address;
    }

    assert!(
        !required,
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
        Ok(value) => is_flag_value_set(&value),
        Err(_) => false,
    }
}

/// Reads one flag value. Kept separate from the environment so it can be tested.
fn is_flag_value_set(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty() && value != "0" && !value.eq_ignore_ascii_case("false")
}

#[cfg(test)]
mod tests {
    use super::{is_env_flag_set, is_flag_value_set, resolve_broker_address};

    #[test]
    fn address_present_is_returned() {
        let address = Some("amqp://127.0.0.1:25672".to_string());
        assert_eq!(
            resolve_broker_address(address.clone(), false),
            address,
            "a present address must come back unchanged"
        );
        assert_eq!(
            resolve_broker_address(address.clone(), true),
            address,
            "the strictness flag must not change a present address"
        );
    }

    #[test]
    fn address_absent_and_not_required_skips() {
        assert_eq!(
            resolve_broker_address(None, false),
            None,
            "a developer without a broker must still run the other tests"
        );
    }

    #[test]
    #[should_panic(expected = "is absent or empty")]
    fn address_absent_and_required_panics() {
        // This is the behavior that keeps a silent skip from returning. A broker that
        // stops running must turn the pipeline red.
        let _ = resolve_broker_address(None, true);
    }

    #[test]
    fn flag_values_follow_the_documented_rule() {
        for off in ["", "   ", "0", "false", "FALSE", "False"] {
            assert!(!is_flag_value_set(off), "{off:?} must read as off");
        }
        for on in ["1", "true", "TRUE", "yes", "on"] {
            assert!(is_flag_value_set(on), "{on:?} must read as on");
        }
    }

    #[test]
    fn absent_variable_is_off() {
        assert!(!is_env_flag_set("AZURE_CORE_AMQP_FLAG_THAT_IS_NEVER_SET"));
    }
}
