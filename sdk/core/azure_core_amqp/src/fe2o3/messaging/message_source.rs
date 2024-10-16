// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp servicebus eventhub mgmt

use fe2o3_amqp_types::definitions::Fields;

use crate::{
    messaging::AmqpSource,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};

#[cfg(test)]
use crate::messaging::{AmqpOutcome, DistributionMode, TerminusDurability, TerminusExpiryPolicy};

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
            builder = builder.timeout(timeout);
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
       amqp_source_builder= amqp_source_builder
            .with_durable(source.durable.into())
            .with_expiry_policy(source.expiry_policy.into())
            .with_timeout(source.timeout)
            .with_dynamic(source.dynamic);

        if let Some(dynamic_node_properties) = source.dynamic_node_properties {
            let dynamic_node_properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> =
                dynamic_node_properties
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect();

                amqp_source_builder = amqp_source_builder.with_dynamic_node_properties(dynamic_node_properties);
        }
        if let Some(distribution_mode) = source.distribution_mode {
            amqp_source_builder = amqp_source_builder.with_distribution_mode(distribution_mode.into());
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
            amqp_source_builder = amqp_source_builder.with_outcomes(outcomes.into_iter().map(|o| o.into()).collect());
        }
        if let Some(capabilities) = source.capabilities {
            amqp_source_builder = amqp_source_builder
                .with_capabilities(capabilities.into_iter().map(|c| c.into()).collect());
        }
        amqp_source_builder.build()
    }
}

#[test]
fn message_source_conversion_fe2o3_amqp() {
    let fe2o3_source = fe2o3_amqp_types::messaging::Source::builder()
        .address("test")
        .durable(fe2o3_amqp_types::messaging::TerminusDurability::UnsettledState)
        .expiry_policy(fe2o3_amqp_types::messaging::TerminusExpiryPolicy::SessionEnd)
        .timeout(95)
        .dynamic(false)
        .dynamic_node_properties(fe2o3_amqp_types::messaging::LifetimePolicy::DeleteOnClose(
            fe2o3_amqp_types::messaging::DeleteOnClose {},
        ))
        .add_lifetime_policy(fe2o3_amqp_types::messaging::DeleteOnClose {})
        .distribution_mode(fe2o3_amqp_types::messaging::DistributionMode::Move {})
        .add_to_filter(
            fe2o3_amqp_types::primitives::Symbol::from("filter_key"),
            Some(serde_amqp::described::Described {
                descriptor: serde_amqp::descriptor::Descriptor::Name(
                    fe2o3_amqp_types::primitives::Symbol::from("filter_descriptor"),
                ),
                value: fe2o3_amqp_types::primitives::Value::from("filter_value"),
            }),
        )
        .outcomes(vec![fe2o3_amqp_types::primitives::Symbol::from("outcome")])
        .default_outcome(fe2o3_amqp_types::messaging::Outcome::Accepted(
            fe2o3_amqp_types::messaging::Accepted {},
        ))
        .capabilities(vec![fe2o3_amqp_types::primitives::Symbol::from(
            "capability",
        )])
        .build();

    let amqp_source = AmqpSource::from(fe2o3_source.clone());
    let round_trip = fe2o3_amqp_types::messaging::Source::from(amqp_source);

    // fe2o3 source does not implement PartialEq
    assert_eq!(fe2o3_source.address, round_trip.address);
    assert_eq!(fe2o3_source.durable, round_trip.durable);
    assert_eq!(fe2o3_source.expiry_policy, round_trip.expiry_policy);
    assert_eq!(fe2o3_source.timeout, round_trip.timeout);
    assert_eq!(fe2o3_source.dynamic, round_trip.dynamic);
    assert_eq!(
        fe2o3_source.dynamic_node_properties,
        round_trip.dynamic_node_properties
    );
    assert_eq!(fe2o3_source.filter, round_trip.filter);
    assert_eq!(fe2o3_source.outcomes, round_trip.outcomes);
    assert_eq!(fe2o3_source.capabilities, round_trip.capabilities);
    assert_eq!(
        fe2o3_source.distribution_mode.is_some(),
        round_trip.distribution_mode.is_some()
    );

    // DistributionMode is an enum, so we need to match on it and it doesn't implement PartialEq
    let original_distribution_mode = fe2o3_source.distribution_mode.unwrap();
    let round_trip_distribution_mode = round_trip.distribution_mode.unwrap();
    match original_distribution_mode {
        fe2o3_amqp_types::messaging::DistributionMode::Move => {
            assert!(matches!(
                round_trip_distribution_mode,
                fe2o3_amqp_types::messaging::DistributionMode::Move
            ));
        }
        fe2o3_amqp_types::messaging::DistributionMode::Copy => {
            assert!(matches!(
                round_trip_distribution_mode,
                fe2o3_amqp_types::messaging::DistributionMode::Copy
            ));
        }
    }

    assert_eq!(
        fe2o3_source.default_outcome.is_some(),
        round_trip.default_outcome.is_some()
    );

    if fe2o3_source.default_outcome.is_some() {
        let original_default_outcome = fe2o3_source.default_outcome.unwrap();
        let round_trip_default_outcome = round_trip.default_outcome.unwrap();

        assert_eq!(
            original_default_outcome.is_accepted(),
            round_trip_default_outcome.is_accepted(),
        );
        assert_eq!(
            original_default_outcome.is_rejected(),
            round_trip_default_outcome.is_rejected(),
        );
        assert_eq!(
            original_default_outcome.is_released(),
            round_trip_default_outcome.is_released(),
        );
        assert_eq!(
            original_default_outcome.is_modified(),
            round_trip_default_outcome.is_modified(),
        );
    }
}

#[test]
fn message_source_conversion_amqp_fe2o3() {
    let amqp_source = AmqpSource::builder()
        .with_address("test")
        .with_durable(TerminusDurability::UnsettledState)
        .with_expiry_policy(TerminusExpiryPolicy::SessionEnd)
        .with_timeout(95)
        .with_dynamic(true)
        .with_dynamic_node_properties(AmqpOrderedMap::new())
        .with_distribution_mode(DistributionMode::Move)
        .with_filter(AmqpOrderedMap::new())
        .with_default_outcome(AmqpOutcome::Accepted)
        .with_outcomes(vec![])
        .with_capabilities(vec![])
        .build();

    let fe2o3_source = fe2o3_amqp_types::messaging::Source::from(amqp_source.clone());

    let round_trip = AmqpSource::from(fe2o3_source);

    assert_eq!(amqp_source, round_trip);
}
