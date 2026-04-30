// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines conditions for when fault injection rules should be applied.

use super::{FaultOperationType, TransportKind};
use crate::regions::Region;

/// Defines the condition under which a fault injection rule should be applied.
#[derive(Clone, Default, Debug)]
pub struct FaultInjectionCondition {
    /// The type of operation to which the fault injection applies.
    pub operation_type: Option<FaultOperationType>,
    /// The region to which the fault injection applies.
    pub region: Option<Region>,
    /// The container ID to which the fault injection applies.
    pub container_id: Option<String>,
    /// Restricts the rule to a specific transport kind (Gateway 1.x vs
    /// Gateway 2.0). When `None`, the rule applies regardless of which
    /// dataplane transport carries the request. When `Some`, the rule
    /// only applies to clients bound to that transport — metadata
    /// clients always skip the rule.
    pub transport_kind: Option<TransportKind>,
}

/// Builder for creating a FaultInjectionCondition.
#[derive(Default)]
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

    /// Restricts the rule to a specific transport kind (e.g.,
    /// [`TransportKind::Gateway20`]). When set, the rule only matches
    /// requests carried by the matching transport.
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
    use super::{FaultInjectionConditionBuilder, TransportKind};

    #[test]
    fn builder_default() {
        let builder = FaultInjectionConditionBuilder::default();
        let condition = builder.build();
        assert!(condition.operation_type.is_none());
        assert!(condition.region.is_none());
        assert!(condition.container_id.is_none());
        assert!(condition.transport_kind.is_none());
    }

    #[test]
    fn with_transport_kind_sets_field() {
        let condition = FaultInjectionConditionBuilder::new()
            .with_transport_kind(TransportKind::Gateway20)
            .build();
        assert_eq!(condition.transport_kind, Some(TransportKind::Gateway20));
    }
}
