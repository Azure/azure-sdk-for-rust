use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Connection string cannot be empty")]
    ConnectionStringIsEmpty,

    #[error("Connection string is malformed")]
    InvalidConnectionString,
}

/// The set of properties that comprise a Service Bus connection string.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ServiceBusConnectionStringProperties<'a> {
    endpoint: Cow<'a, str>,
    entity_path: Cow<'a, str>,
    shared_access_key_name: Cow<'a, str>,
    shared_access_key: Cow<'a, str>,
    shared_access_signature: Cow<'a, str>,
}

impl<'a> ServiceBusConnectionStringProperties<'a> {
    /// The character used to separate a token and its value in the connection string.
    const TOKEN_VALUE_SEPARATOR: char = '=';

    /// The character used to mark the beginning of a new token/value pair in the connection string.
    const TOKEN_VALUE_PAIR_DELIMITER: char = ';';

    /// The name of the protocol used by an Service Bus endpoint.
    const SERVICE_BUS_ENDPOINT_SCHEME_NAME: &'static str = "sb";

    /// The token that identifies the endpoint address for the Service Bus namespace.
    const ENDPOINT_TOKEN: &'static str = "Endpoint";

    /// The token that identifies the name of a specific Service Bus entity under the namespace.
    const ENTITY_PATH_TOKEN: &'static str = "EntityPath";

    /// The token that identifies the name of a shared access key.
    const SHARED_ACCESS_KEY_NAME_TOKEN: &'static str = "SharedAccessKeyName";

    /// The token that identifies the value of a shared access key.
    const SHARED_ACCESS_KEY_VALUE_TOKEN: &'static str = "SharedAccessKey";

    /// The token that identifies the value of a shared access signature.
    const SHARED_ACCESS_SIGNATURE_TOKEN: &'static str = "SharedAccessSignature";

    /// The formatted protocol used by an Service Bus endpoint.
    const SERVICE_BUS_ENDPOINT_SCHEME: &'static str = "sb://";

    /// <summary>
    ///   Creates an Service Bus connection string based on this set of <see cref="ServiceBusConnectionStringProperties" />.
    /// </summary>
    ///
    /// <returns>
    ///   A valid Service Bus connection string; depending on the specified property information, this may
    ///   represent the namespace-level or Event Hub-level.
    /// </returns>
    ///
    ///
    pub fn to_connection_string(&self) -> String {
        todo!()
    }

    /// <summary>
    ///   Parses the specified Service Bus connection string into its component properties.
    /// </summary>
    ///
    /// <param name="connectionString">The connection string to parse.</param>
    ///
    /// <returns>The component properties parsed from the connection string.</returns>
    ///
    /// <exception cref="FormatException">The specified connection string was malformed and could not be parsed.</exception>
    ///
    pub fn parse(connection_string: impl Into<Cow<'a, str>>) -> Result<Self, ParseError> {
        todo!()
    }
}
