use crate::{config_parser::Tag, identifier::parse_ident, write_file};
use crate::{ErrorKind, Result, ResultExt};
use camino::Utf8Path;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::convert::{TryFrom, TryInto};

pub fn create(tags: &[&Tag], default_tag: &Tag, path: &Utf8Path, print_writing_file: bool) -> Result<()> {
    write_file(path, &create_body(tags, default_tag)?.into_token_stream(), print_writing_file)
}

struct Feature {
    pub feature_name: String,
    pub mod_name: Ident,
}

impl TryFrom<&&Tag> for Feature {
    type Error = crate::Error;
    fn try_from(tag: &&Tag) -> Result<Self> {
        let feature_name = tag.rust_feature_name();
        let mod_name = parse_ident(&tag.rust_mod_name()).context(ErrorKind::Parse, "mod name")?;
        Ok(Feature { feature_name, mod_name })
    }
}

struct BodyCode {
    pub default: Feature,
    pub features: Vec<Feature>,
}

fn create_body(tags: &[&Tag], default_tag: &Tag) -> Result<BodyCode> {
    let features: Vec<Feature> = tags.iter().map(|tag| tag.try_into()).collect::<Result<_>>()?;
    let default = (&default_tag).try_into()?;

    Ok(BodyCode { features, default })
}

impl ToTokens for BodyCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut cfgs = TokenStream::new();

        for feature in &self.features {
            let Feature { feature_name, mod_name } = feature;
            cfgs.extend(quote! {
                #[cfg(feature = #feature_name)]
                pub mod #mod_name;
            });
        }

        {
            let Feature { feature_name, mod_name } = &self.default;
            cfgs.extend(quote! {
                #[cfg(all(feature="default_tag", feature = #feature_name))]
                pub use #mod_name::*;
            });
        }
        tokens.extend(quote! {
            #![allow(clippy::module_inception)]
            #![allow(clippy::too_many_arguments)]
            #![allow(clippy::ptr_arg)]
            #![allow(clippy::large_enum_variant)]
            #![allow(clippy::derive_partial_eq_without_eq)]
            #![allow(rustdoc::bare_urls)]
            #![allow(rustdoc::invalid_html_tags)]
            #![allow(rustdoc::broken_intra_doc_links)]
            #cfgs
        });
    }
}
