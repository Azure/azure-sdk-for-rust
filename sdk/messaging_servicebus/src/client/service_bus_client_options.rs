use azure_core::Url;

use crate::primitives::{
    service_bus_retry_options::ServiceBusRetryOptions,
    service_bus_transport_type::ServiceBusTransportType,
};

/// The set of options that can be specified when creating an [`ServiceBusConnection`]
/// to configure its behavior.
#[derive(Debug, Clone, Default)]
pub struct ServiceBusClientOptions {
    /// The type of protocol and transport that will be used for communicating with the Service
    /// Bus service.
    pub transport_type: ServiceBusTransportType,

    /// A property used to set the [`ServiceBusClient`] ID to identify the client. This can be used
    /// to correlate logs and exceptions. If `None` or empty, a random unique value will be
    /// used.
    pub identifier: Option<String>,

    /// A custom endpoint address that can be used when establishing the connection to the Service
    /// Bus service.
    ///
    /// # Remarks
    ///
    /// The custom endpoint address will be used in place of the default endpoint provided by the
    /// Service Bus namespace when establishing the connection. The connection string or fully
    /// qualified namespace will still be needed in order to validate the connection with the
    /// service.
    pub custom_endpoint_address: Option<Url>,

    /// The set of options to use for determining whether a failed operation should be retried and,
    /// if so, the amount of time to wait between retry attempts.  These options also control the
    /// amount of time allowed for receiving messages and other interactions with the Service Bus
    /// service.
    pub retry_options: ServiceBusRetryOptions,

    /// Gets or sets a flag that indicates whether or not transactions may span multiple
    /// Service Bus entities.
    ///
    /// # Value
    ///
    /// `true`, when cross-entity transactions are enabled; `false` when transactions are not being
    /// used or should be limited to a single entity.
    pub enable_cross_entity_transactions: bool,

    /// Gets or sets whether or not to enable metrics for the associated [`ServiceBusClient`]
    /// instance. If set to `true`, [ServiceBusClient::get_transport_metrics`] can be called.
    pub enable_transport_metrics: bool,
}
