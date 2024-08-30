use proc_macro2::{Span, TokenStream};

pub type Result<T> = ::std::result::Result<T, Vec<Error>>;

pub struct Error {
    span: Span,
    message: String,
}

impl Error {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        let message = message.into();
        Error { span, message }
    }
}

impl Into<TokenStream> for Error {
    fn into(self) -> TokenStream {
        let message = self.message;
        quote::quote_spanned!(self.span=> compile_error!(#message))
    }
}
