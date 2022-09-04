pub(crate) struct AmqpConnectionScope {}

impl AmqpConnectionScope {
    /// The name to assign to the SASL handler to specify that CBS tokens are in use.
    const CBS_SASL_HANDLER_NAME: &'static str = "MSSBCBS";

    /// <summary>The suffix to attach to the resource path when using web sockets for service communication.</summary>
    const WEB_SOCKETS_PATH_SUFFIX: &'static str = "/$servicebus/websocket/";

    /// <summary>The URI scheme to apply when using web sockets for service communication.</summary>
    const WEB_SOCKETS_URI_SCHEME: &'static str = "wss";
}
