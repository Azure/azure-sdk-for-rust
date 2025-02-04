// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp servicebus eventhub mgmt

use std::sync::OnceLock;

use crate::messaging::{
    AmqpDelivery, AmqpDeliveryApis, AmqpMessage, AmqpOutcome, DeliveryNumber, DeliveryTag,
    DistributionMode, TerminusDurability, TerminusExpiryPolicy,
};

pub(crate) struct Fe2o3AmqpDelivery {
    pub(crate) delivery: fe2o3_amqp::link::delivery::Delivery<
        fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
    >,
    message: OnceLock<AmqpMessage>,
    delivery_tag: OnceLock<DeliveryTag>,
}

impl
    From<
        fe2o3_amqp::link::delivery::Delivery<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        >,
    > for AmqpDelivery
{
    fn from(
        delivery: fe2o3_amqp::link::delivery::Delivery<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        >,
    ) -> Self {
        AmqpDelivery::new(Fe2o3AmqpDelivery {
            delivery,
            message: OnceLock::new(),
            delivery_tag: OnceLock::new(),
        })
    }
}

impl AmqpDeliveryApis for Fe2o3AmqpDelivery {
    // Return a reference to the message contained in the delivery.
    fn message(&self) -> &AmqpMessage {
        self.message
            .get_or_init(|| AmqpMessage::from(self.delivery.message().clone()))
    }

    fn delivery_id(&self) -> DeliveryNumber {
        *self.delivery.delivery_id()
    }

    fn delivery_tag(&self) -> &DeliveryTag {
        self.delivery_tag
            .get_or_init(|| self.delivery.delivery_tag().to_vec())
    }

    fn message_format(&self) -> &Option<u32> {
        self.delivery.message_format()
    }

    fn into_message(self) -> crate::messaging::AmqpMessage {
        self.delivery.into_message().into()
    }
}

impl From<fe2o3_amqp_types::messaging::Outcome> for AmqpOutcome {
    fn from(outcome: fe2o3_amqp_types::messaging::Outcome) -> Self {
        match outcome {
            fe2o3_amqp_types::messaging::Outcome::Accepted(_) => AmqpOutcome::Accepted,
            fe2o3_amqp_types::messaging::Outcome::Released(_) => AmqpOutcome::Released,
            fe2o3_amqp_types::messaging::Outcome::Rejected(_) => AmqpOutcome::Rejected,
            fe2o3_amqp_types::messaging::Outcome::Modified(_) => AmqpOutcome::Modified,
        }
    }
}

impl From<AmqpOutcome> for fe2o3_amqp_types::messaging::Outcome {
    fn from(outcome: AmqpOutcome) -> Self {
        match outcome {
            AmqpOutcome::Accepted => fe2o3_amqp_types::messaging::Outcome::Accepted(
                fe2o3_amqp_types::messaging::Accepted {},
            ),
            AmqpOutcome::Released => fe2o3_amqp_types::messaging::Outcome::Released(
                fe2o3_amqp_types::messaging::Released {},
            ),
            AmqpOutcome::Rejected => fe2o3_amqp_types::messaging::Outcome::Rejected(
                fe2o3_amqp_types::messaging::Rejected { error: None },
            ),
            AmqpOutcome::Modified => fe2o3_amqp_types::messaging::Outcome::Modified(
                fe2o3_amqp_types::messaging::Modified {
                    delivery_failed: None,
                    undeliverable_here: None,
                    message_annotations: None,
                },
            ),
        }
    }
}

#[test]
fn test_outcome_round_trip() {
    let outcomes = vec![
        AmqpOutcome::Accepted,
        AmqpOutcome::Released,
        AmqpOutcome::Rejected,
        AmqpOutcome::Modified,
    ];

    for outcome in outcomes {
        let fe2o3_outcome: fe2o3_amqp_types::messaging::Outcome = outcome.clone().into();
        let amqp_outcome: AmqpOutcome = fe2o3_outcome.into();
        assert_eq!(outcome, amqp_outcome);
    }
}

impl From<fe2o3_amqp_types::messaging::TerminusDurability>
    for crate::messaging::TerminusDurability
{
    fn from(durability: fe2o3_amqp_types::messaging::TerminusDurability) -> Self {
        match durability {
            fe2o3_amqp_types::messaging::TerminusDurability::None => TerminusDurability::None,
            fe2o3_amqp_types::messaging::TerminusDurability::Configuration => {
                TerminusDurability::Configuration
            }
            fe2o3_amqp_types::messaging::TerminusDurability::UnsettledState => {
                TerminusDurability::UnsettledState
            }
        }
    }
}

