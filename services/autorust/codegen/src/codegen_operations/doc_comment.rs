// TODO: there's some duplication in the codegen_models.rs
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone)]
pub(crate) struct DocCommentCode {
    comment: Option<String>,
}

impl DocCommentCode {
    pub fn new(comment: Option<String>) -> Self {
        Self { comment }
    }
    pub fn is_empty(&self) -> bool {
        if let Some(comment) = &self.comment {
            comment.is_empty()
        } else {
            true
        }
    }
}

impl ToTokens for DocCommentCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(comment) = &self.comment {
            tokens.extend(quote! { #[doc = #comment] })
        }
    }
}
