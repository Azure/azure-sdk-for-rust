// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{messaging::AmqpTarget, value::AmqpValue};

pub(crate) fn to_fe2o3_target(target: AmqpTarget) -> fe2o3_amqp_types::messaging::Target {
    let mut builder = fe2o3_amqp_types::messaging::Target::builder();

    if let Some(address) = target.address {
        builder = builder.address(address);
    }
    if let Some(durable) = target.durable {
        builder = builder.durable(durable.into());
    }
    if let Some(expiry_policy) = target.expiry_policy {
        builder = builder.expiry_policy(expiry_policy.into());
    }
    if let Some(timeout) = target.timeout {
        builder = builder.timeout(timeout);
    }
    if let Some(dynamic) = target.dynamic {
        builder = builder.dynamic(dynamic);
    }
    if let Some(dynamic_node_properties) = target.dynamic_node_properties {
        builder = builder.dynamic_node_properties(
            dynamic_node_properties
                .iter()
                .map(|(k, v)| {
                    (
                        fe2o3_amqp_types::primitives::Symbol::from(k.as_str()),
                        v.into(),
                    )
                })
                .collect::<fe2o3_amqp_types::definitions::Fields>(),
        );
    }
    if let Some(capabilities) = target.capabilities {
        builder = builder.capabilities(
            capabilities
                .into_iter()
                .map(|v| match v {
                    AmqpValue::Symbol(s) => fe2o3_amqp_types::primitives::Symbol::from(s.0),
                    AmqpValue::String(s) => fe2o3_amqp_types::primitives::Symbol::from(s),
                    _ => fe2o3_amqp_types::primitives::Symbol::from(format!("{:?}", v)),
                })
                .collect::<fe2o3_amqp_types::primitives::Array<
                    fe2o3_amqp_types::primitives::Symbol,
                >>(),
        );
    }
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        messaging::{TerminusDurability, TerminusExpiryPolicy},
        value::{AmqpOrderedMap, AmqpSymbol},
    };

    #[test]
    fn message_target_conversion_to_fe2o3() {
        let mut dynamic_node_properties = AmqpOrderedMap::new();
        dynamic_node_properties.insert("prop".to_string(), AmqpValue::from("val"));

        let amqp_target = AmqpTarget::builder()
            .with_address("test".to_string())
            .with_durable(TerminusDurability::UnsettledState)
            .with_expiry_policy(TerminusExpiryPolicy::SessionEnd)
            .with_timeout(95)
            .with_dynamic(false)
            .with_dynamic_node_properties(dynamic_node_properties)
            .with_capabilities(vec![AmqpValue::Symbol(AmqpSymbol::from("capability"))])
            .build();

        let fe2o3_target = to_fe2o3_target(amqp_target);

        assert_eq!(fe2o3_target.address.unwrap().as_str(), "test");
        assert_eq!(
            fe2o3_target.durable,
            fe2o3_amqp_types::messaging::TerminusDurability::UnsettledState
        );
        assert_eq!(
            fe2o3_target.expiry_policy,
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::SessionEnd
        );
        assert_eq!(fe2o3_target.timeout, 95);
        assert_eq!(fe2o3_target.dynamic, false);
        assert_eq!(
            fe2o3_target.capabilities.unwrap().as_slice(),
            &[fe2o3_amqp_types::primitives::Symbol::from("capability")]
        );
    }
}
