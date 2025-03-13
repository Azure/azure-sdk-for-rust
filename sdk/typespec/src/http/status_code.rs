// cspell:disable
//
// The MIT License (MIT)
//
// Copyright (c) 2019 Yoshua Wuyts
// Copyright (c) 2017 http-rs authors
// Copyright (c) 2020 Jacob Brown
// Copyright (c) 2016-2018 Michael Tilli (Pyfisch) & `httpdate` contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::fmt::{self, Display};

/// HTTP response status codes.
///
/// As defined by [rfc7231 section 6](https://tools.ietf.org/html/rfc7231#section-6).
/// [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
#[repr(u16)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StatusCode {
    /// 100 Continue
    ///
    /// This interim response indicates that everything so far is OK and that
    /// the client should continue the request, or ignore the response if
    /// the request is already finished.
    Continue = 100,

    /// 101 Switching Protocols
    ///
    /// This code is sent in response to an Upgrade request header from the
    /// client, and indicates the protocol the server is switching to.
    SwitchingProtocols = 101,

    /// 103 Early Hints
    ///
    /// This status code is primarily intended to be used with the Link header,
    /// letting the user agent start preloading resources while the server
    /// prepares a response.
    EarlyHints = 103,

    /// 200 Ok
    ///
    /// The request has succeeded
    Ok = 200,

    /// 201 Created
    ///
    /// The request has succeeded and a new resource has been created as a
    /// result. This is typically the response sent after POST requests, or
    /// some PUT requests.
    Created = 201,

    /// 202 Accepted
    ///
    /// The request has been received but not yet acted upon. It is
    /// noncommittal, since there is no way in HTTP to later send an
    /// asynchronous response indicating the outcome of the request. It is
    /// intended for cases where another process or server handles the request,
    /// or for batch processing.
    Accepted = 202,

    /// 203 Non Authoritative Information
    ///
    /// This response code means the returned meta-information is not exactly
    /// the same as is available from the origin server, but is collected
    /// from a local or a third-party copy. This is mostly used for mirrors
    /// or backups of another resource. Except for that specific case, the
    /// "200 OK" response is preferred to this status.
    NonAuthoritativeInformation = 203,

    /// 204 No Content
    ///
    /// There is no content to send for this request, but the headers may be
    /// useful. The user-agent may update its cached headers for this
    /// resource with the new ones.
    NoContent = 204,

    /// 205 Reset Content
    ///
    /// Tells the user-agent to reset the document which sent this request.
    ResetContent = 205,

    /// 206 Partial Content
    ///
    /// This response code is used when the Range header is sent from the client
    /// to request only part of a resource.
    PartialContent = 206,

    /// 207 Multi-Status
    ///
    /// A Multi-Status response conveys information about
    /// multiple resources in situations where multiple
    /// status codes might be appropriate.
    MultiStatus = 207,

    /// 226 Im Used
    ///
    /// The server has fulfilled a GET request for the resource, and the
    /// response is a representation of the result of one or more
    /// instance-manipulations applied to the current instance.
    ImUsed = 226,

    /// 300 Multiple Choice
    ///
    /// The request has more than one possible response. The user-agent or user
    /// should choose one of them. (There is no standardized way of choosing
    /// one of the responses, but HTML links to the possibilities are
    /// recommended so the user can pick.)
    MultipleChoice = 300,

    /// 301 Moved Permanently
    ///
    /// The URL of the requested resource has been changed permanently. The new
    /// URL is given in the response.
    MovedPermanently = 301,

    /// 302 Found
    ///
    /// This response code means that the URI of requested resource has been
    /// changed temporarily. Further changes in the URI might be made in the
    /// future. Therefore, this same URI should be used by the client in
    /// future requests.
    Found = 302,

    /// 303 See Other
    ///
    /// The server sent this response to direct the client to get the requested
    /// resource at another URI with a GET request.
    SeeOther = 303,

    /// 304 Not Modified
    ///
    /// This is used for caching purposes. It tells the client that the response
    /// has not been modified, so the client can continue to use the same
    /// cached version of the response.
    NotModified = 304,

    /// 307 Temporary Redirect
    ///
    /// The server sends this response to direct the client to get the requested
    /// resource at another URI with same method that was used in the prior
    /// request. This has the same semantics as the 302 Found HTTP response
    /// code, with the exception that the user agent must not change the
    /// HTTP method used: If a POST was used in the first request, a POST must
    /// be used in the second request.
    TemporaryRedirect = 307,

    /// 308 Permanent Redirect
    ///
    /// This means that the resource is now permanently located at another URI,
    /// specified by the Location: HTTP Response header. This has the same
    /// semantics as the 301 Moved Permanently HTTP response code, with the
    /// exception that the user agent must not change the HTTP method
    /// used: If a POST was used in the first request, a POST must be used in
    /// the second request.
    PermanentRedirect = 308,

    /// 400 Bad Request
    ///
    /// The server could not understand the request due to invalid syntax.
    BadRequest = 400,

    /// 401 Unauthorized
    ///
    /// Although the HTTP standard specifies "unauthorized", semantically this
    /// response means "unauthenticated". That is, the client must
    /// authenticate itself to get the requested response.
    Unauthorized = 401,

    /// 402 Payment Required
    ///
    /// This response code is reserved for future use. The initial aim for
    /// creating this code was using it for digital payment systems, however
    /// this status code is used very rarely and no standard convention
    /// exists.
    PaymentRequired = 402,

    /// 403 Forbidden
    ///
    /// The client does not have access rights to the content; that is, it is
    /// unauthorized, so the server is refusing to give the requested
    /// resource. Unlike 401, the client's identity is known to the server.
    Forbidden = 403,

    /// 404 Not Found
    ///
    /// The server can not find requested resource. In the browser, this means
    /// the URL is not recognized. In an API, this can also mean that the
    /// endpoint is valid but the resource itself does not exist. Servers
    /// may also send this response instead of 403 to hide the existence of
    /// a resource from an unauthorized client. This response code is probably
    /// the most famous one due to its frequent occurrence on the web.
    NotFound = 404,

    /// 405 Method Not Allowed
    ///
    /// The request method is known by the server but has been disabled and
    /// cannot be used. For example, an API may forbid DELETE-ing a
    /// resource. The two mandatory methods, GET and HEAD, must never be
    /// disabled and should not return this error code.
    MethodNotAllowed = 405,

    /// 406 Not Acceptable
    ///
    /// This response is sent when the web server, after performing
    /// server-driven content negotiation, doesn't find any content that
    /// conforms to the criteria given by the user agent.
    NotAcceptable = 406,

    /// 407 Proxy Authentication Required
    ///
    /// This is similar to 401 but authentication is needed to be done by a
    /// proxy.
    ProxyAuthenticationRequired = 407,

    /// 408 Request Timeout
    ///
    /// This response is sent on an idle connection by some servers, even
    /// without any previous request by the client. It means that the server
    /// would like to shut down this unused connection. This response is
    /// used much more since some browsers, like Chrome, Firefox 27+,
    /// or IE9, use HTTP pre-connection mechanisms to speed up surfing. Also
    /// note that some servers merely shut down the connection without
    /// sending this message.
    RequestTimeout = 408,

    /// 409 Conflict
    ///
    /// This response is sent when a request conflicts with the current state of
    /// the server.
    Conflict = 409,

    /// 410 Gone
    ///
    /// This response is sent when the requested content has been permanently
    /// deleted from server, with no forwarding address. Clients are
    /// expected to remove their caches and links to the resource. The HTTP
    /// specification intends this status code to be used for "limited-time,
    /// promotional services". APIs should not feel compelled to indicate
    /// resources that have been deleted with this status code.
    Gone = 410,

    /// 411 Length Required
    ///
    /// Server rejected the request because the Content-Length header field is
    /// not defined and the server requires it.
    LengthRequired = 411,

    /// 412 Precondition Failed
    ///
    /// The client has indicated preconditions in its headers which the server
    /// does not meet.
    PreconditionFailed = 412,

    /// 413 Payload Too Large
    ///
    /// Request entity is larger than limits defined by server; the server might
    /// close the connection or return an Retry-After header field.
    PayloadTooLarge = 413,

    /// 414 URI Too Long
    ///
    /// The URI requested by the client is longer than the server is willing to
    /// interpret.
    UriTooLong = 414,

    /// 415 Unsupported Media Type
    ///
    /// The media format of the requested data is not supported by the server,
    /// so the server is rejecting the request.
    UnsupportedMediaType = 415,

    /// 416 Requested Range Not Satisfiable
    ///
    /// The range specified by the Range header field in the request can't be
    /// fulfilled; it's possible that the range is outside the size of the
    /// target URI's data.
    RequestedRangeNotSatisfiable = 416,

    /// 417 Expectation Failed
    ///
    /// This response code means the expectation indicated by the Expect request
    /// header field can't be met by the server.
    ExpectationFailed = 417,
    ///
    /// 418 I'm a teapot
    ///
    /// The server refuses the attempt to brew coffee with a teapot.
    ImATeapot = 418,

    /// 421 Misdirected Request
    ///
    /// The request was directed at a server that is not able to produce a
    /// response. This can be sent by a server that is not configured to
    /// produce responses for the combination of scheme and authority that
    /// are included in the request URI.
    MisdirectedRequest = 421,

    /// 422 Unprocessable Entity
    ///
    /// The request was well-formed but was unable to be followed due to
    /// semantic errors.
    UnprocessableEntity = 422,

    /// 423 Locked
    ///
    /// The resource that is being accessed is locked.
    Locked = 423,

    /// 424 Failed Dependency
    ///
    /// The request failed because it depended on another request and that
    /// request failed (e.g., a PROPPATCH).
    FailedDependency = 424,

    /// 425 Too Early
    ///
    /// Indicates that the server is unwilling to risk processing a request that
    /// might be replayed.
    TooEarly = 425,

    /// 426 Upgrade Required
    ///
    /// The server refuses to perform the request using the current protocol but
    /// might be willing to do so after the client upgrades to a different
    /// protocol. The server sends an Upgrade header in a 426 response to
    /// indicate the required protocol(s).
    UpgradeRequired = 426,

    /// 428 Precondition Required
    ///
    /// The origin server requires the request to be conditional. This response
    /// is intended to prevent the 'lost update' problem, where a client
    /// GETs a resource's state, modifies it, and PUTs it back to the
    /// server, when meanwhile a third party has modified the state on the
    /// server, leading to a conflict.
    PreconditionRequired = 428,

    /// 429 Too Many Requests
    ///
    /// The user has sent too many requests in a given amount of time ("rate
    /// limiting").
    TooManyRequests = 429,

    /// 431 Request Header Fields Too Large
    ///
    /// The server is unwilling to process the request because its header fields
    /// are too large. The request may be resubmitted after reducing the
    /// size of the request header fields.
    RequestHeaderFieldsTooLarge = 431,

    /// 451 Unavailable For Legal Reasons
    ///
    /// The user-agent requested a resource that cannot legally be provided,
    /// such as a web page censored by a government.
    UnavailableForLegalReasons = 451,

    /// 500 Internal Server Error
    ///
    /// The server has encountered a situation it doesn't know how to handle.
    InternalServerError = 500,

    /// 501 Not Implemented
    ///
    /// The request method is not supported by the server and cannot be handled.
    /// The only methods that servers are required to support (and therefore
    /// that must not return this code) are GET and HEAD.
    NotImplemented = 501,

    /// 502 Bad Gateway
    ///
    /// This error response means that the server, while working as a gateway to
    /// get a response needed to handle the request, got an invalid
    /// response.
    BadGateway = 502,

    /// 503 Service Unavailable
    ///
    /// The server is not ready to handle the request. Common causes are a
    /// server that is down for maintenance or that is overloaded. Note that
    /// together with this response, a user-friendly page explaining the
    /// problem should be sent. This responses should be used for temporary
    /// conditions and the Retry-After: HTTP header should, if possible, contain
    /// the estimated time before the recovery of the service. The webmaster
    /// must also take care about the caching-related headers that are sent
    /// along with this response, as these temporary condition responses
    /// should usually not be cached.
    ServiceUnavailable = 503,

    /// 504 Gateway Timeout
    ///
    /// This error response is given when the server is acting as a gateway and
    /// cannot get a response in time.
    GatewayTimeout = 504,

    /// 505 HTTP Version Not Supported
    ///
    /// The HTTP version used in the request is not supported by the server.
    HttpVersionNotSupported = 505,

    /// 506 Variant Also Negotiates
    ///
    /// The server has an internal configuration error: the chosen variant
    /// resource is configured to engage in transparent content negotiation
    /// itself, and is therefore not a proper end point in the negotiation
    /// process.
    VariantAlsoNegotiates = 506,

    /// 507 Insufficient Storage
    ///
    /// The server is unable to store the representation needed to complete the
    /// request.
    InsufficientStorage = 507,

    /// 508 Loop Detected
    ///
    /// The server detected an infinite loop while processing the request.
    LoopDetected = 508,

    /// 510 Not Extended
    ///
    /// Further extensions to the request are required for the server to fulfil
    /// it.
    NotExtended = 510,

    /// 511 Network Authentication Required
    ///
    /// The 511 status code indicates that the client needs to authenticate to
    /// gain network access.
    NetworkAuthenticationRequired = 511,
}

