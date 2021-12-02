#[derive(Clone, Debug, PartialEq)]
pub struct TableSasQueryParameters {
    /// Gets the storage service version to use to authenticate requests
    /// made with this shared access signature, and the service version to
    /// use when handling requests made with this shared access signature.
    version: String,
    protocol: String,
    start_on: String,

    /// The signature is an HMAC computed over the string-to-sign and key
    /// using the SHA256 algorithm, and then encoded using Base64 encoding.
    signature: String,

    expires_on: String,
    permissions: String,
    resource_types: String,

    ip_range: Option<String>,
    resource: Option<String>,
    identifier: Option<String>,
}

impl TableSasQueryParameters {
    pub fn new(
        version: String,
        resource_types: String,
        protocol: String,
        start_on: String,
        expires_on: String,
        permissions: String,
        signature: String,
        ip_range: Option<String>,
        resource: Option<String>,
        identifier: Option<String>,
    ) -> Self {
        Self {
            version,
            resource_types,
            protocol,
            start_on,
            expires_on,
            permissions,
            signature,
            ip_range,
            resource,
            identifier,
        }
    }
}

// sv=2019-07-07&ss=t&srt=c&se=2021-11-29T04%3A22%3A38Z&sp=r&sig=79EMOCtIXnD3IlP2FRO9qO4Ac6XWgL%2BIF9o882bUMOM%3D
impl From<TableSasQueryParameters> for String {
    fn from(parameters: TableSasQueryParameters) -> Self {
        format!(
            "sv={}ss=tsrt={}se={}sp={}sig={}",
            parameters.version,
            parameters.resource_types,
            parameters.expires_on,
            parameters.permissions,
            parameters.signature
        )
    }
}
