// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines conditions for when fault injection rules should be applied.

use super::FaultOperationType;
use crate::diagnostics::TransportKind;
use crate::options::Region;

/// Defines the condition under which a fault injection rule should be applied.
#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct FaultInjectionCondition {
    operation_type: Option<FaultOperationType>,
    region: Option<Region>,
    container_id: Option<String>,
    transport_kind: Option<TransportKind>,
}

impl FaultInjectionCondition {
    /// Returns the operation type to which the fault injection applies.
    pub fn operation_type(&self) -> Option<FaultOperationType> {
        self.operation_type
    }

    /// Returns the region to which the fault injection applies.
    pub fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    /// Returns the container ID to which the fault injection applies.
    pub fn container_id(&self) -> Option<&str> {
        self.container_id.as_deref()
    }

    /// Returns the transport kind to which the fault injection applies.
    ///
    /// When `Some`, the rule only matches requests sent over the specified
    /// transport (e.g. `TransportKind::GatewayV2`). When `None`, the rule
    /// matches every transport (including metadata, gateway, and Gateway 2.0).
    pub fn transport_kind(&self) -> Option<TransportKind> {
        self.transport_kind
    }
}

/// Builder for creating a FaultInjectionCondition.
#[derive(Default)]
#[non_exhaustive]
pub struct FaultInjectionConditionBuilder {
    operation_type: Option<FaultOperationType>,
    region: Option<Region>,
    container_id: Option<String>,
    transport_kind: Option<TransportKind>,
}

impl FaultInjectionConditionBuilder {
    /// Creates a new FaultInjectionConditionBuilder with default values.
    pub fn new() -> Self {
        Self {
            operation_type: None,
            region: None,
            container_id: None,
            transport_kind: None,
        }
    }

    /// Sets the operation type to which the fault injection applies.
    pub fn with_operation_type(mut self, operation_type: FaultOperationType) -> Self {
        self.operation_type = Some(operation_type);
        self
    }

    /// Sets the region to which the fault injection applies.
    pub fn with_region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Sets the container ID to which the fault injection applies.
    pub fn with_container_id(mut self, container_id: impl Into<String>) -> Self {
        self.container_id = Some(container_id.into());
        self
    }

    /// Restricts the rule to a specific transport kind.
    ///
    /// Use this to scope a fault to (for example) only Gateway 2.0 traffic
    /// (`TransportKind::GatewayV2`) while leaving the standard gateway path
    /// untouched. When unset, the rule applies regardless of which transport
    /// carried the request.
    pub fn with_transport_kind(mut self, transport_kind: TransportKind) -> Self {
        self.transport_kind = Some(transport_kind);
        self
    }

    /// Builds the FaultInjectionCondition.
    pub fn build(self) -> FaultInjectionCondition {
        FaultInjectionCondition {
            operation_type: self.operation_type,
            region: self.region,
            container_id: self.container_id,
            transport_kind: self.transport_kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FaultInjectionConditionBuilder;
    use crate::diagnostics::TransportKind;

    #[test]
    fn builder_default() {
        let builder = FaultInjectionConditionBuilder::default();
        let condition = builder.build();
        assert!(condition.operation_type().is_none());
        assert!(condition.region().is_none());
        assert!(condition.transport_kind().is_none());
    }

    #[test]
    fn with_transport_kind_round_trip() {
        let condition = FaultInjectionConditionBuilder::new()
            .with_transport_kind(TransportKind::GatewayV2)
            .build();
        assert_eq!(condition.transport_kind(), Some(TransportKind::GatewayV2));
    }
}
