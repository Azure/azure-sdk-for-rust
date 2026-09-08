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
    verify_repository_root()?;

    let request = cli::parse();
    diagnostics::info(format!(
        "Using toolchain channel: {}",
        env!("TOOLCHAIN_CHANNEL")
    ));
    diagnostics::info(format!(
        "Loading manifest: {}",
        request.manifest_path.display()
    ));

    let model = driver::load_model(&request)?;
    let output_path = output::output_path(&request);
    diagnostics::info(format!("Generating file: {}", output_path.display()));

    match request.format {
        cli::OutputFormat::Markdown => {
            let lines = render::markdown::render_lines(&model);
            let rendered = render::markdown::render_from_lines(&lines);
            save_or_check(&request, &output_path, &rendered)?;

            if !request.no_map {
                let map_path = output::output_file_path(&request, cli::SOURCE_MAP_FILE_NAME);
                let mappings = render::markdown::source_mappings_from_lines(&lines);
                let repository_root = std::env::current_dir()
                    .map_err(|error| format!("Failed to resolve repository root: {error}"))?;
                let map = source_map::render(
                    cli::OutputFormat::Markdown.default_file_name(),
                    &mappings,
                    &request.output_dir,
                    &repository_root,
                )?;
                save_or_check(&request, &map_path, &map)?;
            }

            if !request.no_docs {
                let patch_path = output::output_file_path(&request, cli::COMMENTS_PATCH_FILE_NAME);
                let file_name = request.format.default_file_name();
                let patch = render::patch::render(&lines, file_name);
                save_or_check(&request, &patch_path, &patch)?;
            }
        }
        cli::OutputFormat::Apiview => {
            let options = render::apiview::RenderOptions::new(!request.no_docs);
            let rendered = render::apiview::render(&model, &options)?;
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

fn verify_repository_root() -> Result<(), String> {
    if Path::new("eng/tools/generate_api").exists() {
        Ok(())
    } else {
        Err("This tool must be run from the root of the azure-sdk-for-rust repository.".to_string())
    }
}
