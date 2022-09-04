/// The set of well-known reasons for an Service Bus operation failure that was the cause of an
/// exception.
#[derive(Debug)]
pub enum ServiceBusFailureReason {
    /// The exception was the result of a general error within the client library.
    GeneralError,

    /// A Service Bus resource cannot be found by the Service Bus service.
    MessagingEntityNotFound,

    /// The lock on the message is lost. Callers should call attempt to receive and process the
    /// message again.
    MessageLockLost,

    /// The requested message was not found.
    MessageNotFound,

    /// A message is larger than the maximum size allowed for its transport.
    MessageSizeExceeded,

    /// The Messaging Entity is disabled. Enable the entity again using Portal.
    MessagingEntityDisabled,

    /// The quota applied to an Service Bus resource has been exceeded while interacting with the
    /// Azure Service Bus service.
    QuotaExceeded,

    /// The Azure Service Bus service reports that it is busy in response to a client request to
    /// perform an operation.
    ServiceBusy,

    /// An operation or other request timed out while interacting with the Azure Service Bus
    /// service.
    ServiceTimeout,

    /// There was a general communications error encountered when interacting with the Azure Service
    /// Bus service.
    ServiceCommunicationProblem,

    /// The requested session cannot be locked.
    SessionCannotBeLocked,

    /// The lock on the session has expired. Callers should request the session again.
    SessionLockLost,

    /// An entity with the same name exists under the same namespace.
    MessagingEntityAlreadyExists,
}
