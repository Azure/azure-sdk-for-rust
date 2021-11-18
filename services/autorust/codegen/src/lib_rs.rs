use crate::{codegen::create_generated_by_header, identifier::ident, write_file};
use proc_macro2::TokenStream;
use quote::quote;
use std::path::Path;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("creating module name for feature {feature}: {source}")]
    ModName { source: crate::identifier::Error, feature: String },
    #[error("WriteFileError")]
    WriteFile(#[source] crate::Error),
}

pub fn create(feature_mod_names: &[(String, String)], path: &Path, print_writing_file: bool) -> Result<()> {
    write_file(path, &create_body(feature_mod_names)?, print_writing_file).map_err(Error::WriteFile)?;
    Ok(())
}

fn create_body(feature_mod_names: &[(String, String)]) -> Result<TokenStream> {
    let mut cfgs = TokenStream::new();
    for (feature_name, mod_name) in feature_mod_names {
        let mod_name = ident(mod_name).map_err(|source| Error::ModName {
            source,
            feature: feature_name.to_owned(),
        })?;
        cfgs.extend(quote! {
            #[cfg(feature = #feature_name)]
            pub mod #mod_name;
            #[cfg(all(feature = #feature_name, not(feature = "no-default-version")))]
            pub use #mod_name::{models, operations, operations::Client, operations::Error};
        });
    }
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
