use crate::headers::{self, Header};

/// Advertises which content types the client is able to understand.
///
/// The Accept request HTTP header advertises which content types, expressed
/// as MIME types, the client is able to understand. Using content
/// negotiation, the server then selects one of the proposals, uses it and
/// informs the client of its choice with the Content-Type response header.
/// Browsers set adequate values for this header depending of the context
/// where the request is done: when fetching a CSS stylesheet a different
/// value is set for the request than when fetching an image, video or a
/// script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accept(String);

impl Accept {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl<S> From<S> for Accept
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Header for Accept {
    fn name(&self) -> headers::HeaderName {
        headers::ACCEPT
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
