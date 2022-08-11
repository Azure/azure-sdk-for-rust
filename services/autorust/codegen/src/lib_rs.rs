use crate::{config_parser::Tag, identifier::parse_ident, write_file};
use crate::{ErrorKind, Result, ResultExt};
use camino::Utf8Path;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

pub fn create(tags: &[&Tag], path: &Utf8Path, print_writing_file: bool) -> Result<()> {
    write_file(path, &create_body(tags)?.into_token_stream(), print_writing_file)
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
            let mod_name = parse_ident(&tag.rust_mod_name()).context(ErrorKind::Parse, "mod name")?;
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
                pub use #mod_name::{models, Client, ClientBuilder};
            });
        }
        tokens.extend(quote! {
            #![allow(clippy::module_inception)]
            #![allow(clippy::too_many_arguments)]
            #![allow(clippy::ptr_arg)]
            #![allow(clippy::large_enum_variant)]
            #![allow(clippy::derive_partial_eq_without_eq)]
            #cfgs
        })
    }
}
