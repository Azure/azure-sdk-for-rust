use crate::headers::{self, Header};

/// Advertises which content encoding the client is able to understand.
///
/// The Accept-Encoding request HTTP header advertises which content
/// encoding, usually a compression algorithm, the client is able to
/// understand. Using content negotiation, the server selects one of the
/// proposals, uses it and informs the client of its choice with the
/// Content-Encoding response header.
///
/// Even if both the client and the server supports the same compression
/// algorithms, the server may choose not to compress the body of a
/// response, if the identity value is also acceptable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptEncoding(String);

impl AcceptEncoding {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl<S> From<S> for AcceptEncoding
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Header for AcceptEncoding {
    fn name(&self) -> headers::HeaderName {
        headers::ACCEPT_ENCODING
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.clone().into()
    }
}
