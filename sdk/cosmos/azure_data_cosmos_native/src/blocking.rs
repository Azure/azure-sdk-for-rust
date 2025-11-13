use std::{future::Future, sync::OnceLock};
use tokio::runtime::Runtime;

// Centralized runtime - single instance per process (following Azure SDK pattern)
// https://github.com/Azure/azure-sdk-for-cpp/blob/main/sdk/core/azure-core-amqp/src/impl/rust_amqp/rust_amqp/rust_wrapper/src/amqp/connection.rs#L100-L107
pub static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    let runtime = RUNTIME.get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"));
    runtime.block_on(future)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocking_runtime_initialization() {
        let result1 = block_on(async { 42 });
        let result2 = block_on(async { 24 });

        assert_eq!(result1, 42);
        assert_eq!(result2, 24);
    }

    #[test]
    fn test_blocking_async_operation() {
        let result = block_on(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            "completed"
        });

        assert_eq!(result, "completed");
    }

    #[test]
    fn test_runtime_singleton() {
        block_on(async { 1 });
        let runtime1 = RUNTIME.get();

        block_on(async { 2 });
        let runtime2 = RUNTIME.get();

        assert!(runtime1.is_some());
        assert!(runtime2.is_some());
        assert!(std::ptr::eq(runtime1.unwrap(), runtime2.unwrap()));
    }
}
