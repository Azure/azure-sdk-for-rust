use std::{marker::PhantomData, time::Duration};

use async_trait::async_trait;
use azure_core::Url;

use crate::{
    authorization::service_bus_token_credential::ServiceBusTokenCredential,
    constants::DEFAULT_LAST_PEEKED_SEQUENCE_NUMBER,
    core::{TransportClient, TransportConnectionScope},
    primitives::{
        service_bus_retry_options::ServiceBusRetryOptions,
        service_bus_retry_policy::ServiceBusRetryPolicy,
        service_bus_transport_type::ServiceBusTransportType,
    },
    receiver::service_bus_receive_mode::ServiceBusReceiveMode,
};

use super::{
    amqp_connection_scope::{AmqpConnectionScope},
    amqp_receiver::AmqpReceiver,
    amqp_sender::AmqpSender,
    amqp_session_receiver::AmqpSessionReceiver,
    error::{OpenReceiverError, OpenSenderError, AmqpClientError},
};

// TODO: current implementation doesn't support running callback in the background to refresh the
// token
// const DEFAULT_CREDENTIAL_REFRESH_BUFFER: Duration = Duration::from_secs(5 * 60);

/// A transport client abstraction responsible for brokering operations for AMQP-based connections.
///
/// See also [`TransportClient`]
#[derive(Debug)]
pub struct AmqpClient<RP> {
    /// The AMQP connection scope responsible for managing transport constructs for this instance.
    connection_scope: AmqpConnectionScope,

    /// Retry policy phantom
    retry_policy: PhantomData<RP>,
}

impl<RP> AmqpClient<RP>
where
    RP: ServiceBusRetryPolicy,
{
    // TODO: changing retry policy
    //
    // pub(crate) fn set_retry_policy<RP2>(self) -> AmqpClient<RP2> {
    //     AmqpClient {
    //         closed: self.closed,
    //         connection_scope: self.connection_scope,
    //         retry_policy: PhantomData,
    //     }
    // }
}

