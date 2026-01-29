use crate::options::CosmosClientOptions;
use std::time::Duration;

/// A fault injection rule that defines when and how to inject faults.
#[derive(Clone, Debug)]
pub struct FaultInjectionRule {
    /// The condition under which to inject the fault.
    pub condition: FaultInjectionCondition,
    /// The result to inject when the condition is met.
    pub result: Box<dyn FaultInjectionResult>,
    /// Duration for which the fault injection is active.
    /// default is infinite duration.
    pub duration: Duration,
    /// Delay before starting the fault injection.
    /// Default is no delay.
    pub start_delay: Duration,
    /// Set the total hit limit of the rule. The rule will be not applicable anymore once it has applied hitLimit times.
    ///
    /// By default, there is no limit.
    pub hit_limit: Option<u32>,
    /// Unique identifier for the fault injection scenario.
    pub id: String,
}

use std::sync::Arc;
use azure_core::http::Transport;
use super::fault_http_client::FaultClient;

/// Builder for creating a fault injection client.
pub struct FaultInjectionClientBuilder {
    /// The fault injection rules to apply.
    /// Rules will be applied from first to last.
    rules: Vec<FaultInjectionRule>,
}

impl FaultInjectionClientBuilder {
    /// Creates a new FaultInjectionClientBuilder.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    /// Injects the fault injection client into the CosmosClientOptions.
    /// Called after building the fault conditions.
    ///
    /// This wraps the existing transport (or creates a default one) with the fault injection client.
    pub fn inject(&self, mut options: CosmosClientOptions) -> CosmosClientOptions {
        // Create a default http client
        let inner_client: Arc<dyn azure_core::http::HttpClient> = azure_core::http::new_http_client();

        let fault_client = FaultClient::new(inner_client, self.rules.clone());
        options.client_options.transport = Some(Transport::new(Arc::new(fault_client)));
        options.fault_injection_enabled = true;

        options
    }

    /// Adds a fault injection rule to the builder.
    pub fn with_rule(&mut self, rule: FaultInjectionRule) -> &mut Self {
        self.rules.push(rule);
        self
    }
}

impl Default for FaultInjectionClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating a fault injection rule.
pub struct FaultInjectionRuleBuilder {
    /// The condition under which to inject the fault.
    condition: FaultInjectionCondition,
    /// The result to inject when the condition is met.
    result: Box<dyn FaultInjectionResult>,
    /// Duration for which the fault injection is active.
    /// default is infinite duration.
    duration: Duration,
    /// Delay before starting the fault injection.
    /// Default is no delay.
    start_delay: Duration,
    /// Set the total hit limit of the rule. The rule will be not applicable anymore once it has applied hitLimit times.
    ///
    /// By default, there is no limit.
    hit_limit: Option<u32>,
    /// Unique identifier for the fault injection scenario.
    id: String,
}

impl FaultInjectionRuleBuilder {
    /// Creates a new FaultInjectionRuleBuilder with default values.
    pub fn new(id: impl Into<String>, result: impl FaultInjectionResult + 'static) -> Self {
        Self {
            condition: FaultInjectionCondition::default(),
            result: Box::new(result),
            duration: Duration::MAX, // Infinite duration by default
            start_delay: Duration::ZERO,
            hit_limit: None,
            id: id.into(),
        }
    }

    /// Sets the condition for when to inject the fault.
    pub fn with_condition(mut self, condition: FaultInjectionCondition) -> Self {
        self.condition = condition;
        self
    }

    /// Sets the result to inject when the condition is met.
    pub fn with_result(mut self, result: impl FaultInjectionResult + 'static) -> Self {
        self.result = Box::new(result);
        self
    }

    /// Sets the duration for which the fault injection is active.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the delay before starting the fault injection.
    pub fn with_start_delay(mut self, start_delay: Duration) -> Self {
        self.start_delay = start_delay;
        self
    }

    /// Sets the total hit limit of the rule.
    pub fn with_hit_limit(mut self, hit_limit: u32) -> Self {
        self.hit_limit = Some(hit_limit);
        self
    }
    
