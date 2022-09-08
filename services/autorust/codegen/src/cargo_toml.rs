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
    let default_tag = tags.iter().find(|tag| Some(tag.name()) == default_tag);
    let is_preview = default_tag.map(|tag| tag.name().contains("preview")).unwrap_or_default();
    let stable_tag = tags.iter().find(|tag| !tag.name().contains("preview"));
    match (default_tag, is_preview, stable_tag) {
        (Some(tag), false, _) => tag,
        (Some(tag), true, None) => tag,
        (_, _, Some(tag)) => tag,
        _ => tags[0],
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

#[cfg(test)]
mod tests {
    use super::*;

    // Some readme.md are specifying the default tag as a preview.
    // If there is a stable version, we should use it instead.
    #[test]
    fn default_tag_is_stable_if_available() -> Result<()> {
        let tags = vec![
            "package-preview-2022-05",
            "package-2021-06",
            "package-2020-09",
            "package-2020-04",
            "package-2019-12",
            "package-2019-06-preview",
            "package-2019-06",
            "package-2019-04",
            "package-2017-10",
            "package-2017-04",
        ];
        let tags: Vec<_> = tags.into_iter().map(Tag::new).collect();
        let tags: Vec<_> = tags.iter().collect();
        assert_eq!("package-2021-06", get_default_tag(&tags, Some("package-preview-2022-05")).name());
        Ok(())
    }

    #[test]
    fn default_tag_is_stable() -> Result<()> {
        let tags = vec![
            "package-preview-2022-05",
            "package-2021-06",
            "package-2020-09",
            "package-2020-04",
            "package-2019-12",
            "package-2019-06-preview",
            "package-2019-06",
            "package-2019-04",
            "package-2017-10",
            "package-2017-04",
        ];
        let tags: Vec<_> = tags.into_iter().map(Tag::new).collect();
        let tags: Vec<_> = tags.iter().collect();
        assert_eq!("package-2021-06", get_default_tag(&tags, None).name());
        Ok(())
    }

    #[test]
    fn specified_tag() -> Result<()> {
        let tags = vec![
            "package-preview-2022-05",
            "package-2021-06",
            "package-2020-09",
            "package-2020-04",
            "package-2019-12",
            "package-2019-06-preview",
            "package-2019-06",
            "package-2019-04",
            "package-2017-10",
            "package-2017-04",
        ];
        let tags: Vec<_> = tags.into_iter().map(Tag::new).collect();
        let tags: Vec<_> = tags.iter().collect();
        assert_eq!("package-2020-04", get_default_tag(&tags, Some("package-2020-04")).name());
        Ok(())
    }

    #[test]
    fn specified_preview() -> Result<()> {
        let tags = vec!["package-preview-2022-05", "package-2019-06-preview", "package-2019-04-preview"];
        let tags: Vec<_> = tags.into_iter().map(Tag::new).collect();
        let tags: Vec<_> = tags.iter().collect();
        assert_eq!(
            "package-2019-06-preview",
            get_default_tag(&tags, Some("package-2019-06-preview")).name()
        );
        Ok(())
    }
}
