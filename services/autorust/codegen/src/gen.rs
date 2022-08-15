use crate::{
    autorust_toml, cargo_toml, io, lib_rs,
    readme_md::{self, ReadmeMd},
    CrateConfig, Error, Result, RunConfig, SpecReadme,
};
use std::{collections::HashMap, fs};

/// Get the package name, such as "azure_svc_blobstorage".
/// It is a concatenation of the prefix such as "azure_svc" & the service name such as "blobstorage".
pub fn package_name(spec: &SpecReadme, run_config: &RunConfig) -> String {
    format!("{}{}", &run_config.crate_name_prefix, &spec.service_name())
}

pub fn gen_crate(spec: &SpecReadme, run_config: &RunConfig, output_folder: &str) -> Result<()> {
    let spec_config = spec.config()?;
    let service_name = &spec.service_name();
    let package_name = &package_name(spec, run_config);
    let output_folder = &io::join(output_folder, service_name)?;
    let mut package_config = autorust_toml::read(&io::join(&output_folder, "autorust.toml")?)?;
    if package_config.tags.sort.is_none() {
        package_config.tags.sort = Some(true);
    }
    if package_config.tags.deny_contains_only.is_none() {
        package_config.tags.deny_contains_only = Some(true);
    }
    if package_config.tags.limit.is_none() {
        package_config.tags.limit = Some(5);
    }
    let tags = &package_config.filter_tags(spec_config.tags());
    if tags.is_empty() {
        println!("not generating {} - no tags", spec.spec());
        return Ok(());
    }

    let src_folder = io::join(output_folder, "src")?;
    if src_folder.exists() {
        fs::remove_dir_all(&src_folder)?;
    }

    let mut operation_totals = HashMap::new();
    let mut api_version_totals = HashMap::new();
    let mut api_versions = HashMap::new();
    for tag in tags {
        println!("  {}", tag.name());
        let output_folder = io::join(&src_folder, &tag.rust_mod_name())?;
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
        let cg = crate::run(crate_config, &package_config)?;
        operation_totals.insert(tag.name(), cg.spec.operations()?.len());
        let mut versions = cg.spec.api_versions();
        versions.sort_unstable();
        api_version_totals.insert(tag.name(), versions.len());
        api_versions.insert(
            tag.name(),
            versions.iter().map(|v| format!("`{}`", v)).collect::<Vec<_>>().join(", "),
        );
    }

    let default_tag_name = if let Some(name) = package_config.default_tag() {
        Some(name)
    } else {
        spec_config.tag()
    };
    let default_tag = cargo_toml::get_default_tag(tags, default_tag_name);
    cargo_toml::create(package_name, tags, default_tag, &io::join(output_folder, "Cargo.toml")?)?;
    lib_rs::create(tags, &io::join(src_folder, "lib.rs")?, false)?;
    let readme = ReadmeMd {
        package_name,
        readme_url: readme_md::url(spec.readme().as_str()),
        tags,
        default_tag,
        operation_totals,
        api_version_totals,
        api_versions,
    };
    readme.create(&io::join(output_folder, "README.md")?)?;

    Ok(())
}
