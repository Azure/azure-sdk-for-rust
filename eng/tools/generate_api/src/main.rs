// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod cli;
mod diagnostics;
mod driver;
mod extract;
mod model;
mod output;
mod render;

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
    let output_path = output::output_path(&request)?;
    diagnostics::info(format!("Generating file: {}", output_path.display()));

    let rendered = match request.format {
        cli::OutputFormat::Markdown => render::markdown::render(&model),
        cli::OutputFormat::Apiview => {
            let options = render::apiview::RenderOptions::new(!request.no_docs);
            render::apiview::render(&model, &options)?
        }
    };

    output::write_file(&output_path, &rendered)?;
    diagnostics::info(format!("Wrote file: {}", output_path.display()));
    Ok(())
}

fn verify_repository_root() -> Result<(), String> {
    if Path::new("eng/tools/generate_api").exists() {
        Ok(())
    } else {
        Err("This tool must be run from the root of the azure-sdk-for-rust repository.".to_string())
    }
}
