use time::OffsetDateTime;

/// A set of metrics that can be used to monitor communication between the client and service.
#[derive(Debug, Default, Clone)]
pub(crate) struct ServiceBusTransportMetrics {
    /// <summary>
    /// Gets the last time that a heartbeat was received from the Service Bus service. These heartbeats are sent from the
    /// service approximately every 30 seconds.
    /// </summary>
    pub(crate) last_heart_beat: Option<OffsetDateTime>,

    /// <summary>
    /// Gets the last time that a connection was opened for the associated <see cref="ServiceBusClient"/> instance.
    /// </summary>
    pub(crate) last_connection_open: Option<OffsetDateTime>,

    /// <summary>
    /// Gets the last time that a connection was closed for the associated <see cref="ServiceBusClient"/> instance. If the <see cref="ServiceBusClient"/>
    /// was disposed, then this time will not be updated again. It may be updated multiple times if the close is initiated by the service.
    /// </summary>
    pub(crate) last_connection_close: Option<OffsetDateTime>,
}

impl ServiceBusTransportMetrics {
    pub fn new() -> Self {
        Default::default()
    }
}
