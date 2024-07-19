// Copyright (c) Microsoft Corp. All Rights Reserved.

// cspell: words amqp servicebus eventhub mgmt

use crate::messaging::AmqpOutcome;

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

impl From<fe2o3_amqp_types::messaging::TerminusDurability>
    for crate::messaging::TerminusDurability
{
    fn from(durability: fe2o3_amqp_types::messaging::TerminusDurability) -> Self {
        match durability {
            fe2o3_amqp_types::messaging::TerminusDurability::None => {
                crate::messaging::TerminusDurability::None
            }
            fe2o3_amqp_types::messaging::TerminusDurability::Configuration => {
                crate::messaging::TerminusDurability::Configuration
            }
            fe2o3_amqp_types::messaging::TerminusDurability::UnsettledState => {
                crate::messaging::TerminusDurability::UnsettledState
            }
        }
    }
}

impl From<crate::messaging::TerminusDurability>
    for fe2o3_amqp_types::messaging::TerminusDurability
{
    fn from(durability: crate::messaging::TerminusDurability) -> Self {
        match durability {
            crate::messaging::TerminusDurability::None => {
                fe2o3_amqp_types::messaging::TerminusDurability::None
            }
            crate::messaging::TerminusDurability::Configuration => {
                fe2o3_amqp_types::messaging::TerminusDurability::Configuration
            }
            crate::messaging::TerminusDurability::UnsettledState => {
                fe2o3_amqp_types::messaging::TerminusDurability::UnsettledState
            }
        }
    }
}

impl From<fe2o3_amqp_types::messaging::TerminusExpiryPolicy>
    for crate::messaging::TerminusExpiryPolicy
{
    fn from(expiry_policy: fe2o3_amqp_types::messaging::TerminusExpiryPolicy) -> Self {
        match expiry_policy {
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::LinkDetach => {
                crate::messaging::TerminusExpiryPolicy::LinkDetach
            }
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::SessionEnd => {
                crate::messaging::TerminusExpiryPolicy::SessionEnd
            }
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::ConnectionClose => {
                crate::messaging::TerminusExpiryPolicy::ConnectionClose
            }
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::Never => {
                crate::messaging::TerminusExpiryPolicy::Never
            }
        }
    }
}

impl From<crate::messaging::TerminusExpiryPolicy>
    for fe2o3_amqp_types::messaging::TerminusExpiryPolicy
{
    fn from(expiry_policy: crate::messaging::TerminusExpiryPolicy) -> Self {
        match expiry_policy {
            crate::messaging::TerminusExpiryPolicy::LinkDetach => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::LinkDetach
            }
            crate::messaging::TerminusExpiryPolicy::SessionEnd => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::SessionEnd
            }
            crate::messaging::TerminusExpiryPolicy::ConnectionClose => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::ConnectionClose
            }
            crate::messaging::TerminusExpiryPolicy::Never => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::Never
            }
        }
    }
}

impl From<fe2o3_amqp_types::messaging::DistributionMode> for crate::messaging::DistributionMode {
    fn from(distribution_mode: fe2o3_amqp_types::messaging::DistributionMode) -> Self {
        match distribution_mode {
            fe2o3_amqp_types::messaging::DistributionMode::Move => {
                crate::messaging::DistributionMode::Move
            }
            fe2o3_amqp_types::messaging::DistributionMode::Copy => {
                crate::messaging::DistributionMode::Copy
            }
        }
    }
}

impl From<crate::messaging::DistributionMode> for fe2o3_amqp_types::messaging::DistributionMode {
    fn from(distribution_mode: crate::messaging::DistributionMode) -> Self {
        match distribution_mode {
            crate::messaging::DistributionMode::Move => {
                fe2o3_amqp_types::messaging::DistributionMode::Move
            }
            crate::messaging::DistributionMode::Copy => {
                fe2o3_amqp_types::messaging::DistributionMode::Copy
            }
        }
    }
}
