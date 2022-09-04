/// The source of the error when [`ProcessErrorEventArgs`] is raised.
#[derive(Debug, Clone)]
pub enum ServiceBusErrorSource {
    /// Message completion operation.
    Complete,

    /// Message abandon operation.
    Abandon,

    /// Process message handler invocation.
    ProcessMessageCallback,

    /// Message receive operation.
    Receive,

    /// Lock renewal operation.
    RenewLock,

    /// Session start operation.
    AcceptSession,

    /// Session close operation.
    CloseSession,
}
