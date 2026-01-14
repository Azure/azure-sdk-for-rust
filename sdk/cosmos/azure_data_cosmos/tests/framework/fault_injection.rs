// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fault injection utility for testing Azure Cosmos DB client behavior.
//!
//! This module provides a `FaultInjectionPolicy` that can be used as a custom transport policy
//! to intercept requests at the transport layer and inject errors for testing purposes.

use async_trait::async_trait;
use azure_core::{
    error::{Error, ErrorKind},
    http::{
        headers::{HeaderName, Headers, CONTENT_TYPE},
        policies::{Policy, PolicyResult},
        AsyncRawResponse, Context, Request, StatusCode,
    },
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// Type alias for predicate functions that determine if a fault should be applied to a request.
pub type PredicateFn = Arc<dyn Fn(&Request) -> bool + Send + Sync>;

/// Type alias for fault factory functions that create errors to inject.
pub type FaultFactoryFn = Arc<dyn Fn(&Request) -> Error + Send + Sync>;

/// Type alias for response transformation functions.
pub type ResponseTransformFn =
    Arc<dyn Fn(&Request, AsyncRawResponse) -> AsyncRawResponse + Send + Sync>;

/// Configuration for a single fault injection rule.
#[derive(Clone)]
struct FaultRule {
    /// Predicate to determine if this fault should be applied.
    predicate: PredicateFn,
    /// Factory function to create the error to inject.
    fault_factory: FaultFactoryFn,
    /// Optional maximum number of times this fault can be applied per request.
    max_count: Option<usize>,
    /// Current count of times this fault has been applied.
    current_count: usize,
    /// Optional response to return after max_count is reached.
    #[allow(clippy::type_complexity)]
    after_max_count: Option<Arc<dyn Fn(&Request) -> AsyncRawResponse + Send + Sync>>,
}

/// Configuration for a response transformation rule.
#[derive(Clone)]
struct TransformRule {
    /// Predicate to determine if this transformation should be applied.
    predicate: PredicateFn,
    /// Transformation function to apply to the response.
    transform: ResponseTransformFn,
}

/// A policy that intercepts requests and injects faults for testing.
///
/// This policy can be configured with predicates and fault factories to simulate
/// various error conditions that might occur in production.
#[derive(Clone)]
pub struct FaultInjectionPolicy {
    /// The underlying transport policy to delegate to after applying faults.
    inner_policy: Arc<dyn Policy>,
    /// List of fault injection rules.
    faults: Arc<Mutex<Vec<FaultRule>>>,
    /// List of response transformation rules.
    response_transforms: Arc<Mutex<Vec<TransformRule>>>,
    /// Counters for tracking fault injection statistics.
    counters: Arc<Mutex<HashMap<String, usize>>>,
}

impl std::fmt::Debug for FaultInjectionPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FaultInjectionPolicy")
            .field("faults_count", &self.faults.lock().unwrap().len())
            .field(
                "transforms_count",
                &self.response_transforms.lock().unwrap().len(),
            )
            .finish()
    }
}

impl FaultInjectionPolicy {
    /// Creates a new `FaultInjectionPolicy` wrapping the given inner policy.
    pub fn new(inner_policy: Arc<dyn Policy>) -> Self {
        let mut counters = HashMap::new();
        counters.insert("error_with_counter".to_string(), 0);

        Self {
            inner_policy,
            faults: Arc::new(Mutex::new(Vec::new())),
            response_transforms: Arc::new(Mutex::new(Vec::new())),
            counters: Arc::new(Mutex::new(counters)),
        }
    }

    /// Adds a fault injection rule.
    ///
    /// # Arguments
    /// * `predicate` - Function that returns true if the fault should be applied to the request.
    /// * `fault_factory` - Function that creates the error to inject.
    /// * `max_count` - Optional maximum number of times the fault can be applied.
    /// * `after_max_count` - Optional function to return a response after max_count is reached.
    #[allow(clippy::type_complexity)]
    pub fn add_fault(
        &self,
        predicate: PredicateFn,
        fault_factory: FaultFactoryFn,
        max_count: Option<usize>,
        after_max_count: Option<Arc<dyn Fn(&Request) -> AsyncRawResponse + Send + Sync>>,
    ) {
        let rule = FaultRule {
            predicate,
            fault_factory,
            max_count,
            current_count: 0,
            after_max_count,
        };
        self.faults.lock().unwrap().push(rule);
    }

    /// Adds a response transformation rule.
    ///
    /// # Arguments
    /// * `predicate` - Function that returns true if the transformation should be applied.
    /// * `transform` - Function that transforms the response.
    pub fn add_response_transformation(
        &self,
        predicate: PredicateFn,
        transform: ResponseTransformFn,
    ) {
        let rule = TransformRule {
            predicate,
            transform,
        };
        self.response_transforms.lock().unwrap().push(rule);
    }

