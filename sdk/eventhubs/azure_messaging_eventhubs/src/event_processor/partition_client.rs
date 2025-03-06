// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use crate::models::ReceivedEventData;
use async_stream::try_stream;
use azure_core::Result;
use azure_core_amqp::AmqpMessage;
use futures::Stream;

pub struct PartitionClient {}

impl PartitionClient {
    fn new() -> Self {
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
}
