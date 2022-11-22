// #[derive(Debug, thiserror::Error)]
// pub enum ServiceBusRecvError {
//     #[error("Lock token cannot be converted from AMQP message")]
//     InvalidLockTokenError,

//     #[error(transparent)]
//     Recv(#[from] RecvError),

//     #[error(transparent)]
//     LinkState(#[from] IllegalLinkStateError),
// }

// impl From<InvalidLockTokenError> for ServiceBusRecvError {
//     fn from(_: InvalidLockTokenError) -> Self {
//         Self::InvalidLockTokenError
//     }
// }
