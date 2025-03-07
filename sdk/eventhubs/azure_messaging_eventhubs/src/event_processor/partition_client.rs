// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use crate::{
    models::{ConsumerClientDetails, ReceivedEventData},
    EventReceiver,
};
use async_stream::try_stream;
use azure_core::Result;
use azure_core_amqp::AmqpMessage;
use futures::Stream;
use std::sync::Arc;

use super::processor::CheckpointStore;

pub struct PartitionClient {}

impl PartitionClient {
    pub fn new<F>(
        partition_id: &str,
        checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
        client_details: &ConsumerClientDetails,
        on_destroy: F,
    ) -> Self
    where
        //        F: FnOnce(String, &mut HashMap<String, EventReceiver>) -> Self,
        F: FnOnce(),
    {
        todo!()
    }
    pub fn get_partition_id(&self) -> String {
        todo!()
    }
    pub fn receive_events(&self) -> impl Stream<Item = azure_core::Result<ReceivedEventData>> + '_ {
        try_stream! {
            // Replace `todo!()` with the actual implementation
            yield ReceivedEventData::from(AmqpMessage::builder().build());
            todo!();
        }
    }

    pub async fn close(self) {}

    pub async fn update_checkpoint(&self, event_data: ReceivedEventData) -> Result<()> {
        todo!()
    }

    pub(crate) fn set_event_receiver(&self, event_receiver: EventReceiver) {
        // Set the event receiver
        todo!()
    }
}
