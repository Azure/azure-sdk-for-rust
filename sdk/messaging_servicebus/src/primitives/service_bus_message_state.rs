/// <summary>Represents the message state of the [`ServiceBusReceivedMessage`]
pub enum ServiceBusMessageState {
    /// Specifies an active message state.
    Active = 0,

    /// Specifies a deferred message state.
    Deferred = 1,

    /// Specifies the scheduled message state.
    Scheduled = 2,
}