impl From<crate::messaging::TerminusDurability>
    for fe2o3_amqp_types::messaging::TerminusDurability
{
    fn from(durability: crate::messaging::TerminusDurability) -> Self {
        match durability {
            TerminusDurability::None => fe2o3_amqp_types::messaging::TerminusDurability::None,
            TerminusDurability::Configuration => {
                fe2o3_amqp_types::messaging::TerminusDurability::Configuration
            }
            TerminusDurability::UnsettledState => {
                fe2o3_amqp_types::messaging::TerminusDurability::UnsettledState
            }
        }
    }
}

#[test]
fn test_terminus_durability_round_trip() {
    let durabilities = vec![
        TerminusDurability::None,
        TerminusDurability::Configuration,
        TerminusDurability::UnsettledState,
    ];

    for durability in durabilities {
        let fe2o3_durability =
            fe2o3_amqp_types::messaging::TerminusDurability::from(durability.clone());
        let amqp_durability = TerminusDurability::from(fe2o3_durability);
        assert_eq!(durability, amqp_durability);
    }
}

impl From<fe2o3_amqp_types::messaging::TerminusExpiryPolicy> for TerminusExpiryPolicy {
    fn from(expiry_policy: fe2o3_amqp_types::messaging::TerminusExpiryPolicy) -> Self {
        match expiry_policy {
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::LinkDetach => {
                TerminusExpiryPolicy::LinkDetach
            }
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::SessionEnd => {
                TerminusExpiryPolicy::SessionEnd
            }
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::ConnectionClose => {
                TerminusExpiryPolicy::ConnectionClose
            }
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::Never => TerminusExpiryPolicy::Never,
        }
    }
}

impl From<TerminusExpiryPolicy> for fe2o3_amqp_types::messaging::TerminusExpiryPolicy {
    fn from(expiry_policy: TerminusExpiryPolicy) -> Self {
        match expiry_policy {
            TerminusExpiryPolicy::LinkDetach => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::LinkDetach
            }
            TerminusExpiryPolicy::SessionEnd => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::SessionEnd
            }
            TerminusExpiryPolicy::ConnectionClose => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::ConnectionClose
            }
            TerminusExpiryPolicy::Never => fe2o3_amqp_types::messaging::TerminusExpiryPolicy::Never,
        }
    }
}

#[test]
fn test_terminus_expiry_policy_round_trip() {
    let expiry_policies = vec![
        TerminusExpiryPolicy::LinkDetach,
        TerminusExpiryPolicy::SessionEnd,
        TerminusExpiryPolicy::ConnectionClose,
        TerminusExpiryPolicy::Never,
    ];

    for expiry_policy in expiry_policies {
        let fe2o3_expiry_policy =
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::from(expiry_policy.clone());
        let amqp_expiry_policy = TerminusExpiryPolicy::from(fe2o3_expiry_policy);
        assert_eq!(expiry_policy, amqp_expiry_policy);
    }
}

impl From<fe2o3_amqp_types::messaging::DistributionMode> for DistributionMode {
    fn from(distribution_mode: fe2o3_amqp_types::messaging::DistributionMode) -> Self {
        match distribution_mode {
            fe2o3_amqp_types::messaging::DistributionMode::Move => DistributionMode::Move,
            fe2o3_amqp_types::messaging::DistributionMode::Copy => DistributionMode::Copy,
        }
    }
}

impl From<crate::messaging::DistributionMode> for fe2o3_amqp_types::messaging::DistributionMode {
    fn from(distribution_mode: crate::messaging::DistributionMode) -> Self {
        match distribution_mode {
            DistributionMode::Move => fe2o3_amqp_types::messaging::DistributionMode::Move,
            DistributionMode::Copy => fe2o3_amqp_types::messaging::DistributionMode::Copy,
        }
    }
}

#[test]
fn test_distribution_mode_round_trip() {
    let distribution_modes = vec![DistributionMode::Move, DistributionMode::Copy];

    for distribution_mode in distribution_modes {
        let fe2o3_distribution_mode =
            fe2o3_amqp_types::messaging::DistributionMode::from(distribution_mode.clone());
        let amqp_distribution_mode = DistributionMode::from(fe2o3_distribution_mode);
        assert_eq!(distribution_mode, amqp_distribution_mode);
    }
}
