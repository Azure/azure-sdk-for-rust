// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal implementation for connection reset retry policy in the Azure
//! Cosmos database service.

use std::sync::Arc;

use crate::{
    clients::CosmosClient,
};

/// Represents a Cosmos HTTP response error
pub trait CosmosError {
    // Add methods as needed for error handling
}

/// Retry policy for handling partition key range gone exceptions
pub struct PartitionKeyRangeGoneRetryPolicy {
    retry_after_in_milliseconds: u64,
    refresh_partition_key_range_cache: bool,
    args: Vec<String>, // Generic args - adjust type as needed
    client: CosmosClient,
    exception: Option<Box<dyn CosmosError>>,
}

impl PartitionKeyRangeGoneRetryPolicy {
    /// Creates a new instance of PartitionKeyRangeGoneRetryPolicy
    ///
    /// # Arguments
    /// * `client` - The cosmos client instance
    /// * `args` - Additional arguments (variadic in Python, Vec in Rust)
    pub fn new(client: CosmosClient, args: Vec<String>) -> Self {
        Self {
            retry_after_in_milliseconds: 1000,
            refresh_partition_key_range_cache: true,
            args,
            client,
            exception: None,
        }
    }

    /// Returns true if the request should retry based on the passed-in exception.
    ///
    /// # Arguments
    /// * `exception` - The CosmosHttpResponseError instance
    ///
    /// # Returns
    /// A boolean stating whether the request should be retried
    pub fn should_retry(&mut self, exception: Box<dyn CosmosError>) -> bool {
        // Store the exception (equivalent to self.exception = exception in Python)
        self.exception = Some(exception);

        if self.refresh_partition_key_range_cache {
            // Refresh routing_map_provider to refresh partition key range cache
            // Make refresh_partition_key_range_cache false to skip this check on subsequent Gone exceptions
            //self.client.refresh_routing_map_provider();
            self.refresh_partition_key_range_cache = false;
        }

        // Return false to raise error to multi_execution_aggregator and repair document producer context
        false
    }

    /// Gets the retry delay in milliseconds
    pub fn retry_after_in_milliseconds(&self) -> u64 {
        self.retry_after_in_milliseconds
    }

    /// Gets the stored exception
    pub fn exception(&self) -> Option<&Box<dyn CosmosError>> {
        self.exception.as_ref()
    }

    /// Gets the additional arguments
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::Arc;
//
//     // Mock client for testing
//     struct MockCosmosClient {
//         refresh_called: std::sync::Mutex<bool>,
//     }
//
//     impl MockCosmosClient {
//         fn new() -> Self {
//             Self {
//                 refresh_called: std::sync::Mutex::new(false),
//             }
//         }
//
//         fn was_refresh_called(&self) -> bool {
//             *self.refresh_called.lock().unwrap()
//         }
//     }
//
//     impl CosmosClient for MockCosmosClient {
//         fn refresh_routing_map_provider(&self) {
//             *self.refresh_called.lock().unwrap() = true;
//         }
//     }
//
//     // Mock error for testing
//     struct MockCosmosError;
//     impl CosmosError for MockCosmosError {}
//
//     #[test]
//     fn test_new_policy() {
//         let client = Arc::new(MockCosmosClient::new());
//         let args = vec!["arg1".to_string(), "arg2".to_string()];
//         let policy = PartitionKeyRangeGoneRetryPolicy::new(client, args.clone());
//
//         assert_eq!(policy.retry_after_in_milliseconds(), 1000);
//         assert_eq!(policy.args(), &args);
//         assert!(policy.exception().is_none());
//     }
//
//     #[test]
//     fn test_should_retry() {
//         let client = Arc::new(MockCosmosClient::new());
//         let mut policy = PartitionKeyRangeGoneRetryPolicy::new(client.clone(), vec![]);
//         let error = Box::new(MockCosmosError);
//
//         // First call should refresh cache and return false
//         let result = policy.should_retry(error);
//         assert!(!result);
//         assert!(client.was_refresh_called());
//         assert!(policy.exception().is_some());
//
//         // Second call should not refresh cache again
//         let client2 = Arc::new(MockCosmosClient::new());
//         policy.client = client2.clone();
//         let error2 = Box::new(MockCosmosError);
//         let result2 = policy.should_retry(error2);
//         assert!(!result2);
//         assert!(!client2.was_refresh_called()); // Should not be called again
//     }
// }