#![allow(unused_doc_comments)]

use crate::identifier::parse_ident;
use crate::{Error, ErrorKind, Result, ResultExt};
use autorust_openapi::{Response, StatusCode};
use heck::{ToPascalCase, ToSnakeCase};
use http::StatusCode as HttpStatusCode;
use indexmap::IndexMap;
use proc_macro2::Ident;

fn get_status_code_name_u16(status_code: &u16) -> Result<&'static str> {
    let sc = HttpStatusCode::from_u16(*status_code).map_kind(ErrorKind::Parse)?;
    get_canonical_reason(&sc)
}

fn get_canonical_reason(status_code: &HttpStatusCode) -> Result<&'static str> {
    status_code
        .canonical_reason()
        .ok_or_else(|| Error::with_message(ErrorKind::Parse, || format!("no canonical reason for status code {status_code}")))
}

/// Get the status code canonical reason
pub fn get_status_code_name(status_code: &StatusCode) -> Result<&'static str> {
    match status_code {
        StatusCode::Code(status_code) => Ok(get_status_code_name_u16(status_code)?),
        StatusCode::Default => Err(Error::with_message(ErrorKind::Parse, || {
            format!("no status code for default {status_code}")
        })),
    }
}

/// The canonical name.
/// examples: OK, CREATED, LOOP_DETECTED
pub fn get_status_code_ident(status_code: &StatusCode) -> Result<Ident> {
    parse_ident(&get_status_code_name(status_code)?.to_snake_case().to_uppercase())
}

#[allow(dead_code)]
/// The canonical name in camel case.
/// examples: Ok, Created, LoopDetected
pub fn get_status_code_ident_camel_case(status_code: &StatusCode) -> Result<Ident> {
    parse_ident(&get_status_code_name(status_code)?.to_pascal_case())
}

fn response_name(status_code: &HttpStatusCode) -> Result<String> {
    let reason = get_canonical_reason(&status_code)?;
    let reason = reason.to_pascal_case();
    let status_code = status_code.as_u16();
    Ok(format!("{reason}{status_code}"))
}

/// The canonical name in camel case with the u16 appended.
/// examples: Ok200, Created201, LoopDetected508
pub fn get_response_type_name(status_code: &StatusCode) -> Result<String> {
    match status_code {
        StatusCode::Code(status_code) => {
            let sc = HttpStatusCode::from_u16(*status_code).map_kind(ErrorKind::Parse)?;
            Ok(response_name(&sc)?)
        }
        StatusCode::Default => Ok("DefaultResponse".to_owned()),
    }
}

pub fn get_response_type_ident(status_code: &StatusCode) -> Result<Ident> {
    parse_ident(&get_response_type_name(status_code)?)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_name() -> Result<()> {
        assert_eq!("Ok200", response_name(&HttpStatusCode::OK)?);
        assert_eq!("FailedDependency424", response_name(&HttpStatusCode::FAILED_DEPENDENCY)?);
        assert_eq!(
            "HttpVersionNotSupported505",
            response_name(&HttpStatusCode::HTTP_VERSION_NOT_SUPPORTED)?
        );
        Ok(())
    }

    #[test]
    fn test_get_status_code_name() -> Result<()> {
        assert_eq!("Loop Detected", get_status_code_name_u16(&508)?);
        Ok(())
    }
}
