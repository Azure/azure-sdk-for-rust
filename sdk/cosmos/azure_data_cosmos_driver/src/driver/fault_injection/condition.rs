// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines conditions for when fault injection rules should be applied.

use super::FaultOperationType;
use crate::options::Region;

/// Defines the condition under which a fault injection rule should be applied.
#[derive(Clone, Default, Debug)]
pub struct FaultInjectionCondition {
    /// The type of operation to which the fault injection applies.
    pub operation_type: Option<FaultOperationType>,
    /// The region to which the fault injection applies.
    pub region: Option<Region>,
    /// The container ID to which the fault injection applies.
    pub container_id: Option<String>,
}

/// Builder for creating a FaultInjectionCondition.
#[derive(Default)]
pub struct FaultInjectionConditionBuilder {
    operation_type: Option<FaultOperationType>,
    region: Option<Region>,
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
    pub fn with_region(mut self, region: Region) -> Self {
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
    use super::{FaultInjectionCondition, FaultInjectionConditionBuilder};
    use crate::driver::fault_injection::FaultOperationType;
    use crate::options::Region;

    #[test]
    fn builder_default_produces_empty_condition() {
        let condition = FaultInjectionConditionBuilder::default().build();
        assert_eq!(
            (
                condition.operation_type,
                condition.region,
                condition.container_id
            ),
            (None, None, None)
        );
    }

    #[test]
    fn builder_sets_all_fields() {
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::ReadItem)
            .with_region(Region::new("East US"))
            .with_container_id("my-container")
            .build();
        assert_eq!(condition.operation_type, Some(FaultOperationType::ReadItem));
        assert_eq!(condition.region, Some(Region::new("East US")));
        assert_eq!(condition.container_id, Some("my-container".to_string()));
    }

    #[test]
    fn default_trait_produces_empty_condition() {
        let condition = FaultInjectionCondition::default();
        assert_eq!(
            (
                condition.operation_type,
                condition.region,
                condition.container_id
            ),
            (None, None, None)
        );
    }
}