impl StatusCode {
    /// Returns `true` if the status code is `1xx` range.
    ///
    /// If this returns `true` it indicates that the request was received,
    /// continuing process.
    pub fn is_informational(&self) -> bool {
        let num: u16 = (*self).into();
        (100..200).contains(&num)
    }

    /// Returns `true` if the status code is the `2xx` range.
    ///
    /// If this returns `true` it indicates that the request was successfully
    /// received, understood, and accepted.
    pub fn is_success(&self) -> bool {
        let num: u16 = (*self).into();
        (200..300).contains(&num)
    }

    /// Returns `true` if the status code is the `3xx` range.
    ///
    /// If this returns `true` it indicates that further action needs to be
    /// taken in order to complete the request.
    pub fn is_redirection(&self) -> bool {
        let num: u16 = (*self).into();
        (300..400).contains(&num)
    }

    /// Returns `true` if the status code is the `4xx` range.
    ///
    /// If this returns `true` it indicates that the request contains bad syntax
    /// or cannot be fulfilled.
    pub fn is_client_error(&self) -> bool {
        let num: u16 = (*self).into();
        (400..500).contains(&num)
    }

    /// Returns `true` if the status code is the `5xx` range.
    ///
    /// If this returns `true` it indicates that the server failed to fulfill an
    /// apparently valid request.
    pub fn is_server_error(&self) -> bool {
        let num: u16 = (*self).into();
        (500..600).contains(&num)
    }