    /// Builds the FaultInjectionRule.
    ///
    /// # Panics
    /// Panics if no result has been set.
    pub fn build(self) -> FaultInjectionRule {
        FaultInjectionRule {
            condition: self.condition,
            result: self.result,
            duration: self.duration,
            start_delay: self.start_delay,
            hit_limit: self.hit_limit,
            id: self.id,
        }
    }
}

/// Defines the condition under which a fault injection rule should be applied.
#[derive(Clone, Default, Debug)]
pub struct FaultInjectionCondition {
    /// The endpoints to which the fault injection applies.
    /// Either the region or the endpoints must be specified.
    pub endpoints: Option<Vec<String>>,
    /// The type of operation to which the fault injection applies.
    pub operation_type: Option<FaultOperationType>,
    /// The region to which the fault injection applies.
    /// Either the endpoints or the region must be specified.
    pub region: Option<String>,
    /// The partition key range ID to which the fault injection applies.
    pub partition_key_range_id: Option<String>,
    /// The container ID to which the fault injection applies.
    pub container_id: Option<String>,
}

/// The type of operation to which the fault injection applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FaultOperationType {
    /// Read items.
    #[default]
    ReadItem,
    /// Query items.
    QueryItem,
    /// Create item.
    CreateItem,
    /// Upsert item.
    UpsertItem,
    /// Replace item.
    ReplaceItem,
    /// Delete item.
    DeleteItem,
    /// Patch item.
    PatchItem,
    /// Batch item.
    BatchItem,
    /// Read change feed items.
    ChangeFeedItem,
    /// Read container request.
    MetadataReadContainer,
    /// Read database account request.
    MetadataReadDatabaseAccount,
    /// Query query plan request.
    MetadataQueryPlan,
    /// Partition key ranges request.
    MetadataPartitionKeyRanges,
}

impl From<FaultOperationType> for &'static str {
    fn from(op: FaultOperationType) -> Self {
        match op {
            FaultOperationType::ReadItem => "ReadItem",
            FaultOperationType::QueryItem => "QueryItem",
            FaultOperationType::CreateItem => "CreateItem",
            FaultOperationType::UpsertItem => "UpsertItem",
            FaultOperationType::ReplaceItem => "ReplaceItem",
            FaultOperationType::DeleteItem => "DeleteItem",
            FaultOperationType::PatchItem => "PatchItem",
            FaultOperationType::BatchItem => "BatchItem",
            FaultOperationType::ChangeFeedItem => "ChangeFeedItem",
            FaultOperationType::MetadataReadContainer => "MetadataReadContainer",
            FaultOperationType::MetadataReadDatabaseAccount => "MetadataReadDatabaseAccount",
            FaultOperationType::MetadataQueryPlan => "MetadataQueryPlan",
            FaultOperationType::MetadataPartitionKeyRanges => "MetadataPartitionKeyRanges",
        }
    }
}

/// Represents different server error types that can be injected for fault testing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultInjectionServerErrorType {
    /// 449 from server.
    RetryWith,
    /// 500 from server.
    InternalServerError,
    /// 429 from server.
    TooManyRequests,
    /// 404-1002 from server.
    ReadSessionNotAvailable,
    /// 408 from server.
    Timeout,
    /// Response delay, when it is over request timeout, can simulate transit timeout.
    ResponseDelay,
    /// Simulate high channel acquisition, when it is over connection timeout, can simulate connectionTimeoutException.
    ConnectionDelay,
    /// Simulate service unavailable (503).
    ServiceUnavailable,
    /// 410-1002 from server.
    PartitionIsGone,
}

/// Trait for fault injection results.
pub trait FaultInjectionResult: Send + Sync + std::fmt::Debug {
    /// Clones this result into a boxed trait object.
    fn clone_box(&self) -> Box<dyn FaultInjectionResult>;

    /// Returns this result as a server error if it is one.
    fn as_server_error(&self) -> Option<&FaultInjectionServerError>;
}

