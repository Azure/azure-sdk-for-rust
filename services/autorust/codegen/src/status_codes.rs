#![allow(unused_doc_comments)]

use crate::identifier::parse_ident;
use crate::{Error, ErrorKind, Result};
use autorust_openapi::StatusCode;
use heck::ToPascalCase;
use http_types::StatusCode as HttpStatusCode;
use proc_macro2::Ident;
use std::convert::TryFrom;

fn try_from_u16(status_code: u16) -> Result<HttpStatusCode> {
    HttpStatusCode::try_from(status_code)
        .map_err(|_| Error::with_message(ErrorKind::Parse, || format!("invalid status code '{status_code}'")))
}

/// Get the status code canonical reason
pub fn get_status_code_name(status_code: &StatusCode) -> Result<&'static str> {
    match status_code {
        StatusCode::Code(status_code) => Ok(try_from_u16(*status_code)?.canonical_reason()),
        StatusCode::Default => Err(Error::with_message(ErrorKind::Parse, || "no status code name for default")),
    }
}

/// The canonical name in camel case.
/// examples: Ok, Created, LoopDetected
pub fn get_status_code_ident(status_code: &StatusCode) -> Result<Ident> {
    match status_code {
        StatusCode::Code(_) => parse_ident(&get_status_code_name(status_code)?.to_pascal_case()),
        StatusCode::Default => parse_ident("Default"),
    }
}

pub fn is_success(status_code: &StatusCode) -> bool {
    match status_code {
        StatusCode::Code(status_code) => match try_from_u16(*status_code) {
            Ok(status_code) => status_code.is_success(),
            Err(_) => false,
        },
        StatusCode::Default => false,
    }
}

pub fn is_error(status_code: &StatusCode) -> bool {
    match status_code {
        StatusCode::Code(status_code) => match try_from_u16(*status_code) {
            Ok(status_code) => !status_code.is_success(),
            Err(_) => false,
        },
        StatusCode::Default => false,
    }
}

pub fn is_default(status_code: &StatusCode) -> bool {
    match status_code {
        StatusCode::Code(_status_code) => false,
        StatusCode::Default => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status_code_name() -> Result<()> {
        assert_eq!("Loop Detected", get_status_code_name(&StatusCode::Code(508))?);
        Ok(())
    }
}
