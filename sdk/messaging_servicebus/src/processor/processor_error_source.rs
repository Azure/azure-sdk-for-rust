/// <summary>The source of the error when <see cref="ProcessErrorEventArgs"/> is raised.</summary>
pub enum ServiceBusErrorSource {
    /// <summary>Message completion operation.</summary>
    Complete,

    /// <summary>Message abandon operation.</summary>
    Abandon,

    /// <summary>Process message handler invocation.</summary>
    ProcessMessageCallback,

    /// <summary>Message receive operation.</summary>
    Receive,

    /// <summary>Lock renewal operation.</summary>
    RenewLock,

    /// <summary>Session start operation.</summary>
    AcceptSession,

    /// <summary>Session close operation.</summary>
    CloseSession,
}
