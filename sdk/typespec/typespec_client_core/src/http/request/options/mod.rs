// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Options sent with requests to the service.

#[macro_use]
mod macros;

mod content_length;
mod if_match_condition;
mod if_modified_since;
mod if_modified_since_condition;

use crate::http::headers::{
    ACCEPT, ACCEPT_ENCODING, CONTENT_ENCODING, CONTENT_LANGUAGE, CONTENT_TYPE, USER_AGENT,
};
pub use content_length::ContentLength;
pub use if_match_condition::IfMatchCondition;
pub use if_modified_since::IfModifiedSince;
pub use if_modified_since_condition::IfModifiedSinceCondition;

request_header!(
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
    AcceptEncoding,
    ACCEPT_ENCODING,
);

request_header!(
    /// The Content Type indicates the media type of the request body
    ContentType,
    CONTENT_TYPE,
    (APPLICATION_JSON, "application/json")
);

request_header!(
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
    Accept,
    ACCEPT
);

request_header!(ContentEncoding, CONTENT_ENCODING);
request_header!(ContentLanguage, CONTENT_LANGUAGE);
request_header!(UserAgent, USER_AGENT);
