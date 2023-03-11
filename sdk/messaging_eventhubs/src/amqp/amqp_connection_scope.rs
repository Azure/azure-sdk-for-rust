use std::time::Duration;

use url::Url;

use crate::event_hubs_transport_type::EventHubsTransportType;

use super::amqp_connection::AmqpConnection;

pub(crate) struct AmqpConnectionScope {
        /// The recommended timeout to associate with an AMQP session.  It is recommended that this
        /// interval be used when creating or opening AMQP links and related constructs.
        pub(crate) session_timeout: Duration,

        /// The amount of time to allow a connection to have no observed traffic before considering it idle.
        pub(crate) connection_idle_timeout: Duration,

        /// Indicates whether this <see cref="AmqpConnectionScope"/> has been disposed.
        pub(crate) is_disposed: bool,

        // /// <summary>
        // ///   The cancellation token to use with operations initiated by the scope.
        // /// </summary>
        // ///
        // private CancellationTokenSource OperationCancellationSource { get; } = new CancellationTokenSource();

        // /// <summary>
        // ///   The set of active AMQP links associated with the connection scope.  These are considered children
        // ///   of the active connection and should be managed as such.
        // /// </summary>
        // ///
        // private ConcurrentDictionary<AmqpObject, Timer> ActiveLinks { get; } = new ConcurrentDictionary<AmqpObject, Timer>();

        /// The unique identifier of the scope.
        pub(crate) id: String,

        /// The endpoint for the Event Hubs service to which the scope is associated.
        pub(crate) service_endpoint: Url,

        /// The endpoint to used establishing a connection to the Event Hubs service to which the scope is associated.
        pub(crate) connection_endpoint: Url,

        /// The name of the Event Hub to which the scope is associated.
        pub(crate) event_hub_name: String,

        // ///   The provider to use for obtaining a token for authorization with the Event Hubs service.
        // private CbsTokenProvider TokenProvider { get; }

        /// The type of transport to use for communication.
        pub(crate) transport: EventHubsTransportType,

        // /// <summary>
        // ///   The proxy, if any, which should be used for communication.
        // /// </summary>
        // ///
        // private IWebProxy Proxy { get; }

        /// The size of the buffer used for sending information via the active transport.
        pub(crate) send_buffer_size_in_bytes: usize,

        /// The size of the buffer used for receiving information via the active transport.
        pub(crate) receive_buffer_size_in_bytes: usize,

        // /// <summary>
        // ///   A <see cref="RemoteCertificateValidationCallback" /> delegate allowing custom logic to be considered for
        // ///   validation of the remote certificate responsible for encrypting communication.
        // /// </summary>
        // ///
        // private RemoteCertificateValidationCallback CertificateValidationCallback { get; }

        /// <summary>
        ///   The AMQP connection that is active for the current scope.
        /// </summary>
        ///
        // private FaultTolerantAmqpObject<AmqpConnection> ActiveConnection { get; }\
        pub(crate) active_connection: AmqpConnection,

}