impl Clone for Box<dyn FaultInjectionResult> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Represents a server error to be injected.
#[derive(Clone, Debug)]
pub struct FaultInjectionServerError {
    /// The type of server error to inject.
    pub error_type: FaultInjectionServerErrorType,
    /// Number of times to inject the error.
    /// Default is that it will be injected forever.
    pub times: Option<u32>,
    /// Delay before injecting the error.
    /// default is no delay.
    pub delay: Duration,
    /// Probability of injecting the error (0.0 to 1.0).
    /// Default is 1.0 (always inject).
    pub probability: f32,
}

impl FaultInjectionResult for FaultInjectionServerError {
    fn clone_box(&self) -> Box<dyn FaultInjectionResult> {
        Box::new(self.clone())
    }

    fn as_server_error(&self) -> Option<&FaultInjectionServerError> {
        Some(self)
    }
}

/// Builder for creating a FaultInjectionServerError.
pub struct FaultInjectionServerErrorBuilder {
    error_type: FaultInjectionServerErrorType,
    times: Option<u32>,
    delay: Duration,
    probability: f32,
}

impl FaultInjectionServerErrorBuilder {
    /// Creates a new FaultInjectionServerErrorBuilder with the specified error type.
    pub fn new(error_type: FaultInjectionServerErrorType) -> Self {
        Self {
            error_type,
            times: None,
            delay: Duration::ZERO,
            probability: 1.0,
        }
    }

    /// Sets the number of times to inject the error.
    pub fn with_times(mut self, times: u32) -> Self {
        self.times = Some(times);
        self
    }

    /// Sets the delay before injecting the error.
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Sets the probability of injecting the error (0.0 to 1.0).
    pub fn with_probability(mut self, probability: f32) -> Self {
        self.probability = probability.clamp(0.0, 1.0);
        self
    }

    /// Builds the FaultInjectionServerError.
    pub fn build(self) -> FaultInjectionServerError {
        FaultInjectionServerError {
            error_type: self.error_type,
            times: self.times,
            delay: self.delay,
            probability: self.probability,
        }
    }
}

/// Builder for creating a FaultInjectionCondition.
pub struct FaultInjectionConditionBuilder {
    endpoints: Option<Vec<String>>,
    operation_type: Option<FaultOperationType>,
    region: Option<String>,
    partition_key_range_id: Option<String>,
    container_id: Option<String>,
}

impl FaultInjectionConditionBuilder {
    /// Creates a new FaultInjectionConditionBuilder with default values.
    pub fn new() -> Self {
        Self {
            endpoints: None,
            operation_type: None,
            region: None,
            partition_key_range_id: None,
            container_id: None,
        }
    }

    /// Sets the endpoints to which the fault injection applies.
    pub fn with_endpoints(mut self, endpoints: Vec<String>) -> Self {
        self.endpoints = Some(endpoints);
        self
    }

    /// Adds an endpoint to which the fault injection applies.
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoints
            .get_or_insert_with(Vec::new)
            .push(endpoint.into());
        self
    }

    /// Sets the operation type to which the fault injection applies.
    pub fn with_operation_type(mut self, operation_type: FaultOperationType) -> Self {
        self.operation_type = Some(operation_type);
        self
    }

    /// Sets the region to which the fault injection applies.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Sets the feed range to which the fault injection applies.
    pub fn with_partition_key_range_id(mut self, feed_range: impl Into<String>) -> Self {
        self.partition_key_range_id = Some(feed_range.into());
        self
    }

    /// Sets the container ID to which the fault injection applies.
    pub fn with_container_id(mut self, container_id: impl Into<String>) -> Self {
        self.container_id = Some(container_id.into());
        self
    }

    /// Builds the FaultInjectionCondition.
    pub fn build(self) -> FaultInjectionCondition {
        FaultInjectionCondition {
            endpoints: self.endpoints,
            operation_type: self.operation_type,
            region: self.region,
            partition_key_range_id: self.partition_key_range_id,
            container_id: self.container_id,
        }
    }
}

impl Default for FaultInjectionConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

