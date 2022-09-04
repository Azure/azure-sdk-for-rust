use azure_core::Url;

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
    pub(crate) endpoint: url::Url,
    pub(crate) entity_path: Option<&'a str>,
    pub(crate) shared_access_key_name: Option<&'a str>,
    pub(crate) shared_access_key: Option<&'a str>,
    pub(crate) shared_access_signature: Option<&'a str>,
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

    /// The fully qualified Service Bus namespace that the consumer is associated with.  This is
    /// likely to be similar to `{yournamespace}.servicebus.windows.net`.
    ///
    /// # Value
    ///
    /// The namespace of the Service Bus, as derived from the endpoint address of the connection
    /// string.
    pub fn fully_qualified_namespace(&self) -> Option<&str> {
        self.endpoint.host_str()
    }

    /// The endpoint to be used for connecting to the Service Bus namespace.
    ///
    /// # Value
    ///
    /// The endpoint address, including protocol, from the connection string.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// The name of the specific Service Bus entity instance under the associated Service Bus
    /// namespace.
    pub fn entity_path(&self) -> Option<&str> {
        self.entity_path
    }

    /// The name of the shared access key, either for the Service Bus namespace or the Service Bus
    /// entity.
    pub fn shared_access_key_name(&self) -> Option<&str> {
        self.shared_access_key_name
    }

    /// The value of the shared access key, either for the Service Bus namespace or the Service Bus
    /// entity.
    pub fn shared_access_key(&self) -> Option<&str> {
        self.shared_access_key
    }

    /// The value of the fully-formed shared access signature, either for the Service Bus namespace
    /// or the Service Bus entity.
    pub fn shared_access_signature(&self) -> Option<&str> {
        self.shared_access_signature
    }

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
        let mut s = String::new();

        s.push_str(Self::ENDPOINT_TOKEN);
        s.push(Self::TOKEN_VALUE_SEPARATOR);
        s.push_str(self.endpoint.as_str());
        s.push(Self::TOKEN_VALUE_PAIR_DELIMITER);

        if let Some(entity_path) = self.entity_path.and_then(|s| match !s.is_empty() {
            true => Some(s),
            false => None,
        }) {
            s.push_str(Self::ENTITY_PATH_TOKEN);
            s.push(Self::TOKEN_VALUE_SEPARATOR);
            s.push_str(entity_path);
            s.push(Self::TOKEN_VALUE_PAIR_DELIMITER);
        }

        if let Some(shared_access_signature) =
            self.shared_access_signature
                .and_then(|s| match !s.is_empty() {
                    true => Some(s),
                    false => None,
                })
        {
            s.push_str(Self::SHARED_ACCESS_SIGNATURE_TOKEN);
            s.push(Self::TOKEN_VALUE_SEPARATOR);
            s.push_str(shared_access_signature);
            s.push(Self::TOKEN_VALUE_PAIR_DELIMITER);
        } else {
            match (self.shared_access_key_name, self.shared_access_key) {
                (Some(key_name), Some(key)) if !key_name.is_empty() && !key.is_empty() => {
                    s.push_str(Self::SHARED_ACCESS_KEY_NAME_TOKEN);
                    s.push(Self::TOKEN_VALUE_SEPARATOR);
                    s.push_str(key_name);
                    s.push(Self::TOKEN_VALUE_PAIR_DELIMITER);
                    s.push_str(Self::SHARED_ACCESS_KEY_VALUE_TOKEN);
                    s.push(Self::TOKEN_VALUE_SEPARATOR);
                    s.push_str(key);
                    s.push(Self::TOKEN_VALUE_PAIR_DELIMITER);
                }
                _ => {}
            }
        }

        s
    }

    /// Parses the specified Service Bus connection string into its component properties.
    ///
    /// # Arguments
    ///
    /// * `connection_string` - The connection string to parse.
    ///
    /// # Returns
    ///
    /// ## `Ok`
    ///
    /// The component properties parsed from the connection string.
    ///
    /// ## `Err`
    ///
    /// Returns `Err(_)` if the specified connection string was malformed and could not be parsed.
    pub fn parse(connection_string: &'a str) -> Result<Self, ParseError> {
        if connection_string.is_empty() {
            return Err(ParseError::ConnectionStringIsEmpty);
        }

        let mut endpoint: Option<Url> = None;
        let mut entity_path: Option<&'a str> = None;
        let mut shared_access_key_name: Option<&'a str> = None;
        let mut shared_access_key: Option<&'a str> = None;
        let mut shared_access_signature: Option<&'a str> = None;

        let token_value_pairs = connection_string.split(Self::TOKEN_VALUE_PAIR_DELIMITER);

        for token_value_pair in token_value_pairs {
            let mut split = token_value_pair.split(Self::TOKEN_VALUE_SEPARATOR);
            let token = split
                .next()
                .ok_or(ParseError::InvalidConnectionString)?
                .trim();
            let value = split
                .next()
                .ok_or(ParseError::InvalidConnectionString)?
                .trim();

            // If there was no value for a key, then consider the connection string to
            // be malformed.
            if token.is_empty() || value.is_empty() {
                return Err(ParseError::InvalidConnectionString);
            }

            // Compare the token against the known connection string properties and capture the
            // pair if they are a known attribute.
            match token {
                Self::ENDPOINT_TOKEN => {
                    // TODO: What about the port?
                    let url = Url::parse(value).map_err(|_| ParseError::InvalidConnectionString)?;
                    if url.scheme() != Self::SERVICE_BUS_ENDPOINT_SCHEME_NAME {
                        return Err(ParseError::InvalidConnectionString);
                    }
                    endpoint = Some(url);
                }
                Self::ENTITY_PATH_TOKEN => entity_path = Some(value),
                Self::SHARED_ACCESS_KEY_NAME_TOKEN => shared_access_key_name = Some(value),
                Self::SHARED_ACCESS_KEY_VALUE_TOKEN => shared_access_key = Some(value),
                Self::SHARED_ACCESS_SIGNATURE_TOKEN => shared_access_signature = Some(value),
                _ => {}
            }
        }

        Ok(Self {
            endpoint: endpoint.ok_or(ParseError::InvalidConnectionString)?,
            entity_path,
            shared_access_key_name,
            shared_access_key,
            shared_access_signature,
        })
    }
}

#[cfg(test)]
mod tests {
    use azure_core::Url;

    #[test]
    fn test_parsing_endpoint_as_url() {
        let connection_endpoint = "sb://fe2o3-amqp-example.servicebus.windows.net/";
        let url = Url::parse(&connection_endpoint).unwrap();
        println!("{:?}", url.host_str());
    }
}