    /// The canonical reason for a given status code
    pub fn canonical_reason(&self) -> &'static str {
        match self {
            StatusCode::Continue => "Continue",
            StatusCode::SwitchingProtocols => "Switching Protocols",
            StatusCode::EarlyHints => "Early Hints",
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NonAuthoritativeInformation => "Non Authoritative Information",
            StatusCode::NoContent => "No Content",
            StatusCode::ResetContent => "Reset Content",
            StatusCode::PartialContent => "Partial Content",
            StatusCode::MultiStatus => "Multi-Status",
            StatusCode::ImUsed => "Im Used",
            StatusCode::MultipleChoice => "Multiple Choice",
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::Found => "Found",
            StatusCode::SeeOther => "See Other",
            StatusCode::NotModified => "Modified",
            StatusCode::TemporaryRedirect => "Temporary Redirect",
            StatusCode::PermanentRedirect => "Permanent Redirect",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::PaymentRequired => "Payment Required",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::NotAcceptable => "Not Acceptable",
            StatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
            StatusCode::RequestTimeout => "Request Timeout",
            StatusCode::Conflict => "Conflict",
            StatusCode::Gone => "Gone",
            StatusCode::LengthRequired => "Length Required",
            StatusCode::PreconditionFailed => "Precondition Failed",
            StatusCode::PayloadTooLarge => "Payload Too Large",
            StatusCode::UriTooLong => "URI Too Long",
            StatusCode::UnsupportedMediaType => "Unsupported Media Type",
            StatusCode::RequestedRangeNotSatisfiable => "Requested Range Not Satisfiable",
            StatusCode::ExpectationFailed => "Expectation Failed",
            StatusCode::ImATeapot => "I'm a teapot",
            StatusCode::MisdirectedRequest => "Misdirected Request",
            StatusCode::UnprocessableEntity => "Unprocessable Entity",
            StatusCode::Locked => "Locked",
            StatusCode::FailedDependency => "Failed Dependency",
            StatusCode::TooEarly => "Too Early",
            StatusCode::UpgradeRequired => "Upgrade Required",
            StatusCode::PreconditionRequired => "Precondition Required",
            StatusCode::TooManyRequests => "Too Many Requests",
            StatusCode::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            StatusCode::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
            StatusCode::GatewayTimeout => "Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported",
            StatusCode::VariantAlsoNegotiates => "Variant Also Negotiates",
            StatusCode::InsufficientStorage => "Insufficient Storage",
            StatusCode::LoopDetected => "Loop Detected",
            StatusCode::NotExtended => "Not Extended",
            StatusCode::NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }
}

