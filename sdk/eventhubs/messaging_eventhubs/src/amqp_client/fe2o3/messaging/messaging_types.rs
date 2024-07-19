// Copyright (c) Microsoft Corp. All Rights Reserved.

// cspell: words amqp servicebus eventhub mgmt

use crate::amqp_client::messaging::AmqpOutcome;

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
    for crate::amqp_client::messaging::TerminusDurability
{
    fn from(durability: fe2o3_amqp_types::messaging::TerminusDurability) -> Self {
        match durability {
            fe2o3_amqp_types::messaging::TerminusDurability::None => {
                crate::amqp_client::messaging::TerminusDurability::None
            }
            fe2o3_amqp_types::messaging::TerminusDurability::Configuration => {
                crate::amqp_client::messaging::TerminusDurability::Configuration
            }
            fe2o3_amqp_types::messaging::TerminusDurability::UnsettledState => {
                crate::amqp_client::messaging::TerminusDurability::UnsettledState
            }
        }
    }
}

impl From<crate::amqp_client::messaging::TerminusDurability>
    for fe2o3_amqp_types::messaging::TerminusDurability
{
    fn from(durability: crate::amqp_client::messaging::TerminusDurability) -> Self {
        match durability {
            crate::amqp_client::messaging::TerminusDurability::None => {
                fe2o3_amqp_types::messaging::TerminusDurability::None
            }
            crate::amqp_client::messaging::TerminusDurability::Configuration => {
                fe2o3_amqp_types::messaging::TerminusDurability::Configuration
            }
            crate::amqp_client::messaging::TerminusDurability::UnsettledState => {
                fe2o3_amqp_types::messaging::TerminusDurability::UnsettledState
            }
        }
    }
}

impl From<fe2o3_amqp_types::messaging::TerminusExpiryPolicy>
    for crate::amqp_client::messaging::TerminusExpiryPolicy
{
    fn from(expiry_policy: fe2o3_amqp_types::messaging::TerminusExpiryPolicy) -> Self {
        match expiry_policy {
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::LinkDetach => {
                crate::amqp_client::messaging::TerminusExpiryPolicy::LinkDetach
            }
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::SessionEnd => {
                crate::amqp_client::messaging::TerminusExpiryPolicy::SessionEnd
            }
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::ConnectionClose => {
                crate::amqp_client::messaging::TerminusExpiryPolicy::ConnectionClose
            }
            fe2o3_amqp_types::messaging::TerminusExpiryPolicy::Never => {
                crate::amqp_client::messaging::TerminusExpiryPolicy::Never
            }
        }
    }
}

impl From<crate::amqp_client::messaging::TerminusExpiryPolicy>
    for fe2o3_amqp_types::messaging::TerminusExpiryPolicy
{
    fn from(expiry_policy: crate::amqp_client::messaging::TerminusExpiryPolicy) -> Self {
        match expiry_policy {
            crate::amqp_client::messaging::TerminusExpiryPolicy::LinkDetach => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::LinkDetach
            }
            crate::amqp_client::messaging::TerminusExpiryPolicy::SessionEnd => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::SessionEnd
            }
            crate::amqp_client::messaging::TerminusExpiryPolicy::ConnectionClose => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::ConnectionClose
            }
            crate::amqp_client::messaging::TerminusExpiryPolicy::Never => {
                fe2o3_amqp_types::messaging::TerminusExpiryPolicy::Never
            }
        }
    }
}

impl From<fe2o3_amqp_types::messaging::DistributionMode>
    for crate::amqp_client::messaging::DistributionMode
{
    fn from(distribution_mode: fe2o3_amqp_types::messaging::DistributionMode) -> Self {
        match distribution_mode {
            fe2o3_amqp_types::messaging::DistributionMode::Move => {
                crate::amqp_client::messaging::DistributionMode::Move
            }
            fe2o3_amqp_types::messaging::DistributionMode::Copy => {
                crate::amqp_client::messaging::DistributionMode::Copy
            }
        }
    }
}

impl From<crate::amqp_client::messaging::DistributionMode>
    for fe2o3_amqp_types::messaging::DistributionMode
{
    fn from(distribution_mode: crate::amqp_client::messaging::DistributionMode) -> Self {
        match distribution_mode {
            crate::amqp_client::messaging::DistributionMode::Move => {
                fe2o3_amqp_types::messaging::DistributionMode::Move
            }
            crate::amqp_client::messaging::DistributionMode::Copy => {
                fe2o3_amqp_types::messaging::DistributionMode::Copy
            }
        }
    }
}
