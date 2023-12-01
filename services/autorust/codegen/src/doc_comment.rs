//! Provides the [DocCommentCode] struct, which can be used to generate doc comment tokens.
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone)]
pub struct DocCommentCode {
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

impl From<&str> for DocCommentCode {
    fn from(comment: &str) -> Self {
        Self {
            comment: Some(comment.to_string()),
        }
    }
}

impl From<Option<String>> for DocCommentCode {
    fn from(comment: Option<String>) -> Self {
        Self { comment }
    }
}

impl From<&Option<String>> for DocCommentCode {
    fn from(comment: &Option<String>) -> Self {
        Self { comment: comment.clone() }
    }
}

impl ToTokens for DocCommentCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(comment) = &self.comment {
            tokens.extend(quote! { #[doc = #comment] })
        }
    }
}
