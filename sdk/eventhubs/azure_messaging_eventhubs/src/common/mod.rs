// Copyright (c) Microsoft Corporation. All rights reserved
// Licensed under the MIT license.

// cspell:ignore sastoken

pub(crate) mod authorizer;
pub(crate) mod connection_string;
pub(crate) mod management;
pub(crate) mod recoverable;
pub mod retry;
pub(crate) mod sas_credential;
pub(crate) mod user_agent;

// Public API
pub(crate) use management::ManagementInstance;
pub(crate) use retry::recover_azure_operation;

/// The AMQP claims-based-security (CBS) token type for a Shared Access
/// Signature. JWT (Entra) tokens use the default type, signalled by `None`.
pub(crate) const SAS_TOKEN_TYPE: &str = "servicebus.windows.net:sastoken";

pub(crate) fn validate_idle_timeout(
    idle_timeout: Option<azure_core::time::Duration>,
) -> crate::Result<()> {
    let Some(idle_timeout) = idle_timeout else {
        return Ok(());
    };
    let milliseconds = idle_timeout.whole_milliseconds();
    if !(1..=i128::from(u32::MAX)).contains(&milliseconds) {
        return Err(crate::EventHubsError::with_message(format!(
            "The idle timeout must be between 1 and {} milliseconds.",
            u32::MAX
        )));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {

    use azure_core::{sleep::sleep, time::Duration, Result};
    use tokio::select;
    use tracing::info;

    /// This function is used to force errors in an EventHubs client.
    ///
    /// It will run the provided test function and then force an error on the producer after a stable duration.
    pub(crate) async fn force_errors<C: Clone, T: AsyncFn(C), E: Fn(C)>(
        context: C,
        test: T,
        force_error: E,
        force_error_duration: Duration,
        test_duration: Duration,
    ) -> Result<()> {
        // This function is used to force errors in the event hub producer.
        // It will be used in tests to ensure that the producer can handle errors gracefully.
        select! {
            // Run the test function. Normally this function is not expected to return.
            _ = async {
                test(context.clone()).await;
                info!("Test completed successfully");
            } => {
                info!("Returning from test");
                Ok::<(),azure_core::Error>(())
            },
            // Force an error on the producer after waiting for the client to stabilize.
            _ = async {
            sleep(force_error_duration).await;
            info!("Forcing error on producer");
            force_error(context.clone());
            sleep(test_duration).await;

        } => { info!("Forcing error on producer"); Ok(()) }
        // Overall test duration - this ensures that we recover to a stable state with no errors.
        _ = sleep(test_duration) => { info!("Test expired"); Ok(()) }
        }
    }
}
