#![allow(unused_doc_comments)]

use crate::identifier::ident;
use autorust_openapi::{Response, StatusCode};
use heck::{CamelCase, SnakeCase};
use http::StatusCode as HttpStatusCode;
use indexmap::IndexMap;
use proc_macro2::TokenStream;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid status code: {0}")]
    InvalidStatusCode(#[from] http::status::InvalidStatusCode),
    #[error("no canical reasons for status code: {0}")]
    NoCanonicalReason(u16),
    #[error("no status code for default")]
    NoStatusCodeForDefault,
    #[error("creating name for status code: {0}")]
    StatusCodeName(#[source] crate::identifier::Error),
    #[error("creating type name for response: {0}")]
    ResponseTypeName(#[source] crate::identifier::Error),
}

fn get_status_code_name_u16(status_code: &u16) -> Result<&'static str, Error> {
    let sc = HttpStatusCode::from_u16(*status_code)?;
    sc.canonical_reason()
        .ok_or_else(|| Error::NoCanonicalReason(status_code.to_owned()))
}

/// Get the status code canonical reason
pub fn get_status_code_name(status_code: &StatusCode) -> Result<&'static str, Error> {
    match status_code {
        StatusCode::Code(status_code) => Ok(get_status_code_name_u16(status_code)?),
        StatusCode::Default => Err(Error::NoStatusCodeForDefault),
    }
}

/// The canonical name.
/// examples: OK, CREATED, LOOP_DETECTED
pub fn get_status_code_ident(status_code: &StatusCode) -> Result<TokenStream, Error> {
    ident(&get_status_code_name(status_code)?.to_snake_case().to_uppercase()).map_err(Error::StatusCodeName)
}

#[allow(dead_code)]
/// The canonical name in camel case.
/// examples: Ok, Created, LoopDetected
pub fn get_status_code_ident_camel_case(status_code: &StatusCode) -> Result<TokenStream, Error> {
    ident(&get_status_code_name(status_code)?.to_camel_case()).map_err(Error::StatusCodeName)
}

fn response_name(status_code: &HttpStatusCode) -> Result<String, Error> {
    let sc = status_code.as_u16();
    let name = status_code.canonical_reason().ok_or(Error::NoCanonicalReason(sc))?;
    let name = name.to_camel_case();
    Ok(format!("{}{}", name, sc))
}

/// The canonical name in camel case with the u16 appended.
/// examples: Ok200, Created201, LoopDetected508
pub fn get_response_type_name(status_code: &StatusCode) -> Result<String, Error> {
    match status_code {
        StatusCode::Code(status_code) => {
            let sc = HttpStatusCode::from_u16(*status_code)?;
            Ok(response_name(&sc)?)
        }
        StatusCode::Default => Ok("DefaultResponse".to_owned()),
    }
}

pub fn get_response_type_ident(status_code: &StatusCode) -> Result<TokenStream, Error> {
    ident(&get_response_type_name(status_code)?).map_err(Error::ResponseTypeName)
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
    fn test_response_name() -> Result<(), Error> {
        assert_eq!("Ok200", response_name(&HttpStatusCode::OK)?);
        assert_eq!("FailedDependency424", response_name(&HttpStatusCode::FAILED_DEPENDENCY)?);
        assert_eq!(
            "HttpVersionNotSupported505",
            response_name(&HttpStatusCode::HTTP_VERSION_NOT_SUPPORTED)?
        );
        Ok(())
    }

    #[test]
    fn test_get_status_code_name() -> Result<(), Error> {
        assert_eq!("Loop Detected", get_status_code_name_u16(&508)?);
        Ok(())
    }
}
