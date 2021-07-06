#![allow(unused_doc_comments)]

use std::collections::HashMap;

use autorust_openapi::{Response, StatusCode};
use heck::CamelCase;
use http::StatusCode as HttpStatusCode;
use indexmap::IndexMap;
use once_cell::sync::Lazy;

// for looking up the const names
// https://github.com/hyperium/http/blob/master/src/status.rs
// found \((\d+), (\w+), "[\w ]+"\);
// replaced with ($1, "$2"),
const STATUS_CODES: &[(u16, &str)] = &[
    /// 100 Continue
    /// [[RFC7231, Section 6.2.1](https://tools.ietf.org/html/rfc7231#section-6.2.1)]
    (100, "CONTINUE"),
    /// 101 Switching Protocols
    /// [[RFC7231, Section 6.2.2](https://tools.ietf.org/html/rfc7231#section-6.2.2)]
    (101, "SWITCHING_PROTOCOLS"),
    /// 102 Processing
    /// [[RFC2518](https://tools.ietf.org/html/rfc2518)]
    (102, "PROCESSING"),
    /// 200 OK
    /// [[RFC7231, Section 6.3.1](https://tools.ietf.org/html/rfc7231#section-6.3.1)]
    (200, "OK"),
    /// 201 Created
    /// [[RFC7231, Section 6.3.2](https://tools.ietf.org/html/rfc7231#section-6.3.2)]
    (201, "CREATED"),
    /// 202 Accepted
    /// [[RFC7231, Section 6.3.3](https://tools.ietf.org/html/rfc7231#section-6.3.3)]
    (202, "ACCEPTED"),
    /// 203 Non-Authoritative Information
    /// [[RFC7231, Section 6.3.4](https://tools.ietf.org/html/rfc7231#section-6.3.4)]
    (203, "NON_AUTHORITATIVE_INFORMATION"),
    /// 204 No Content
    /// [[RFC7231, Section 6.3.5](https://tools.ietf.org/html/rfc7231#section-6.3.5)]
    (204, "NO_CONTENT"),
    /// 205 Reset Content
    /// [[RFC7231, Section 6.3.6](https://tools.ietf.org/html/rfc7231#section-6.3.6)]
    (205, "RESET_CONTENT"),
    /// 206 Partial Content
    /// [[RFC7233, Section 4.1](https://tools.ietf.org/html/rfc7233#section-4.1)]
    (206, "PARTIAL_CONTENT"),
    /// 207 Multi-Status
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (207, "MULTI_STATUS"),
    /// 208 Already Reported
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    (208, "ALREADY_REPORTED"),
    /// 226 IM Used
    /// [[RFC3229](https://tools.ietf.org/html/rfc3229)]
    (226, "IM_USED"),
    /// 300 Multiple Choices
    /// [[RFC7231, Section 6.4.1](https://tools.ietf.org/html/rfc7231#section-6.4.1)]
    (300, "MULTIPLE_CHOICES"),
    /// 301 Moved Permanently
    /// [[RFC7231, Section 6.4.2](https://tools.ietf.org/html/rfc7231#section-6.4.2)]
    (301, "MOVED_PERMANENTLY"),
    /// 302 Found
    /// [[RFC7231, Section 6.4.3](https://tools.ietf.org/html/rfc7231#section-6.4.3)]
    (302, "FOUND"),
    /// 303 See Other
    /// [[RFC7231, Section 6.4.4](https://tools.ietf.org/html/rfc7231#section-6.4.4)]
    (303, "SEE_OTHER"),
    /// 304 Not Modified
    /// [[RFC7232, Section 4.1](https://tools.ietf.org/html/rfc7232#section-4.1)]
    (304, "NOT_MODIFIED"),
    /// 305 Use Proxy
    /// [[RFC7231, Section 6.4.5](https://tools.ietf.org/html/rfc7231#section-6.4.5)]
    (305, "USE_PROXY"),
    /// 307 Temporary Redirect
    /// [[RFC7231, Section 6.4.7](https://tools.ietf.org/html/rfc7231#section-6.4.7)]
    (307, "TEMPORARY_REDIRECT"),
    /// 308 Permanent Redirect
    /// [[RFC7238](https://tools.ietf.org/html/rfc7238)]
    (308, "PERMANENT_REDIRECT"),
    /// 400 Bad Request
    /// [[RFC7231, Section 6.5.1](https://tools.ietf.org/html/rfc7231#section-6.5.1)]
    (400, "BAD_REQUEST"),
    /// 401 Unauthorized
    /// [[RFC7235, Section 3.1](https://tools.ietf.org/html/rfc7235#section-3.1)]
    (401, "UNAUTHORIZED"),
    /// 402 Payment Required
    /// [[RFC7231, Section 6.5.2](https://tools.ietf.org/html/rfc7231#section-6.5.2)]
    (402, "PAYMENT_REQUIRED"),
    /// 403 Forbidden
    /// [[RFC7231, Section 6.5.3](https://tools.ietf.org/html/rfc7231#section-6.5.3)]
    (403, "FORBIDDEN"),
    /// 404 Not Found
    /// [[RFC7231, Section 6.5.4](https://tools.ietf.org/html/rfc7231#section-6.5.4)]
    (404, "NOT_FOUND"),
    /// 405 Method Not Allowed
    /// [[RFC7231, Section 6.5.5](https://tools.ietf.org/html/rfc7231#section-6.5.5)]
    (405, "METHOD_NOT_ALLOWED"),
    /// 406 Not Acceptable
    /// [[RFC7231, Section 6.5.6](https://tools.ietf.org/html/rfc7231#section-6.5.6)]
    (406, "NOT_ACCEPTABLE"),
    /// 407 Proxy Authentication Required
    /// [[RFC7235, Section 3.2](https://tools.ietf.org/html/rfc7235#section-3.2)]
    (407, "PROXY_AUTHENTICATION_REQUIRED"),
    /// 408 Request Timeout
    /// [[RFC7231, Section 6.5.7](https://tools.ietf.org/html/rfc7231#section-6.5.7)]
    (408, "REQUEST_TIMEOUT"),
    /// 409 Conflict
    /// [[RFC7231, Section 6.5.8](https://tools.ietf.org/html/rfc7231#section-6.5.8)]
    (409, "CONFLICT"),
    /// 410 Gone
    /// [[RFC7231, Section 6.5.9](https://tools.ietf.org/html/rfc7231#section-6.5.9)]
    (410, "GONE"),
    /// 411 Length Required
    /// [[RFC7231, Section 6.5.10](https://tools.ietf.org/html/rfc7231#section-6.5.10)]
    (411, "LENGTH_REQUIRED"),
    /// 412 Precondition Failed
    /// [[RFC7232, Section 4.2](https://tools.ietf.org/html/rfc7232#section-4.2)]
    (412, "PRECONDITION_FAILED"),
    /// 413 Payload Too Large
    /// [[RFC7231, Section 6.5.11](https://tools.ietf.org/html/rfc7231#section-6.5.11)]
    (413, "PAYLOAD_TOO_LARGE"),
    /// 414 URI Too Long
    /// [[RFC7231, Section 6.5.12](https://tools.ietf.org/html/rfc7231#section-6.5.12)]
    (414, "URI_TOO_LONG"),
    /// 415 Unsupported Media Type
    /// [[RFC7231, Section 6.5.13](https://tools.ietf.org/html/rfc7231#section-6.5.13)]
    (415, "UNSUPPORTED_MEDIA_TYPE"),
    /// 416 Range Not Satisfiable
    /// [[RFC7233, Section 4.4](https://tools.ietf.org/html/rfc7233#section-4.4)]
    (416, "RANGE_NOT_SATISFIABLE"),
    /// 417 Expectation Failed
    /// [[RFC7231, Section 6.5.14](https://tools.ietf.org/html/rfc7231#section-6.5.14)]
    (417, "EXPECTATION_FAILED"),
    /// 418 I'm a teapot
    /// [curiously not registered by IANA but [RFC2324](https://tools.ietf.org/html/rfc2324)]
    (418, "IM_A_TEAPOT"),
    /// 421 Misdirected Request
    /// [RFC7540, Section 9.1.2](http://tools.ietf.org/html/rfc7540#section-9.1.2)
    (421, "MISDIRECTED_REQUEST"),
    /// 422 Unprocessable Entity
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (422, "UNPROCESSABLE_ENTITY"),
    /// 423 Locked
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (423, "LOCKED"),
    /// 424 Failed Dependency
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (424, "FAILED_DEPENDENCY"),
    /// 426 Upgrade Required
    /// [[RFC7231, Section 6.5.15](https://tools.ietf.org/html/rfc7231#section-6.5.15)]
    (426, "UPGRADE_REQUIRED"),
    /// 428 Precondition Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (428, "PRECONDITION_REQUIRED"),
    /// 429 Too Many Requests
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (429, "TOO_MANY_REQUESTS"),
    /// 431 Request Header Fields Too Large
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (431, "REQUEST_HEADER_FIELDS_TOO_LARGE"),
    /// 451 Unavailable For Legal Reasons
    /// [[RFC7725](http://tools.ietf.org/html/rfc7725)]
    (451, "UNAVAILABLE_FOR_LEGAL_REASONS"),
    /// 500 Internal Server Error
    /// [[RFC7231, Section 6.6.1](https://tools.ietf.org/html/rfc7231#section-6.6.1)]
    (500, "INTERNAL_SERVER_ERROR"),
    /// 501 Not Implemented
    /// [[RFC7231, Section 6.6.2](https://tools.ietf.org/html/rfc7231#section-6.6.2)]
    (501, "NOT_IMPLEMENTED"),
    /// 502 Bad Gateway
    /// [[RFC7231, Section 6.6.3](https://tools.ietf.org/html/rfc7231#section-6.6.3)]
    (502, "BAD_GATEWAY"),
    /// 503 Service Unavailable
    /// [[RFC7231, Section 6.6.4](https://tools.ietf.org/html/rfc7231#section-6.6.4)]
    (503, "SERVICE_UNAVAILABLE"),
    /// 504 Gateway Timeout
    /// [[RFC7231, Section 6.6.5](https://tools.ietf.org/html/rfc7231#section-6.6.5)]
    (504, "GATEWAY_TIMEOUT"),
    /// 505 HTTP Version Not Supported
    /// [[RFC7231, Section 6.6.6](https://tools.ietf.org/html/rfc7231#section-6.6.6)]
    (505, "HTTP_VERSION_NOT_SUPPORTED"),
    /// 506 Variant Also Negotiates
    /// [[RFC2295](https://tools.ietf.org/html/rfc2295)]
    (506, "VARIANT_ALSO_NEGOTIATES"),
    /// 507 Insufficient Storage
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (507, "INSUFFICIENT_STORAGE"),
    /// 508 Loop Detected
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    (508, "LOOP_DETECTED"),
    /// 510 Not Extended
    /// [[RFC2774](https://tools.ietf.org/html/rfc2774)]
    (510, "NOT_EXTENDED"),
    /// 511 Network Authentication Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (511, "NETWORK_AUTHENTICATION_REQUIRED"),
];

