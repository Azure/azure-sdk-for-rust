use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::spec::WebVerb;

/// Calls `azure_core::Request::new` and set the authentication.
pub struct NewRequestCode {
    pub auth: AuthCode,
    pub verb: WebVerb,
    pub path: String,
}

impl ToTokens for NewRequestCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let auth = &self.auth;
        let verb = verb_to_tokens(&self.verb);
        tokens.extend(quote! {
            let mut req = azure_core::Request::new(url, #verb);
            #auth
        })
    }
}

/// Sets the authentication.
/// Only bearer token authentication is supported right now.
/// TODO: move authentication within generated crates to use policies instead of adding to requests.
pub(crate) struct AuthCode {}

impl ToTokens for AuthCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            let bearer_token = this.client.bearer_token().await?;
            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
        })
    }
}

fn verb_to_tokens(verb: &WebVerb) -> TokenStream {
    match verb {
        WebVerb::Get => quote! { azure_core::Method::GET },
        WebVerb::Post => quote! { azure_core::Method::POST },
        WebVerb::Put => quote! { azure_core::Method::PUT },
        WebVerb::Patch => quote! { azure_core::Method::PATCH },
        WebVerb::Delete => quote! { azure_core::Method::DELETE },
        WebVerb::Options => quote! { azure_core::Method::OPTION },
        WebVerb::Head => quote! { azure_core::Method::HEAD },
    }
}