    /// Resets all counters to zero.
    pub fn reset_counters(&self) {
        let mut counters = self.counters.lock().unwrap();
        for value in counters.values_mut() {
            *value = 0;
        }
    }

    /// Gets the current value of a counter.
    pub fn get_counter(&self, name: &str) -> Option<usize> {
        self.counters.lock().unwrap().get(name).copied()
    }

    /// Increments the "error_with_counter" counter and returns the error.
    pub fn error_with_counter(&self, error: Error) -> Error {
        let mut counters = self.counters.lock().unwrap();
        if let Some(count) = counters.get_mut("error_with_counter") {
            *count += 1;
        }
        error
    }

    /// Clears all fault injection rules.
    pub fn clear_faults(&self) {
        self.faults.lock().unwrap().clear();
    }

    /// Clears all response transformation rules.
    pub fn clear_transforms(&self) {
        self.response_transforms.lock().unwrap().clear();
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for FaultInjectionPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        _next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // Check for matching fault rules - extract info without holding lock across await
        let fault_to_apply: Option<(Error, Option<AsyncRawResponse>)> = {
            let mut faults = self.faults.lock().unwrap();
            let matching_fault = faults.iter_mut().find(|rule| (rule.predicate)(request));

            if let Some(fault_rule) = matching_fault {
                // Check if we should still inject this fault
                if let Some(max_count) = fault_rule.max_count {
                    if fault_rule.current_count >= max_count {
                        // Reset the counter
                        fault_rule.current_count = 0;

                        // If there's an after_max_count handler, use it
                        if let Some(handler) = &fault_rule.after_max_count {
                            let response = handler(request);
                            Some((
                                Error::with_message(ErrorKind::Other, "Skipped after max"),
                                Some(response),
                            ))
                        } else {
                            // Skip fault injection and proceed normally
                            None
                        }
                    } else {
                        fault_rule.current_count += 1;
                        Some(((fault_rule.fault_factory)(request), None))
                    }
                } else {
                    // No max count, always inject
                    Some(((fault_rule.fault_factory)(request), None))
                }
            } else {
                None
            }
        }; // Lock is released here

        // Handle the fault if one was found
        match fault_to_apply {
            Some((_, Some(response))) => return Ok(response),
            Some((error, None)) => return Err(error),
            None => {}
        }

        // No fault matched, proceed with the actual request
        let response = self.inner_policy.send(ctx, request, &[]).await?;

        // Check for response transformations
        let should_transform = {
            let transforms = self.response_transforms.lock().unwrap();
            transforms.iter().find_map(|rule| {
                if (rule.predicate)(request) {
                    Some(rule.transform.clone())
                } else {
                    None
                }
            })
        }; // Lock is released here

        if let Some(transform) = should_transform {
            Ok(transform(request, response))
        } else {
            Ok(response)
        }
    }
}

// Predicate helper functions

/// Creates a predicate that checks if the request URL contains a specific ID.
pub fn predicate_url_contains_id(id_value: String) -> PredicateFn {
    Arc::new(move |request: &Request| request.url().as_str().contains(&id_value))
}

/// Creates a predicate that checks if the request targets a specific region endpoint.
pub fn predicate_targets_region(region_endpoint: String) -> PredicateFn {
    Arc::new(move |request: &Request| request.url().as_str().starts_with(&region_endpoint))
}

