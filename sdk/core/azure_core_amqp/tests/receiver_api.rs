// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_amqp::{
    error::Result, AmqpDelivery, AmqpDeliveryOutcome, AmqpOrderedMap, AmqpReceiverApis,
    AmqpReceiverOptions, AmqpSession, AmqpSource, AmqpSymbol, AmqpValue, ReceiverCreditMode,
};

#[test]
fn exported_delivery_outcome_has_all_variants_and_fields() {
    let accepted = AmqpDeliveryOutcome::Accepted;
    assert!(matches!(accepted, AmqpDeliveryOutcome::Accepted));

    let rejected = AmqpDeliveryOutcome::Rejected(None);
    assert!(matches!(rejected, AmqpDeliveryOutcome::Rejected(None)));

    let released = AmqpDeliveryOutcome::Released;
    assert!(matches!(released, AmqpDeliveryOutcome::Released));

    let mut annotations = AmqpOrderedMap::new();
    annotations.insert(
        AmqpSymbol::from("x-opt-sequence-number"),
        AmqpValue::from(7),
    );
    let modified = AmqpDeliveryOutcome::Modified {
        delivery_failed: Some(true),
        undeliverable_here: Some(false),
        message_annotations: Some(annotations.clone()),
    };

    match modified {
        AmqpDeliveryOutcome::Modified {
            delivery_failed,
            undeliverable_here,
            message_annotations,
        } => {
            assert_eq!(delivery_failed, Some(true));
            assert_eq!(undeliverable_here, Some(false));
            assert_eq!(message_annotations, Some(annotations));
        }
        _ => panic!("expected Modified delivery outcome"),
    }
}

pub async fn settle_delivery_with_public_signature(
    receiver: &(impl AmqpReceiverApis + Sync),
    delivery: &AmqpDelivery,
    outcome: AmqpDeliveryOutcome,
) -> Result<()> {
    receiver.settle_delivery(delivery, outcome).await
}

struct LegacyReceiver;

#[async_trait::async_trait]
impl AmqpReceiverApis for LegacyReceiver {
    async fn attach(
        &self,
        _session: &AmqpSession,
        _source: impl Into<AmqpSource> + Send,
        _options: Option<AmqpReceiverOptions>,
    ) -> Result<()> {
        panic!("not called")
    }

    async fn detach(self) -> Result<()> {
        panic!("not called")
    }

    async fn set_credit_mode(&self, _credit_mode: ReceiverCreditMode) -> Result<()> {
        panic!("not called")
    }

    async fn credit_mode(&self) -> Result<ReceiverCreditMode> {
        panic!("not called")
    }

    async fn receive_delivery(&self) -> Result<AmqpDelivery> {
        panic!("not called")
    }

    async fn accept_delivery(&self, _delivery: &AmqpDelivery) -> Result<()> {
        panic!("not called")
    }

    async fn reject_delivery(&self, _delivery: &AmqpDelivery) -> Result<()> {
        panic!("not called")
    }

    async fn release_delivery(&self, _delivery: &AmqpDelivery) -> Result<()> {
        panic!("not called")
    }
}

#[test]
fn legacy_receiver_without_settle_delivery_still_implements_the_trait() {
    fn assert_receiver_apis<T: AmqpReceiverApis>() {}

    assert_receiver_apis::<LegacyReceiver>();
}
