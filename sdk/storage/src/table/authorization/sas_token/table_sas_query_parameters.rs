use azure_core::SasError;

use crate::authorization::sas_token::options::constants::TABLE_ACCOUNT_SERVICES_IDENTIFIER;

#[derive(Clone, Debug, PartialEq)]
pub struct TableSasQueryParameters {
    /// Gets the storage service version to use to authenticate requests
    /// made with this shared access signature, and the service version to
    /// use when handling requests made with this shared access signature.
    pub(crate) version: String,

    /// The signature is an HMAC computed over the string-to-sign and key
    /// using the SHA256 algorithm, and then encoded using Base64 encoding.
    pub(crate) signature: String,

    pub(crate) expires_on: String,
    pub(crate) permissions: String,
    pub(crate) resource_types: String,

    pub(crate) ip: Option<String>,
    pub(crate) start_on: Option<String>,
    pub(crate) protocol: Option<String>,
}

impl TableSasQueryParameters {
    pub(crate) fn new(
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

    pub fn token(&self) -> Result<String, SasError> {
        let mut sas_query_params = Vec::with_capacity(9);
        sas_query_params.push(("sv", self.version.as_str()));
        sas_query_params.push(("ss", TABLE_ACCOUNT_SERVICES_IDENTIFIER));
        sas_query_params.push(("srt", &self.resource_types));
        if let Some(ref start_time) = self.start_on {
            sas_query_params.push(("st", start_time.as_str()))
        }
        sas_query_params.push(("se", &self.expires_on.as_str()));
        sas_query_params.push(("sp", &self.permissions.as_str()));
        if let Some(ref ip) = self.ip {
            sas_query_params.push(("sip", ip.as_str()))
        }
        if let Some(ref protocol) = self.protocol {
            sas_query_params.push(("spr", protocol.as_str()))
        }
        sas_query_params.push(("sig", &self.signature.as_str()));

        serde_urlencoded::to_string(&sas_query_params).map_err(|_| {
            SasError::GeneralError("error creating sas token from the given parameters".to_string())
        })
    }

    /// Get a reference to the table sas query parameters's signature.
    pub fn signature(&self) -> &str {
        self.signature.as_ref()
    }

    /// Get a reference to the table sas query parameters's expires on.
    pub fn expires_on(&self) -> &str {
        self.expires_on.as_ref()
    }

    /// Get a reference to the table sas query parameters's protocol.
    pub fn protocol(&self) -> Option<&String> {
        self.protocol.as_ref()
    }

    /// Get a reference to the table sas query parameters's start on.
    pub fn start_on(&self) -> Option<&String> {
        self.start_on.as_ref()
    }

    /// Get a reference to the table sas query parameters's ip.
    pub fn ip(&self) -> Option<&String> {
        self.ip.as_ref()
    }

    /// Get a reference to the table sas query parameters's resource types.
    pub fn resource_types(&self) -> &str {
        self.resource_types.as_ref()
    }

    /// Get a reference to the table sas query parameters's permissions.
    pub fn permissions(&self) -> &str {
        self.permissions.as_ref()
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
        sas_query_params.push(("srt", parameters.resource_types));
        if let Some(start_time) = parameters.start_on {
            sas_query_params.push(("st", start_time))
        }
        sas_query_params.push(("se", parameters.expires_on));
        sas_query_params.push(("sp", parameters.permissions));
        if let Some(ip) = parameters.ip {
            sas_query_params.push(("sip", ip))
        }
        if let Some(protocol) = parameters.protocol {
            sas_query_params.push(("spr", protocol))
        }
        sas_query_params.push(("sig", parameters.signature));

        serde_urlencoded::to_string(&sas_query_params).unwrap()
    }
}
