use azure_core::Url;

#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    #[error("Connection string cannot be empty")]
    ConnectionStringIsEmpty,

    #[error("Connection string is malformed")]
    InvalidConnectionString,
}

/// The set of properties that comprise a Service Bus connection string.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ServiceBusConnectionStringProperties<'a> {
    pub(crate) endpoint: Option<url::Url>,
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
        self.endpoint.as_ref().and_then(|url| url.host_str())
    }

    /// The endpoint to be used for connecting to the Service Bus namespace.
    ///
    /// # Value
    ///
    /// The endpoint address, including protocol, from the connection string.
    pub fn endpoint(&self) -> Option<&Url> {
        self.endpoint.as_ref()
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

    /// Creates an Service Bus connection string based on this set of
    /// [`ServiceBusConnectionStringProperties`].
    ///
    /// # Returns
    ///
    /// ## `Some(String)`
    ///
    /// A valid Service Bus connection string; depending on the specified property information, this
    /// may represent the namespace-level or Event Hub-level.
    ///
    /// ## `None`
    ///
    /// If field `endpoint` is `None`
    ///
    pub fn to_connection_string(&self) -> Option<String> {
        let mut s = String::new();

        if let Some(endpoint) = self.endpoint() {
            s.push_str(Self::ENDPOINT_TOKEN);
            s.push(Self::TOKEN_VALUE_SEPARATOR);
            s.push_str(endpoint.as_str());
            s.push(Self::TOKEN_VALUE_PAIR_DELIMITER);
        } else {
            return None;
        }

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

        Some(s)
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
    pub fn parse(connection_string: &'a str) -> Result<Self, FormatError> {
        if connection_string.is_empty() {
            return Err(FormatError::ConnectionStringIsEmpty);
        }

        let mut endpoint: Option<Url> = None;
        let mut entity_path: Option<&'a str> = None;
        let mut shared_access_key_name: Option<&'a str> = None;
        let mut shared_access_key: Option<&'a str> = None;
        let mut shared_access_signature: Option<&'a str> = None;

        let token_value_pairs = connection_string.split(Self::TOKEN_VALUE_PAIR_DELIMITER);

        for token_value_pair in token_value_pairs {
            let mut split = token_value_pair.split(Self::TOKEN_VALUE_SEPARATOR);
            let token = match split.next().and_then(|s| match s.trim() {
                "" => None,
                s => Some(s),
            }) {
                Some(token) => token,
                None => continue,
            };

            let value = split
                .next()
                .and_then(|s| match s.trim() {
                    "" => None,
                    s => Some(s),
                })
                // If there was no value for a key, then consider the connection string to
                // be malformed.
                .ok_or(FormatError::InvalidConnectionString)?;

            // Compare the token against the known connection string properties and capture the
            // pair if they are a known attribute.
            match token {
                Self::ENDPOINT_TOKEN => {
                    // TODO: What about the port?
                    let url =
                        Url::parse(value).map_err(|_| FormatError::InvalidConnectionString)?;
                    if url.scheme() != Self::SERVICE_BUS_ENDPOINT_SCHEME_NAME {
                        return Err(FormatError::InvalidConnectionString);
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
            endpoint,
            entity_path,
            shared_access_key_name,
            shared_access_key,
            shared_access_signature,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use azure_core::Url;

    use super::ServiceBusConnectionStringProperties;

    const ENDPOINT: &str = "test.endpoint.com";
    const EVENT_HUB: &str = "some-path";
    const SAS_KEY_NAME: &str = "sasName";
    const SAS_KEY: &str = "sasKey";
    const SAS: &str = "fullsas";

    ///
    struct Expected {
        endpoint: Option<&'static str>,
        event_hub: Option<&'static str>,
        sas_key_name: Option<&'static str>,
        sas_key: Option<&'static str>,
        sas: Option<&'static str>,
    }

    macro_rules! assert_parsed_and_expected {
        ($connection_string:ident, $expected:ident) => {
            let parsed = ServiceBusConnectionStringProperties::parse(&$connection_string).unwrap();

            assert_eq!(
                parsed.endpoint().and_then(|url| url.host_str()),
                $expected.endpoint
            );
            assert_eq!(parsed.shared_access_key_name(), $expected.sas_key_name);
            assert_eq!(parsed.shared_access_key(), $expected.sas_key);
            assert_eq!(parsed.shared_access_signature(), $expected.sas);
            assert_eq!(parsed.entity_path(), $expected.event_hub);
        };
    }

    /// Provides the reordered token test cases for the <see
    /// cref="ServiceBusConnectionStringProperties.Parse" /> tests.
    fn random_ordering_connection_string_cases() -> Vec<(String, Expected)> {
        vec![
            (
                format!(
                    "Endpoint=sb://{};SharedAccessKeyName={};SharedAccessKey={};EntityPath={}",
                    ENDPOINT, SAS_KEY_NAME, SAS_KEY, EVENT_HUB
                ),
                Expected {
                    endpoint: Some(ENDPOINT),
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: Some(SAS_KEY),
                    sas: None,
                },
            ),
            (
                format!(
                    "Endpoint=sb://{};SharedAccessKey={};EntityPath={};SharedAccessKeyName={}",
                    ENDPOINT, SAS_KEY, EVENT_HUB, SAS_KEY_NAME,
                ),
                Expected {
                    endpoint: Some(ENDPOINT),
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: Some(SAS_KEY),
                    sas: None,
                },
            ),
            (
                format!(
                    "Endpoint=sb://{};EntityPath={};SharedAccessKeyName={};SharedAccessKey={}",
                    ENDPOINT, EVENT_HUB, SAS_KEY_NAME, SAS_KEY
                ),
                Expected {
                    endpoint: Some(ENDPOINT),
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: Some(SAS_KEY),
                    sas: None,
                },
            ),
            (
                format!(
                    "SharedAccessKeyName={};SharedAccessKey={};Endpoint=sb://{};EntityPath={}",
                    SAS_KEY_NAME, SAS_KEY, ENDPOINT, EVENT_HUB
                ),
                Expected {
                    endpoint: Some(ENDPOINT),
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: Some(SAS_KEY),
                    sas: None,
                },
            ),
            (
                format!(
                    "EntityPath={};SharedAccessKey={};SharedAccessKeyName={};Endpoint=sb://{}",
                    EVENT_HUB, SAS_KEY, SAS_KEY_NAME, ENDPOINT
                ),
                Expected {
                    endpoint: Some(ENDPOINT),
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: Some(SAS_KEY),
                    sas: None,
                },
            ),
            (
                format!(
                    "EntityPath={};SharedAccessSignature={};Endpoint=sb://{}",
                    EVENT_HUB, SAS, ENDPOINT,
                ),
                Expected {
                    endpoint: Some(ENDPOINT),
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: None,
                    sas_key: None,
                    sas: Some(SAS),
                },
            ),
            (
                format!(
                    "SharedAccessKeyName={};SharedAccessKey={};Endpoint=sb://{};EntityPath={};SharedAccessSignature={}",
                    SAS_KEY_NAME, SAS_KEY, ENDPOINT, EVENT_HUB, SAS
                ),
                Expected {
                    endpoint: Some(ENDPOINT),
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: Some(SAS_KEY),
                    sas: Some(SAS),
                },
            ),
        ]
    }

    /// <summary>
    ///   Provides the reordered token test cases for the <see cref="ServiceBusConnectionStringProperties.Parse" /> tests.
    /// </summary>
    ///
    fn partial_connection_string_cases() -> Vec<(String, Expected)> {
        vec![
            (
                format!("Endpoint=sb://{}", ENDPOINT),
                Expected {
                    endpoint: Some(ENDPOINT),
                    event_hub: None,
                    sas_key_name: None,
                    sas_key: None,
                    sas: None,
                },
            ),
            (
                format!("SharedAccessKey={}", SAS_KEY),
                Expected {
                    endpoint: None,
                    event_hub: None,
                    sas_key_name: None,
                    sas_key: Some(SAS_KEY),
                    sas: None,
                },
            ),
            (
                format!(
                    "EntityPath={};SharedAccessKeyName={}",
                    EVENT_HUB, SAS_KEY_NAME
                ),
                Expected {
                    endpoint: None,
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: None,
                    sas: None,
                },
            ),
            (
                format!(
                    "SharedAccessKeyName={};SharedAccessKey={}",
                    SAS_KEY_NAME, SAS_KEY
                ),
                Expected {
                    endpoint: None,
                    event_hub: None,
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: Some(SAS_KEY),
                    sas: None,
                },
            ),
            (
                format!(
                    "EntityPath={};SharedAccessKey={};SharedAccessKeyName={}",
                    EVENT_HUB, SAS_KEY, SAS_KEY_NAME
                ),
                Expected {
                    endpoint: None,
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: Some(SAS_KEY),
                    sas: None,
                },
            ),
            (
                format!(
                    "SharedAccessKeyName={};SharedAccessSignature={}",
                    SAS_KEY_NAME, SAS
                ),
                Expected {
                    endpoint: None,
                    event_hub: None,
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: None,
                    sas: Some(SAS),
                },
            ),
            (
                format!("EntityPath={};SharedAccessSignature={}", EVENT_HUB, SAS),
                Expected {
                    endpoint: None,
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: None,
                    sas_key: None,
                    sas: Some(SAS),
                },
            ),
            (
                format!(
                "EntityPath={};SharedAccessKey={};SharedAccessKeyName={};SharedAccessSignature={}",
                EVENT_HUB, SAS_KEY, SAS_KEY_NAME, SAS
            ),
                Expected {
                    endpoint: None,
                    event_hub: Some(EVENT_HUB),
                    sas_key_name: Some(SAS_KEY_NAME),
                    sas_key: Some(SAS_KEY),
                    sas: Some(SAS),
                },
            ),
        ]
    }

    /// Verifies functionality of the [`ServiceBusConnectionStringProperties::parse`]
    /// method.
    #[test]
    fn parse_correctly_parses_a_namespace_connection_string() {
        let endpoint = "test.endpoint.com";
        let sas_key = "sasKey";
        let sas_key_name = "sasName";
        let shared_access_signature = "fakeSAS";
        let connection_string = format!("Endpoint=sb://{endpoint};SharedAccessKeyName={sas_key_name};SharedAccessKey={sas_key};SharedAccessSignature={shared_access_signature}");
        let parsed = ServiceBusConnectionStringProperties::parse(&connection_string).unwrap();

        assert_eq!(
            parsed.endpoint().and_then(|url| url.host_str()),
            Some(endpoint)
        );
        assert_eq!(parsed.shared_access_key_name(), Some(sas_key_name));
        assert_eq!(parsed.shared_access_key(), Some(sas_key));
        assert_eq!(
            parsed.shared_access_signature(),
            Some(shared_access_signature)
        );
        assert_eq!(parsed.entity_path(), None);
    }

    /// <summary>
    ///   Verifies functionality of the <see cref="ServiceBusConnectionStringProperties.Parse" />
    ///   method.
    /// </summary>
    ///
    #[test]
    fn parse_correctly_parses_an_entity_connection_string() {
        let endpoint = "test.endpoint.com";
        let event_hub = "some-path";
        let sas_key = "sasKey";
        let sas_key_name = "sasName";
        let shared_access_signature = "fakeSAS";
        let connection_string = format!("Endpoint=sb://{endpoint};SharedAccessKeyName={sas_key_name};SharedAccessKey={sas_key};EntityPath={event_hub};SharedAccessSignature={shared_access_signature}");
        let parsed = ServiceBusConnectionStringProperties::parse(&connection_string).unwrap();

        assert_eq!(
            parsed.endpoint().and_then(|url| url.host_str()),
            Some(endpoint)
        );
        assert_eq!(parsed.shared_access_key_name(), Some(sas_key_name));
        assert_eq!(parsed.shared_access_key(), Some(sas_key));
        assert_eq!(
            parsed.shared_access_signature(),
            Some(shared_access_signature)
        );
        assert_eq!(parsed.entity_path(), Some(event_hub));
    }

    #[test]
    fn parse_correctly_parses_partial_connection_strings() {
        let cases = partial_connection_string_cases();

        for (connection_string, expected) in cases {
            assert_parsed_and_expected!(connection_string, expected);
        }
    }

    /// <summary>
    ///   Verifies functionality of the <see cref="ServiceBusConnectionStringProperties.Parse" />
    ///   method.
    /// </summary>
    ///
    #[test]
    fn parse_tolerates_leading_delimiters() {
        let endpoint = "test.endpoint.com";
        let event_hub = "some-path";
        let sas_key = "sasKey";
        let sas_key_name = "sasName";
        let connection_string = format!(";Endpoint=sb://{endpoint};SharedAccessKeyName={sas_key_name};SharedAccessKey={sas_key};EntityPath={event_hub}");
        let parsed = ServiceBusConnectionStringProperties::parse(&connection_string).unwrap();

        assert_eq!(
            parsed.endpoint().and_then(|url| url.host_str()),
            Some(endpoint)
        );
        assert_eq!(parsed.shared_access_key_name(), Some(sas_key_name));
        assert_eq!(parsed.shared_access_key(), Some(sas_key));
        assert_eq!(parsed.entity_path(), Some(event_hub));
    }

    /// <summary>
    ///   Verifies functionality of the <see cref="ServiceBusConnectionStringProperties.Parse" />
    ///   method.
    /// </summary>
    ///
    #[test]
    fn parse_tolerates_spaces_between_pairs() {
        let endpoint = "test.endpoint.com";
        let event_hub = "some-path";
        let sas_key = "sasKey";
        let sas_key_name = "sasName";
        let connection_string = format!("Endpoint=sb://{endpoint}; SharedAccessKeyName={sas_key_name}; SharedAccessKey={sas_key}; EntityPath={event_hub}");
        let parsed = ServiceBusConnectionStringProperties::parse(&connection_string).unwrap();

        assert_eq!(
            parsed.endpoint().and_then(|url| url.host_str()),
            Some(endpoint)
        );
        assert_eq!(parsed.shared_access_key_name(), Some(sas_key_name));
        assert_eq!(parsed.shared_access_key(), Some(sas_key));
        assert_eq!(parsed.entity_path(), Some(event_hub));
    }

    /// <summary>
    ///   Verifies functionality of the <see cref="ServiceBusConnectionStringProperties.Parse" />
    ///   method.
    /// </summary>
    ///
    #[test]
    fn parse_tolerates_spaces_between_values() {
        let endpoint = "test.endpoint.com";
        let event_hub = "some-path";
        let sas_key = "sasKey";
        let sas_key_name = "sasName";
        let connection_string = format!("Endpoint = sb://{endpoint};SharedAccessKeyName ={sas_key_name};SharedAccessKey= {sas_key}; EntityPath  =  {event_hub}");
        let parsed = ServiceBusConnectionStringProperties::parse(&connection_string).unwrap();

        assert_eq!(
            parsed.endpoint().and_then(|url| url.host_str()),
            Some(endpoint)
        );
        assert_eq!(parsed.shared_access_key_name(), Some(sas_key_name));
        assert_eq!(parsed.shared_access_key(), Some(sas_key));
        assert_eq!(parsed.entity_path(), Some(event_hub));
    }

    /// <summary>
    ///   Verifies functionality of the <see cref="ServiceBusConnectionStringProperties.Parse" />
    ///   method.
    /// </summary>
    ///
    #[test]
    fn parse_does_not_force_token_ordering() {
        let cases = random_ordering_connection_string_cases();

        for (connection_string, expected) in cases {
            assert_parsed_and_expected!(connection_string, expected);
        }
    }

    /// <summary>
    ///   Verifies functionality of the <see cref="ServiceBusConnectionStringProperties.Parse" />
    ///   method.
    /// </summary>
    ///
    #[test]
    fn parse_ignores_unknown_tokens() {
        let endpoint = "test.endpoint.com";
        let event_hub = "some-path";
        let sas_key = "sasKey";
        let sas_key_name = "sasName";
        let connection_string = format!("Endpoint=sb://{endpoint};SharedAccessKeyName={sas_key_name};Unknown=INVALID;SharedAccessKey={sas_key};EntityPath={event_hub};Trailing=WHOAREYOU");
        let parsed = ServiceBusConnectionStringProperties::parse(&connection_string).unwrap();

        assert_eq!(
            parsed.endpoint().and_then(|url| url.host_str()),
            Some(endpoint)
        );
        assert_eq!(parsed.shared_access_key_name(), Some(sas_key_name));
        assert_eq!(parsed.shared_access_key(), Some(sas_key));
        assert_eq!(parsed.entity_path(), Some(event_hub));
    }

    // Stopped at `ParseDoesAcceptsHostNamesAndUrisForTheEndpoint` because not all dotnet tests are
    // applicable
}
