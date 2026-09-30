// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod cli;
mod diagnostics;
mod driver;
mod extract;
mod model;
mod output;
mod render;
mod rustdoc_compat;
mod source_cache;
mod source_map;

use std::path::Path;

fn main() {
    if let Err(error) = run() {
        diagnostics::fatal(&error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let request = cli::parse()?;
    let repository_root = resolve_repository_root(&request)?;
    std::env::set_current_dir(&repository_root).map_err(|error| {
        format!(
            "Failed to set current directory to repository root '{}': {error}",
            repository_root.display()
        )
    })?;

    diagnostics::info(format!(
        "Using toolchain channel: {}",
        env!("TOOLCHAIN_CHANNEL")
    ));
    diagnostics::info(format!(
        "Using repository root: {}",
        repository_root.display()
    ));
    diagnostics::info(format!(
        "Loading manifest: {}",
        request.manifest_path.display()
    ));

    let loaded_package = driver::load_model(&request)?;
    let output_dir = request.output_dir(&loaded_package.package_dir);
    let working_dir = request.working_dir(&loaded_package.package_dir);
    let model = &loaded_package.model;
    let output_path = output::output_path(output_dir, request.format);
    diagnostics::info(format!("Generating file: {}", output_path.display()));

    match request.format {
        cli::OutputFormat::Markdown => {
            let lines = render::markdown::render_lines(model);
            let rendered = render::markdown::render_from_lines(&lines);
            save_or_check(&request, &output_path, &rendered)?;

            if request.review {
                let metadata_path =
                    output::output_file_path(output_dir, cli::MARKDOWN_METADATA_FILE_NAME);
                let rust_version = driver::rust_version()?;
                let metadata = output::render_markdown_metadata(
                    &rendered,
                    &model.package_version,
                    &model.parser_version,
                    &rust_version,
                )?;
                save_or_check(&request, &metadata_path, &metadata)?;

                let map_path = output::output_file_path(output_dir, cli::SOURCE_MAP_FILE_NAME);
                let mappings = render::markdown::source_mappings_from_lines(&lines);
                let map = source_map::render(
                    cli::OutputFormat::Markdown.default_file_name(),
                    &mappings,
                    output_dir,
                    &repository_root,
                )?;
                save_or_check(&request, &map_path, &map)?;

                let patch_path =
                    output::output_file_path(output_dir, cli::DOCUMENTATION_PATCH_FILE_NAME);
                let file_name = request.format.default_file_name();
                let patch = render::patch::render(&lines, file_name);
                save_or_check(&request, &patch_path, &patch)?;

                let version_path = output::state_file_path(working_dir, cli::VERSION_FILE_NAME);
                save_or_check(
                    &request,
                    &version_path,
                    &format!("{}\n", model.package_version),
                )?;

                let package_relative_path =
                    output::state_file_path(working_dir, cli::PACKAGE_RELATIVE_PATH_FILE_NAME);
                save_or_check(
                    &request,
                    &package_relative_path,
                    &format!("{}\n", loaded_package.package_relative_path),
                )?;
            }
        }
        cli::OutputFormat::Apiview => {
            let options = render::apiview::RenderOptions::new(true);
            let rendered = render::apiview::render(model, &options)?;
            save_or_check(&request, &output_path, &rendered)?;
        }
    }

    Ok(())
}

fn save_or_check(request: &cli::Request, path: &Path, contents: &str) -> Result<(), String> {
    if request.check {
        if output::check_file(path, contents)? {
            diagnostics::info(format!("Generated content matches: {}", path.display()));
        } else {
            diagnostics::info(format!("No existing file to check: {}", path.display()));
        }
    } else {
        output::write_file(path, contents)?;
        diagnostics::info(format!("Wrote file: {}", path.display()));
    }

    Ok(())
}

fn resolve_repository_root(request: &cli::Request) -> Result<std::path::PathBuf, String> {
    let repository_root = if let Some(root) = &request.root {
        root.clone()
    } else {
        std::env::current_dir().map_err(|error| {
            format!("Failed to resolve current directory as repository root: {error}")
        })?
    };

    verify_repository_root(&repository_root)?;
    std::path::absolute(&repository_root).map_err(|error| {
        format!(
            "Failed to resolve repository root '{}': {error}",
            repository_root.display()
        )
    })
}

fn verify_repository_root(repository_root: &Path) -> Result<(), String> {
    if repository_root.join("eng/tools/generate_api").exists() {
        Ok(())
    } else {
        Err(format!(
            "Repository root '{}' does not contain eng/tools/generate_api.",
            repository_root.display()
        ))
    }
}
