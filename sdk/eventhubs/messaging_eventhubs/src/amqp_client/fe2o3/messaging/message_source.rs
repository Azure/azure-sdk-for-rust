// Copyright (c) Microsoft Corp. All Rights Reserved.

// cspell: words amqp servicebus eventhub mgmt

use fe2o3_amqp_types::definitions::Fields;

use crate::amqp_client::{
    messaging::AmqpSource,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};

impl From<AmqpSource> for fe2o3_amqp_types::messaging::Source {
    fn from(source: AmqpSource) -> Self {
        let mut builder = fe2o3_amqp_types::messaging::Source::builder();

        if let Some(address) = source.address {
            builder = builder.address(address);
        }
        if let Some(durable) = source.durable {
            builder = builder.durable(durable.into());
        }
        if let Some(expiry_policy) = source.expiry_policy {
            builder = builder.expiry_policy(expiry_policy.into());
        }
        if let Some(timeout) = source.timeout {
            builder = builder.timeout(timeout.into());
        }
        if let Some(dynamic) = source.dynamic {
            builder = builder.dynamic(dynamic);
        }
        if let Some(dynamic_node_properties) = source.dynamic_node_properties {
            let fields: Fields = Fields::new();
            let fields = dynamic_node_properties
                .into_iter()
                .fold(fields, |mut fields, (k, v)| {
                    fields.insert(k.into(), v.into());
                    fields
                });

            builder = builder.dynamic_node_properties(fields);
        }
        if let Some(distribution_mode) = source.distribution_mode {
            builder = builder.distribution_mode(distribution_mode.into());
        }
        if let Some(filter) = source.filter {
            builder = builder.filter(
                filter
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect(),
            );
        }
        if let Some(default_outcome) = source.default_outcome {
            builder = builder.default_outcome(default_outcome.into());
        }
        if let Some(outcomes) = source.outcomes {
            let outcomes: fe2o3_amqp_types::primitives::Array<
                fe2o3_amqp_types::primitives::Symbol,
            > = outcomes.into_iter().map(|o| o.into()).collect();
            builder = builder.outcomes(outcomes);
        }
        if let Some(capabilities) = source.capabilities {
            let capabilities: fe2o3_amqp_types::primitives::Array<
                fe2o3_amqp_types::primitives::Symbol,
            > = capabilities.into_iter().map(|c| c.into()).collect();
            builder = builder.capabilities(capabilities);
        }
        builder.build()
    }
}

impl From<fe2o3_amqp_types::messaging::Source> for AmqpSource {
    fn from(source: fe2o3_amqp_types::messaging::Source) -> Self {
        let mut amqp_source_builder = AmqpSource::builder();

        if let Some(address) = source.address {
            amqp_source_builder = amqp_source_builder.with_address(address);
        }
        amqp_source_builder = amqp_source_builder
            .with_durable(source.durable.into())
            .with_expiry_policy(source.expiry_policy.into())
            .with_timeout(source.timeout.into())
            .with_dynamic(source.dynamic);

        if let Some(dynamic_node_properties) = source.dynamic_node_properties {
            let dynamic_node_properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> =
                dynamic_node_properties
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect();
            amqp_source_builder =
                amqp_source_builder.with_dynamic_node_properties(dynamic_node_properties);
        }
        if let Some(distribution_mode) = source.distribution_mode {
            amqp_source_builder =
                amqp_source_builder.with_distribution_mode(distribution_mode.into());
        }
        if let Some(filter) = source.filter {
            let filter: AmqpOrderedMap<AmqpSymbol, AmqpValue> = filter
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect();
            amqp_source_builder = amqp_source_builder.with_filter(filter);
        }
        if let Some(default_outcome) = source.default_outcome {
            amqp_source_builder = amqp_source_builder.with_default_outcome(default_outcome.into());
        }
        if let Some(outcomes) = source.outcomes {
            amqp_source_builder =
                amqp_source_builder.with_outcomes(outcomes.into_iter().map(|o| o.into()).collect());
        }
        if let Some(capabilities) = source.capabilities {
            amqp_source_builder = amqp_source_builder
                .with_capabilities(capabilities.into_iter().map(|c| c.into()).collect());
        }
        amqp_source_builder.build()
    }
}