/// Creates a predicate that checks if the request payload contains a specific ID.
pub fn predicate_req_payload_contains_id(id_value: String) -> PredicateFn {
    Arc::new(move |request: &Request| {
        if request.body().is_empty() {
            return false;
        }
        let body_bytes: azure_core::Bytes = request.body().into();
        let body_str = String::from_utf8_lossy(&body_bytes);
        body_str.contains(&format!(r#""id":"{}""#, id_value))
    })
}

/// Creates a predicate that checks if the request payload contains a specific field.
pub fn predicate_req_payload_contains_field(
    field_name: String,
    field_value: Option<String>,
) -> PredicateFn {
    Arc::new(move |request: &Request| {
        if request.body().is_empty() {
            return false;
        }
        let body_bytes: azure_core::Bytes = request.body().into();
        let body_str = String::from_utf8_lossy(&body_bytes);
        match &field_value {
            None => body_str.contains(&format!(r#""{}":"#, field_name)),
            Some(val) => body_str.contains(&format!(r#""{}":"{}""#, field_name, val)),
        }
    })
}

/// Creates a predicate that checks if the request is for a document with a specific ID.
pub fn predicate_req_for_document_with_id(id_value: String) -> PredicateFn {
    let url_pred = predicate_url_contains_id(id_value.clone());
    let payload_pred = predicate_req_payload_contains_id(id_value);
    Arc::new(move |request: &Request| url_pred(request) || payload_pred(request))
}

/// Creates a predicate that checks if the request is a database account read operation.
pub fn predicate_is_database_account_call() -> PredicateFn {
    Arc::new(|request: &Request| {
        // Check if this is a database account read operation
        // In Cosmos DB, this is typically indicated by specific headers or URL patterns
        let path = request.url().path();
        path == "/" || path.is_empty() || request.url().as_str().ends_with("/")
    })
}

/// Creates a predicate that checks if the request is a document operation.
pub fn predicate_is_document_operation() -> PredicateFn {
    Arc::new(|request: &Request| {
        request.url().path().contains("/docs/")
            || request
                .headers()
                .get_str(&HeaderName::from_static("x-ms-documentdb-partitionkey"))
                .is_ok()
    })
}

/// Creates a predicate that checks if the request is for a specific resource type.
pub fn predicate_is_resource_type(resource_type: String) -> PredicateFn {
    Arc::new(move |request: &Request| {
        request
            .headers()
            .get_str(&HeaderName::from_static("x-ms-documentdb-resourcetype"))
            .map(|v| v == resource_type)
            .unwrap_or(false)
    })
}

/// Creates a predicate that checks if the request is a specific operation type.
pub fn predicate_is_operation_type(operation_type: String) -> PredicateFn {
    Arc::new(move |request: &Request| {
        request
            .headers()
            .get_str(&HeaderName::from_static("x-ms-documentdb-operationtype"))
            .map(|v| v == operation_type)
            .unwrap_or(false)
    })
}

/// Creates a predicate that checks if the request is a write operation to a specific URI prefix.
pub fn predicate_is_write_operation(uri_prefix: String) -> PredicateFn {
    Arc::new(move |request: &Request| {
        let is_write = matches!(
            request.method().as_str(),
            "POST" | "PUT" | "PATCH" | "DELETE"
        );
        is_write && request.url().as_str().contains(&uri_prefix)
    })
}

// Fault factory functions

/// Creates a fault factory that returns a write forbidden error.
pub fn error_write_forbidden() -> FaultFactoryFn {
    Arc::new(|_request: &Request| {
        Error::with_message(
            ErrorKind::HttpResponse {
                status: StatusCode::Forbidden,
                error_code: Some("WriteForbidden".to_string()),
                raw_response: None,
            },
            "Injected error disallowing writes in this region.",
        )
    })
}

/// Creates a fault factory that returns a request timeout error.
pub fn error_request_timeout() -> FaultFactoryFn {
    Arc::new(|_request: &Request| {
        Error::with_message(
            ErrorKind::HttpResponse {
                status: StatusCode::RequestTimeout,
                error_code: Some("RequestTimeout".to_string()),
                raw_response: None,
            },
            "Injected request timeout error.",
        )
    })
}

/// Creates a fault factory that returns an internal server error.
pub fn error_internal_server_error() -> FaultFactoryFn {
    Arc::new(|_request: &Request| {
        Error::with_message(
            ErrorKind::HttpResponse {
                status: StatusCode::InternalServerError,
                error_code: Some("InternalServerError".to_string()),
                raw_response: None,
            },
            "Injected internal server error.",
        )
    })
}

/// Creates a fault factory that returns a service unavailable error (simulating region down).
pub fn error_region_down() -> FaultFactoryFn {
    Arc::new(|_request: &Request| {
        Error::new(
            ErrorKind::Io,
            std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Injected region down error.",
            ),
        )
    })
}

/// Creates a fault factory that returns a service response error.
pub fn error_service_response() -> FaultFactoryFn {
    Arc::new(|_request: &Request| {
        Error::with_message(ErrorKind::Other, "Injected service response error.")
    })
}

/// Creates a fault factory that delays for a specified duration before returning an error.
pub fn error_after_delay(delay_ms: u64, error_factory: FaultFactoryFn) -> FaultFactoryFn {
    Arc::new(move |request: &Request| {
        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        error_factory(request)
    })
}

// Response transformation functions

/// Creates a mock response with the given status code and optional JSON body.
pub fn create_mock_response(
    status: StatusCode,
    json_body: Option<serde_json::Value>,
) -> AsyncRawResponse {
    let body = json_body
        .map(|v| serde_json::to_vec(&v).unwrap_or_default())
        .unwrap_or_default();

    let mut headers = Headers::new();
    if !body.is_empty() {
        headers.insert(CONTENT_TYPE, "application/json");
    }

    AsyncRawResponse::from_bytes(status, headers, body)
}
