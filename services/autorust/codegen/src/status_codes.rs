#![allow(unused_doc_comments)]

use std::convert::TryFrom;

use crate::identifier::parse_ident;
use crate::{Error, ErrorKind, Result};
use autorust_openapi::{Response, StatusCode};
use heck::ToPascalCase;
use http_types::StatusCode as HttpStatusCode;
use indexmap::IndexMap;
use proc_macro2::Ident;

fn try_from_u16(status_code: u16) -> Result<HttpStatusCode> {
    HttpStatusCode::try_from(status_code)
        .map_err(|_| Error::with_message(ErrorKind::Parse, || format!("invalid status code '{status_code}'")))
}

/// Get the status code canonical reason
pub fn get_status_code_name(status_code: &StatusCode) -> Result<&'static str> {
    match status_code {
        StatusCode::Code(status_code) => Ok(try_from_u16(*status_code)?.canonical_reason()),
        StatusCode::Default => Err(Error::with_message(ErrorKind::Parse, || {
            format!("no status code for default {status_code}")
        })),
    }
}

/// The canonical name in camel case.
/// examples: Ok, Created, LoopDetected
pub fn get_status_code_ident(status_code: &StatusCode) -> Result<Ident> {
    parse_ident(&get_status_code_name(status_code)?.to_pascal_case())
}

fn response_name(status_code: HttpStatusCode) -> Result<String> {
    let reason = status_code.canonical_reason().to_pascal_case();
    let status_code = status_code as u16;
    Ok(format!("{reason}{status_code}"))
}

/// The canonical name in camel case with the u16 appended.
/// examples: Ok200, Created201, LoopDetected508
pub fn get_response_type_name(status_code: &StatusCode) -> Result<String> {
    match status_code {
        StatusCode::Code(status_code) => {
            let sc = try_from_u16(*status_code)?;
            Ok(response_name(sc)?)
        }
        StatusCode::Default => Ok("DefaultResponse".to_owned()),
    }
}

pub fn get_response_type_ident(status_code: &StatusCode) -> Result<Ident> {
    parse_ident(&get_response_type_name(status_code)?)
}

fn is_success(status_code: &StatusCode) -> bool {
    match status_code {
        StatusCode::Code(status_code) => match try_from_u16(*status_code) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_name() -> Result<()> {
        assert_eq!("Ok200", response_name(HttpStatusCode::Ok)?);
        assert_eq!("FailedDependency424", response_name(HttpStatusCode::FailedDependency)?);
        assert_eq!(
            "HttpVersionNotSupported505",
            response_name(HttpStatusCode::HttpVersionNotSupported)?
        );
        Ok(())
    }

    #[test]
    fn test_get_status_code_name() -> Result<()> {
        assert_eq!("Loop Detected", get_status_code_name(&StatusCode::Code(508))?);
        Ok(())
    }
}
