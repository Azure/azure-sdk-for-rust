use crate::core::TransportClient;

#[derive(Debug)]
pub(crate) enum InnerClient {}

// impl TransportClient for InnerClient {
//     type Error = ();

//     type Sender;

//     type Receiver;

//     type RuleManager;

//     fn is_closed(&self) -> bool {
//         todo!()
//     }

//     fn service_endpoint(&self) -> &str {
//         todo!()
//     }

//     fn create_sender(
//         &mut self,
//         entity_path: impl Into<String>,
//         retry_policy: impl super::service_bus_retry_policy::ServiceBusRetryPolicy,
//         identifier: impl Into<String>,
//     ) -> Result<Self::Sender, Self::Error> {
//         todo!()
//     }

//     fn create_receiver(
//         &mut self,
//         entity_path: impl Into<String>,
//         retry_policy: impl super::service_bus_retry_policy::ServiceBusRetryPolicy,
//         receive_mode: crate::receiver::service_bus_receive_mode::ServiceBusReceiveMode,
//         prefetch_count: u32,
//         identifier: impl Into<String>,
//         session_id: impl Into<String>,
//         is_session_receiver: bool,
//         is_processor: bool,
//     ) -> Result<Self::Receiver, Self::Error> {
//         todo!()
//     }

//     fn create_rule_manager(
//         &mut self,
//         subscription_path: impl Into<String>,
//         retry_policy: impl super::service_bus_retry_policy::ServiceBusRetryPolicy,
//         identifier: impl Into<String>,
//     ) -> Result<Self::RuleManager, Self::Error> {
//         todo!()
//     }

//     fn close<'life0, 'async_trait>(
//         &'life0 mut self,
//         cancellation_token: impl 'async_trait + Into<Option<tokio_util::sync::CancellationToken>>,
//     ) -> core::pin::Pin<
//         Box<
//             dyn core::future::Future<Output = Result<(), Self::Error>>
//                 + core::marker::Send
//                 + 'async_trait,
//         >,
//     >
//     where
//         'life0: 'async_trait,
//         Self: 'async_trait,
//     {
//         todo!()
//     }
// }