static STATUS_CODE_MAP: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| STATUS_CODES.iter().cloned().collect());

fn get_status_code_name_u16(status_code: &u16) -> String {
    let name = STATUS_CODE_MAP.get(status_code);
    name.unwrap_or(&"INVALID").to_string() // TODO Error
}

pub fn get_status_code_name(status_code: &StatusCode) -> String {
    match status_code {
        StatusCode::Code(status_code) => get_status_code_name_u16(status_code),
        StatusCode::Default => "INVALID".to_owned(), // TODO Error
    }
}

fn get_response_name(status_code: &HttpStatusCode) -> String {
    let name = status_code.canonical_reason().unwrap_or("StatusCode");
    let name = name.to_camel_case();
    format!("{}{}", name, status_code.as_u16())
}

pub fn get_response_type_name(status_code: &StatusCode) -> String {
    match status_code {
        StatusCode::Code(status_code) => match HttpStatusCode::from_u16(*status_code) {
            Ok(status_code) => get_response_name(&status_code),
            Err(_) => format!("StatusCode{}", status_code),
        },
        StatusCode::Default => "DefaultResponse".to_owned(),
    }
}

fn is_success(status_code: &StatusCode) -> bool {
    match status_code {
        StatusCode::Code(status_code) => match HttpStatusCode::from_u16(*status_code) {
            Ok(status_code) => status_code.is_success(),
            Err(_) => false,
        },
        StatusCode::Default => false,
    }
}

