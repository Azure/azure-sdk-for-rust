use crate::{codegen::create_generated_by_header, config_parser::Tag, identifier::parse_ident, write_file};
use camino::Utf8Path;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] crate::io::Error),
    #[error("creating module name for feature {feature}: {source}")]
    ModName { source: crate::identifier::Error, feature: String },
}

pub fn create(tags: &[&Tag], path: &Utf8Path, print_writing_file: bool) -> Result<()> {
    Ok(write_file(path, &create_body(tags)?.into_token_stream(), print_writing_file)?)
}

struct Feature {
    pub feature_name: String,
    pub mod_name: Ident,
}

struct BodyCode {
    pub features: Vec<Feature>,
}

fn create_body(tags: &[&Tag]) -> Result<BodyCode> {
    let features: Vec<Feature> = tags
        .iter()
        .map(|tag| {
            let feature_name = tag.rust_feature_name();
            let mod_name = parse_ident(&tag.rust_mod_name()).map_err(|source| Error::ModName {
                source,
                feature: feature_name.to_owned(),
            })?;
            Ok(Feature { feature_name, mod_name })
        })
        .collect::<Result<_>>()?;
    Ok(BodyCode { features })
}

impl ToTokens for BodyCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut cfgs = TokenStream::new();
        for feature in &self.features {
            let feature_name = &feature.feature_name;
            let mod_name = &feature.mod_name;
            cfgs.extend(quote! {
                #[cfg(feature = #feature_name)]
                pub mod #mod_name;
                #[cfg(all(feature = #feature_name, not(feature = "no-default-tag")))]
                pub use #mod_name::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
            });
        }
        let generated_by = create_generated_by_header();
        tokens.extend(quote! {
            #![allow(clippy::module_inception)]
            #![allow(clippy::too_many_arguments)]
            #![allow(clippy::ptr_arg)]
            #![allow(clippy::large_enum_variant)]
            #generated_by
            #cfgs
        })
    }
}