#[cfg(feature = "json")]
mod serde {
    use super::StatusCode;
    use ::serde::{
        de::{Error as DeError, Unexpected, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use std::fmt;

    impl Serialize for StatusCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let value: u16 = *self as u16;
            serializer.serialize_u16(value)
        }
    }

    struct StatusCodeU16Visitor;

    impl Visitor<'_> for StatusCodeU16Visitor {
        type Value = StatusCode;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a u16 representing the status code")
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            use std::convert::TryFrom;
            match StatusCode::try_from(v) {
                Ok(status_code) => Ok(status_code),
                Err(_) => Err(DeError::invalid_value(
                    Unexpected::Unsigned(v as u64),
                    &self,
                )),
            }
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }
    }

    impl<'de> Deserialize<'de> for StatusCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(StatusCodeU16Visitor)
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(code: StatusCode) -> u16 {
        code as u16
    }
}

impl std::convert::TryFrom<u16> for StatusCode {
    type Error = crate::Error;

    fn try_from(num: u16) -> Result<Self, Self::Error> {
        match num {
            100 => Ok(StatusCode::Continue),
            101 => Ok(StatusCode::SwitchingProtocols),
            103 => Ok(StatusCode::EarlyHints),
            200 => Ok(StatusCode::Ok),
            201 => Ok(StatusCode::Created),
            202 => Ok(StatusCode::Accepted),
            203 => Ok(StatusCode::NonAuthoritativeInformation),
            204 => Ok(StatusCode::NoContent),
            205 => Ok(StatusCode::ResetContent),
            206 => Ok(StatusCode::PartialContent),
            207 => Ok(StatusCode::MultiStatus),
            226 => Ok(StatusCode::ImUsed),
            300 => Ok(StatusCode::MultipleChoice),
            301 => Ok(StatusCode::MovedPermanently),
            302 => Ok(StatusCode::Found),
            303 => Ok(StatusCode::SeeOther),
            304 => Ok(StatusCode::NotModified),
            307 => Ok(StatusCode::TemporaryRedirect),
            308 => Ok(StatusCode::PermanentRedirect),
            400 => Ok(StatusCode::BadRequest),
            401 => Ok(StatusCode::Unauthorized),
            402 => Ok(StatusCode::PaymentRequired),
            403 => Ok(StatusCode::Forbidden),
            404 => Ok(StatusCode::NotFound),
            405 => Ok(StatusCode::MethodNotAllowed),
            406 => Ok(StatusCode::NotAcceptable),
            407 => Ok(StatusCode::ProxyAuthenticationRequired),
            408 => Ok(StatusCode::RequestTimeout),
            409 => Ok(StatusCode::Conflict),
            410 => Ok(StatusCode::Gone),
            411 => Ok(StatusCode::LengthRequired),
            412 => Ok(StatusCode::PreconditionFailed),
            413 => Ok(StatusCode::PayloadTooLarge),
            414 => Ok(StatusCode::UriTooLong),
            415 => Ok(StatusCode::UnsupportedMediaType),
            416 => Ok(StatusCode::RequestedRangeNotSatisfiable),
            417 => Ok(StatusCode::ExpectationFailed),
            418 => Ok(StatusCode::ImATeapot),
            421 => Ok(StatusCode::MisdirectedRequest),
            422 => Ok(StatusCode::UnprocessableEntity),
            423 => Ok(StatusCode::Locked),
            424 => Ok(StatusCode::FailedDependency),
            425 => Ok(StatusCode::TooEarly),
            426 => Ok(StatusCode::UpgradeRequired),
            428 => Ok(StatusCode::PreconditionRequired),
            429 => Ok(StatusCode::TooManyRequests),
            431 => Ok(StatusCode::RequestHeaderFieldsTooLarge),
            451 => Ok(StatusCode::UnavailableForLegalReasons),
            500 => Ok(StatusCode::InternalServerError),
            501 => Ok(StatusCode::NotImplemented),
            502 => Ok(StatusCode::BadGateway),
            503 => Ok(StatusCode::ServiceUnavailable),
            504 => Ok(StatusCode::GatewayTimeout),
            505 => Ok(StatusCode::HttpVersionNotSupported),
            506 => Ok(StatusCode::VariantAlsoNegotiates),
            507 => Ok(StatusCode::InsufficientStorage),
            508 => Ok(StatusCode::LoopDetected),
            510 => Ok(StatusCode::NotExtended),
            511 => Ok(StatusCode::NetworkAuthenticationRequired),
            _ => Err(crate::error::Error::new(
                crate::error::ErrorKind::Other,
                "Invalid status code",
            )),
        }
    }
}

impl PartialEq<StatusCode> for u16 {
    fn eq(&self, other: &StatusCode) -> bool {
        *self == *other as u16
    }
}

impl PartialEq<u16> for StatusCode {
    fn eq(&self, other: &u16) -> bool {
        *self as u16 == *other
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u16)
    }
}

#[cfg(test)]
mod test {
    use super::StatusCode;

    #[cfg(feature = "json")]
    #[test]
    fn serde_as_u16() -> Result<(), serde_json::Error> {
        let status_code: StatusCode = serde_json::from_str("202")?;
        assert_eq!(StatusCode::Accepted, status_code);
        assert_eq!(
            Some(202),
            serde_json::to_value(StatusCode::Accepted)?.as_u64()
        );
        Ok(())
    }
}
