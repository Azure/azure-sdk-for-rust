use crate::{
    amqp::amqp_sender::AmqpSender, core::TransportClient,
    primitives::service_bus_connection::ServiceBusConnection, ServiceBusSenderOptions,
};

use super::error::ServiceBusSenderError;

pub struct ServiceBusSender {
    pub(crate) inner: AmqpSender,
    pub(crate) entity_path: String,
    pub(crate) identifier: String,
}

impl ServiceBusSender {
    /// <summary>
    ///   Initializes a new instance of the <see cref="ServiceBusSender"/> class.
    /// </summary>
    /// <param name="entityPath">The entity path to send the message to.</param>
    /// <param name="connection">The connection for the sender.</param>
    /// <param name="options">The set of options to use when configuring the sender.</param>
    ///
    ///
    // internal ServiceBusSender(
    //     string entityPath,
    //     ServiceBusConnection connection,
    //     ServiceBusSenderOptions options = default)
    // {
    //     Logger.ClientCreateStart(typeof(ServiceBusSender), connection?.FullyQualifiedNamespace, entityPath);
    //     try
    //     {
    //         Argument.AssertNotNull(connection, nameof(connection));
    //         Argument.AssertNotNull(connection.RetryOptions, nameof(connection.RetryOptions));
    //         Argument.AssertNotNullOrWhiteSpace(entityPath, nameof(entityPath));
    //         connection.ThrowIfClosed();

    //         options = options?.Clone() ?? new ServiceBusSenderOptions();

    //         EntityPath = entityPath;
    //         Identifier = string.IsNullOrEmpty(options.Identifier) ? DiagnosticUtilities.GenerateIdentifier(EntityPath) : options.Identifier;
    //         _connection = connection;
    //         _retryPolicy = _connection.RetryOptions.ToRetryPolicy();
    //         _innerSender = _connection.CreateTransportSender(
    //             entityPath,
    //             _retryPolicy,
    //             Identifier);
    //         _scopeFactory = new EntityScopeFactory(EntityPath, FullyQualifiedNamespace);
    //     }
    //     catch (Exception ex)
    //     {
    //         Logger.ClientCreateException(typeof(ServiceBusSender), connection?.FullyQualifiedNamespace, entityPath, ex);
    //         throw;
    //     }
    //     Logger.ClientCreateComplete(typeof(ServiceBusSender), Identifier);
    // }
    pub(crate) async fn new<C>(
        entity_path: &str,
        connection: &ServiceBusConnection<C>,
        options: Option<ServiceBusSenderOptions>,
    ) -> Result<Self, ServiceBusSenderError>
    where
        C: TransportClient,
    {
        todo!()
    }
}
