use autorust_openapi::Header;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::{codegen::TypeNameCode, doc_comment::DocCommentCode, identifier::SnakeCaseIdent, spec::TypeName, Error, ErrorKind, Result};

/// Code for a function to get a header value from the http response.
#[derive(Clone)]
pub(crate) struct HeaderCode {
    header_name: String,
    function_name: Ident,
    type_name_code: TypeNameCode,
    description: DocCommentCode,
}

impl HeaderCode {
    pub fn new(header_name: String, header: &Header) -> Result<Self> {
        let function_name = header_name.to_snake_case_ident()?; // ETag as e_tag
        let header_name = header_name.to_lowercase(); // ETag as etag
        let type_name = match (header.type_.as_str(), header.format.as_deref()) {
            ("string", None) => Ok(TypeName::String),
            ("string", Some("byte")) => Ok(TypeName::String), // base64-encoded
            ("string", Some("duration")) => Ok(TypeName::String),
            ("string", Some("uuid")) => Ok(TypeName::String),
            ("string", Some("uri")) => Ok(TypeName::String),
            ("string", Some("date-time-rfc1123")) => Ok(TypeName::DateTimeRfc1123),
            ("integer", None) => Ok(TypeName::Int32),
            ("integer", Some("int32")) => Ok(TypeName::Int32),
            ("number", None) => Ok(TypeName::Float32),
            ("number", Some("float")) => Ok(TypeName::Float32),
            ("number", Some("double")) => Ok(TypeName::Float64),
            ("integer", Some("int64")) => Ok(TypeName::Int64),
            ("boolean", None) => Ok(TypeName::Boolean),
            (header_type, header_format) => Err(Error::with_message(ErrorKind::CodeGen, || {
                format!("header type '{header_type}' format '{header_format:?}' not matched")
            })),
        }?;
        let type_name_code = TypeNameCode::new(&type_name)?;
        let description = DocCommentCode::new(header.description.clone());
        Ok(Self {
            header_name,
            function_name,
            type_name_code,
            description,
        })
    }
}

impl ToTokens for HeaderCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            header_name,
            function_name,
            type_name_code,
            description,
        } = &self;
        let hdr_fn = if type_name_code.is_string() {
            quote! {
                pub fn #function_name(&self) -> azure_core::Result<&str> {
                    self.0.get_str(&azure_core::headers::HeaderName::from_static(#header_name))
                }
            }
        } else if type_name_code.is_date_time() {
            quote! {
                pub fn #function_name(&self) -> azure_core::Result<time::OffsetDateTime> {
                    azure_core::date::parse_rfc3339(self.0.get_str(&azure_core::headers::HeaderName::from_static(#header_name))?)
                }
            }
        } else if type_name_code.is_date_time_rfc1123() {
            quote! {
                pub fn #function_name(&self) -> azure_core::Result<time::OffsetDateTime> {
                    azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static(#header_name))?)
                }
            }
        } else {
            quote! {
                pub fn #function_name(&self) -> azure_core::Result<#type_name_code> {
                    self.0.get_as(&azure_core::headers::HeaderName::from_static(#header_name))
                }
            }
        };
        description.to_tokens(tokens);
        hdr_fn.to_tokens(tokens)
    }
}

/// [HeadersCode] is a collection of [HeaderCode]s for the response of a request
#[derive(Clone)]
pub(crate) struct HeadersCode {
    headers: Vec<HeaderCode>,
}

impl HeadersCode {
    pub fn new(headers: Vec<HeaderCode>) -> Self {
        Self { headers }
    }

    pub fn has_headers(&self) -> bool {
        !self.headers.is_empty()
    }
}

impl ToTokens for HeadersCode {
    /// Generates tokens to access the headers returned in the Response.
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.has_headers() {
            let headers = &self.headers;
            tokens.extend(quote! {
                pub struct Headers<'a>(&'a azure_core::headers::Headers);
                impl<'a> Headers<'a> {
                    #(#headers)*
                }
            })
        }
    }
}
