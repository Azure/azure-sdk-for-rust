use crate::authorization::sas_token::options::constants::TABLE_ACCOUNT_SERVICES_IDENTIFIER;

#[derive(Clone, Debug, PartialEq)]
pub struct TableSasQueryParameters {
    /// Gets the storage service version to use to authenticate requests
    /// made with this shared access signature, and the service version to
    /// use when handling requests made with this shared access signature.
    pub version: String,

    /// The signature is an HMAC computed over the string-to-sign and key
    /// using the SHA256 algorithm, and then encoded using Base64 encoding.
    pub signature: String,

    pub expires_on: String,
    pub permissions: String,
    pub resource_types: String,

    pub ip: Option<String>,
    pub start_on: Option<String>,
    pub protocol: Option<String>,
}

impl TableSasQueryParameters {
    pub fn new(
        version: String,
        signature: String,
        expires_on: String,
        permissions: String,
        resource_types: String,
        ip: Option<String>,
        protocol: Option<String>,
        start_on: Option<String>,
    ) -> Self {
        Self {
            version,
            signature,
            expires_on,
            permissions,
            resource_types,
            ip,
            protocol,
            start_on,
        }
    }
}

/// sv=2019-02-02&
/// st=2019-04-29T22%3A18%3A26Z&
/// se=2019-04-30T02%3A23%3A26Z&
/// sr=b&
/// sp=rw&
/// sip=168.1.5.60-168.1.5.70&
/// spr=https&
/// sig=Z%2FRHIX5Xcg0Mq2rqI3OlWTjEg2tYkboXr1P9ZUXDtkk%3D
impl From<TableSasQueryParameters> for String {
    fn from(parameters: TableSasQueryParameters) -> Self {
        let mut sas_query_params = Vec::with_capacity(9);
        sas_query_params.push(("sv", parameters.version));
        sas_query_params.push(("ss", TABLE_ACCOUNT_SERVICES_IDENTIFIER.to_string()));
        if let Some(start_time) = parameters.start_on {
            sas_query_params.push(("st", start_time))
        }
        sas_query_params.push(("se", parameters.expires_on));
        sas_query_params.push(("sr", parameters.resource_types));
        sas_query_params.push(("sp", parameters.permissions));
        if let Some(ip) = parameters.ip {
            sas_query_params.push(("sip", ip))
        }
        if let Some(protocol) = parameters.protocol {
            sas_query_params.push(("spr", protocol))
        }
        sas_query_params.push(("sig", parameters.signature));

        url::Url::parse_with_params("https://example.com/products", &sas_query_params)
            .unwrap()
            .query()
            .map(|sas| sas.to_string())
            .unwrap()
    }
}
