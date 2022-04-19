use crate::{codegen::create_generated_by_header, write_file};
use camino::Utf8Path;
use proc_macro2::TokenStream;
use quote::quote;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] crate::io::Error),
    #[error("creating module name for feature {feature}: {source}")]
    ModName { source: crate::identifier::Error, feature: String },
}

pub fn create(path: &Utf8Path, print_writing_file: bool) -> Result<()> {
    Ok(write_file(path, &create_body()?, print_writing_file)?)
}

fn create_body() -> Result<TokenStream> {
    let mut cfgs = TokenStream::new();
    cfgs.extend(quote! {
        pub mod models;
        pub mod operations;
        pub use crate::{operations::{Client, ClientBuilder, Error}};
    });
    let generated_by = create_generated_by_header();
    Ok(quote! {
        #![allow(clippy::module_inception)]
        #![allow(clippy::too_many_arguments)]
        #![allow(clippy::ptr_arg)]
        #![allow(clippy::large_enum_variant)]
        #generated_by
        #cfgs
    })
}
