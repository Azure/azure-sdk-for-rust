//! Provides a framework for integration tests for the Azure Storage service.
//!
//! The framework allows tests to easily run against real Azure Storage instances, the local emulator, or a mock server using test proxy.

mod test_account;

pub use test_account::TestAccount;
