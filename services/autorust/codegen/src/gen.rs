use crate::{
    autorust_toml, cargo_toml, io, lib_rs,
    readme_md::{self, ReadmeMd},
    run, CrateConfig, Error, Result, RunConfig, SpecReadme,
};
use std::{collections::HashMap, fs};

/// Get the package name, such as "azure_svc_blobstorage".
/// It is a concatenation of the prefix such as "azure_svc" & the service name such as "blobstorage".
pub fn package_name(spec: &SpecReadme, run_config: &RunConfig) -> String {
    format!("{}{}", &run_config.crate_name_prefix, &spec.service_name())
}

pub fn gen_crate(package_name: &str, spec: &SpecReadme, run_config: &RunConfig, output_folder: &str) -> Result<Vec<String>> {
    let mut generated_tags = vec![];
    let spec_config = spec.config()?;
    let service_name = &spec.service_name();
    let output_folder = &io::join(output_folder, service_name)?;
    let mut package_config = autorust_toml::read(&io::join(output_folder, "autorust.toml")?)?;
    if package_config.tags.sort.is_none() {
        package_config.tags.sort = Some(true);
    }
    if package_config.tags.deny_contains_only.is_none() {
        package_config.tags.deny_contains_only = Some(true);
    }
    if package_config.tags.limit.is_none() {
        package_config.tags.limit = Some(5);
    }

    let src_folder = io::join(output_folder, "src")?;
    let lib_rs_path = &io::join(&src_folder, "lib.rs")?;

    if src_folder.exists() {
        fs::remove_dir_all(&src_folder)?;
    }

    let readme_path = io::join(output_folder, "README.md")?;
    if readme_path.exists() {
        std::fs::remove_file(&readme_path)?;
    }

    let cargo_toml_path = io::join(output_folder, "Cargo.toml")?;
    if cargo_toml_path.exists() {
        std::fs::remove_file(&cargo_toml_path)?;
    }

    let tags = &package_config.filter_tags(spec_config.tags());
    if tags.is_empty() {
        return Ok(generated_tags);
    }

    let mut operation_totals = HashMap::new();
    let mut api_version_totals = HashMap::new();
    let mut api_versions = HashMap::new();
    let mut has_xml = false;
    for tag in tags {
        generated_tags.push(tag.name().to_owned());
        let output_folder = io::join(&src_folder, tag.rust_mod_name())?;
        let input_files: Result<Vec<_>> = tag
            .input_files()
            .iter()
            .map(|input_file| io::join(spec.readme(), input_file).map_err(Error::from))
            .collect();
        let input_files = input_files?;
        let crate_config = &CrateConfig {
            run_config,
            output_folder,
            input_files,
        };
        let cg = run(crate_config, &package_config)?;
        let operations = cg.spec.operations()?;
        operation_totals.insert(tag.name(), operations.len());
        let mut versions = cg.spec.api_versions();
        versions.sort_unstable();
        api_version_totals.insert(tag.name(), versions.len());
        api_versions.insert(tag.name(), versions.iter().map(|v| format!("`{v}`")).collect::<Vec<_>>().join(", "));
        has_xml = cg.has_xml();
    }

    let default_tag_name = if let Some(name) = package_config.default_tag() {
        Some(name)
    } else {
        spec_config.tag()
    };
    let default_tag = cargo_toml::get_default_tag(tags, default_tag_name);

    cargo_toml::create(package_name, tags, default_tag, has_xml, &cargo_toml_path)?;
    lib_rs::create(tags, lib_rs_path, false)?;
    let readme = ReadmeMd {
        package_name,
        readme_url: readme_md::url(spec.readme().as_str()),
        tags,
        default_tag,
        operation_totals,
        api_version_totals,
        api_versions,
    };
    readme.create(&readme_path)?;

    Ok(generated_tags)
}
