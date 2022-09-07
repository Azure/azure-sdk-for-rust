use crate::Result;
use crate::{config_parser::Tag, jinja::CargoToml};
use camino::Utf8Path;

pub fn create(package_name: &str, tags: &[&Tag], default_tag: &Tag, has_xml: bool, path: &Utf8Path) -> Result<()> {
    let default_feature = &default_tag.rust_feature_name();

    // https://docs.rs/about/metadata
    // let docs_rs_features = docs_rs_features(tags, &default_feature);

    let features: Vec<_> = tags.iter().map(|tag| tag.rust_feature_name()).collect();
    let azure_core_features = if has_xml { vec!["xml"] } else { Vec::new() };
    let cargo_toml = CargoToml {
        package_name,
        default_feature,
        features,
        azure_core_features,
    };
    cargo_toml.create(path)?;
    Ok(())
}

pub fn get_default_tag<'a>(tags: &[&'a Tag], default_tag: Option<&str>) -> &'a Tag {
    if let Some(default_tag) = default_tag {
        if let Some(tag) = tags.iter().find(|tag| tag.name() == default_tag) {
            return tag;
        }
    }
    let tag = tags.iter().find(|tag| !tag.name().contains("preview"));
    match tag {
        Some(tag) => tag,
        None => tags[0],
    }
}

// const MAX_DOCS_RS_FEATURES: usize = 4;
// const NO_DEFAULT_TAG: &str = "no-default-tag";

// /// Get a list of features to document at docs.rs in addition the default
// fn docs_rs_features(tags: &[&Tag], default_feature: &str) -> Vec<String> {
//     let mut features: Vec<_> = tags
//         .iter()
//         .filter_map(|tag| {
//             let feature = tag.rust_feature_name();
//             if feature == default_feature {
//                 None
//             } else {
//                 Some(feature)
//             }
//         })
//         .collect();
//     features.truncate(MAX_DOCS_RS_FEATURES);
//     features.insert(0, NO_DEFAULT_TAG.to_owned());
//     features
// }