pub fn get_success_responses(responses: &IndexMap<StatusCode, Response>) -> IndexMap<StatusCode, Response> {
    let mut map = IndexMap::new();
    for (status_code, rsp) in responses {
        if is_success(status_code) {
            map.insert(status_code.to_owned(), rsp.to_owned());
        }
    }
    map
}

pub fn get_error_responses(responses: &IndexMap<StatusCode, Response>) -> IndexMap<StatusCode, Response> {
    let mut map = IndexMap::new();
    for (status_code, rsp) in responses {
        if !is_success(status_code) {
            map.insert(status_code.to_owned(), rsp.to_owned());
        }
    }
    map
}

pub fn has_default_response(responses: &IndexMap<StatusCode, Response>) -> bool {
    for (status_code, _rsp) in responses {
        match status_code {
            StatusCode::Code(_) => {}
            StatusCode::Default => return true,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_response_name() {
        assert_eq!("Ok200", get_response_name(&HttpStatusCode::OK));
        assert_eq!("FailedDependency424", get_response_name(&HttpStatusCode::FAILED_DEPENDENCY));
        assert_eq!(
            "HttpVersionNotSupported505",
            get_response_name(&HttpStatusCode::HTTP_VERSION_NOT_SUPPORTED)
        );
    }

    #[test]
    fn test_get_status_code_name() {
        assert_eq!("LOOP_DETECTED", get_status_code_name_u16(&508).as_str());
    }
}