#[async_trait]
impl<RP> TransportClient for AmqpClient<RP>
where
    RP: ServiceBusRetryPolicy + Send + Sync,
{
    type CreateClientError = AmqpClientError;
    type CreateSenderError = OpenSenderError;
    type CreateReceiverError = OpenReceiverError;
    type DisposeError = AmqpClientError;

    type Sender = AmqpSender<RP>;
    type Receiver = AmqpReceiver<RP>;
    type SessionReceiver = AmqpSessionReceiver<RP>;

    // type CreateRuleManagerError = AmqpClientError;
    // type RuleManager = AmqpRuleManager;

    async fn create_transport_client(
        host: &str,
        credential: ServiceBusTokenCredential,
        transport_type: ServiceBusTransportType,
        custom_endpoint: Option<Url>,
        retry_timeout: Duration,
    ) -> Result<Self, Self::CreateClientError> {
        let service_endpoint = match transport_type {
            ServiceBusTransportType::AmqpTcp => {
                let addr = format!("{}://{}", transport_type.url_scheme(), host);
                Url::parse(&addr)?
            }
            ServiceBusTransportType::AmqpWebSockets => {
                let addr = format!(
                    "{}://{}{}",
                    transport_type.url_scheme(),
                    host,
                    AmqpConnectionScope::WEB_SOCKETS_PATH_SUFFIX
                );
                Url::parse(&addr)?
            }
        };

        let connection_endpoint = match custom_endpoint.as_ref().and_then(|url| url.host_str()) {
            Some(custom_host) => match transport_type {
                ServiceBusTransportType::AmqpTcp => {
                    let addr = format!("{}://{}", service_endpoint.scheme(), custom_host);
                    Url::parse(&addr)?
                }
                ServiceBusTransportType::AmqpWebSockets => {
                    let addr = format!(
                        "{}://{}{}",
                        service_endpoint.scheme(),
                        custom_host,
                        AmqpConnectionScope::WEB_SOCKETS_PATH_SUFFIX
                    );
                    Url::parse(&addr)?
                }
            },
            None => service_endpoint.clone(),
        };

        // Create AmqpConnectionScope
        let connection_scope = AmqpConnectionScope::new(
            service_endpoint,
            connection_endpoint,
            credential,
            transport_type,
            retry_timeout,
        )
        .await?;

        Ok(Self {
            connection_scope,
            retry_policy: PhantomData,
        })
    }

    fn transport_type(&self) -> ServiceBusTransportType {
        self.connection_scope.transport_type()
    }

    fn is_closed(&self) -> bool {
        self.connection_scope.is_disposed()
    }

    fn service_endpoint(&self) -> &Url {
        self.connection_scope.service_endpoint()
    }

    async fn create_sender(
        &mut self,
        entity_path: &str,
        identifier: &str,
        retry_options: ServiceBusRetryOptions,
    ) -> Result<Self::Sender, Self::CreateSenderError> {
        let (link_identifier, sender, cbs_command_sender) = self
            .connection_scope
            .open_sender_link(entity_path, identifier)
            .await?;
        let management_client = self
            .connection_scope
            .open_management_link(entity_path, identifier)
            .await?;
        let retry_policy = RP::new(retry_options);
        Ok(AmqpSender {
            identifier: link_identifier,
            retry_policy,
            sender,
            management_client,
            cbs_command_sender,
        })
    }

    async fn create_receiver(
        &mut self,
        entity_path: &str,
        identifier: &str,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        prefetch_count: u32,
        is_processor: bool,
    ) -> Result<Self::Receiver, Self::CreateReceiverError> {
        let (link_identifier, receiver, cbs_command_sender) = self
            .connection_scope
            .open_receiver_link(
                entity_path,
                identifier,
                &receive_mode,
                None,
                prefetch_count,
            )
            .await?;
        let management_client = self
            .connection_scope
            .open_management_link(&entity_path, &identifier)
            .await?;
        let retry_policy = RP::new(retry_options);
        Ok(AmqpReceiver {
            identifier: link_identifier,
            retry_policy,
            receiver,
            receive_mode,
            _is_processor: is_processor,
            prefetch_count,
            management_client,
            request_response_locked_messages: Default::default(),
            last_peeked_sequence_number: DEFAULT_LAST_PEEKED_SEQUENCE_NUMBER,
            cbs_command_sender,
        })
    }

    async fn create_session_receiver(
        &mut self,
        entity_path: &str,
        identifier: &str,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        session_id: String,
        prefetch_count: u32,
        is_processor: bool,
    ) -> Result<Self::SessionReceiver, Self::CreateReceiverError> {
        let (link_identifier, receiver, cbs_command_sender) = self
            .connection_scope
            .open_receiver_link(
                entity_path,
                identifier,
                &receive_mode,
                Some(session_id),
                prefetch_count,
            )
            .await?;
        let management_client = self
            .connection_scope
            .open_management_link(&entity_path, &identifier)
            .await?;
        let retry_policy = RP::new(retry_options);
        let inner = AmqpReceiver {
            identifier: link_identifier,
            retry_policy,
            receiver,
            receive_mode,
            _is_processor: is_processor,
            prefetch_count,
            management_client,
            request_response_locked_messages: Default::default(),
            last_peeked_sequence_number: DEFAULT_LAST_PEEKED_SEQUENCE_NUMBER,
            cbs_command_sender,
        };
        Ok(AmqpSessionReceiver { inner })
    }

    // TODO:
    // async fn create_rule_manager(
    //     &mut self,
    //     _subscription_path: String,
    //     _retry_options: ServiceBusRetryOptions,
    //     _identifier: String,
    // ) -> Result<Self::RuleManager, Self::CreateRuleManagerError> {
    //     todo!()
    // }

    async fn close(
        &mut self,
        // cancellation_token: Option<CancellationToken>,
    ) -> Result<(), Self::DisposeError> {
        if self.is_closed() {
            Ok(())
        } else {
            // match cancellation_token {
            //     Some(token) => {
            //         tokio::select! {
            //             _cancel = token.cancelled() => Err(Self::DisposeError::Cancelled),
            //             result = self.connection_scope.dispose() => {
            //                 result.map_err(Into::into)
            //             }
            //         }
            //     }
            //     None => self
            //         .connection_scope
            //         .dispose()
            //         .await
            //         .map_err(Into::into),
            // }
            self
                .connection_scope
                .dispose()
                .await
                .map_err(Into::into)
        }
    }
}

#[cfg(test)]
mod tests {}
