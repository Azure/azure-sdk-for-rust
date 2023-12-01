use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// Sets the authentication.
/// Only bearer token authentication is supported right now.
/// TODO: move authentication within generated crates to use policies instead of adding to requests.
pub(crate) struct AuthCode {}

impl ToTokens for AuthCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            let credential = this.client.token_credential();
            let token_response = credential
                .get_token(&this.client.scopes().join(" "))
                .await?;
            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        })
    }
}
