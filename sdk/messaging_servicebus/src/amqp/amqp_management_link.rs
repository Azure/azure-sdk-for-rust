use fe2o3_amqp::link::DetachError;
use fe2o3_amqp_management::MgmtClient;
use tokio::sync::mpsc;

use super::amqp_cbs_link;

#[derive(Debug)]
pub(crate) struct AmqpManagementLink {
    identifier: u32,
    client: MgmtClient,
    cbs_command_sender: mpsc::Sender<amqp_cbs_link::Command>,
}

impl AmqpManagementLink {
    pub(crate) fn new(
        identifier: u32,
        client: MgmtClient,
        cbs_command_sender: mpsc::Sender<amqp_cbs_link::Command>,
    ) -> Self {
        Self {
            identifier,
            client,
            cbs_command_sender,
        }
    }

    pub(crate) fn client_mut(&mut self) -> &mut MgmtClient {
        &mut self.client
    }

    pub(crate) async fn close(self) -> Result<(), DetachError> {
        let _ = self
            .cbs_command_sender
            .send(amqp_cbs_link::Command::RemoveAuthorizationRefresher(
                self.identifier,
            ))
            .await;
        self.client.close().await
    }
}
