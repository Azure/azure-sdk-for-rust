// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines conditions for when fault injection rules should be applied.

use super::FaultOperationType;
use crate::regions::RegionName;

/// Defines the condition under which a fault injection rule should be applied.
#[derive(Clone, Default, Debug)]
pub struct FaultInjectionCondition {
    /// The type of operation to which the fault injection applies.
    pub operation_type: Option<FaultOperationType>,
    /// The region to which the fault injection applies.
    pub region: Option<RegionName>,
    /// The container ID to which the fault injection applies.
    pub container_id: Option<String>,
}

/// Builder for creating a FaultInjectionCondition.
#[derive(Default)]
pub struct FaultInjectionConditionBuilder {
    operation_type: Option<FaultOperationType>,
    region: Option<RegionName>,
    container_id: Option<String>,
}

impl FaultInjectionConditionBuilder {
    /// Creates a new FaultInjectionConditionBuilder with default values.
    pub fn new() -> Self {
        Self {
            operation_type: None,
            region: None,
            container_id: None,
        }
    }

    /// Sets the operation type to which the fault injection applies.
    pub fn with_operation_type(mut self, operation_type: FaultOperationType) -> Self {
        self.operation_type = Some(operation_type);
        self
    }

    /// Sets the region to which the fault injection applies.
    pub fn with_region(mut self, region: RegionName) -> Self {
        self.region = Some(region);
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
            operation_type: self.operation_type,
            region: self.region,
            container_id: self.container_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FaultInjectionConditionBuilder;

    #[test]
    fn builder_default() {
        let builder = FaultInjectionConditionBuilder::default();
        let condition = builder.build();
        assert!(condition.operation_type.is_none());
        assert!(condition.region.is_none());
        assert!(condition.container_id.is_none());
    }
}
