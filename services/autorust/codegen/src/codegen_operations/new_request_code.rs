use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::spec::WebVerb;

use super::auth_code::AuthCode;
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

fn verb_to_tokens(verb: &WebVerb) -> TokenStream {
    match verb {
        WebVerb::Get => quote! { azure_core::Method::Get },
        WebVerb::Post => quote! { azure_core::Method::Post },
        WebVerb::Put => quote! { azure_core::Method::Put },
        WebVerb::Patch => quote! { azure_core::Method::Patch },
        WebVerb::Delete => quote! { azure_core::Method::Delete },
        WebVerb::Options => quote! { azure_core::Method::Option },
        WebVerb::Head => quote! { azure_core::Method::Head },
    }
}
