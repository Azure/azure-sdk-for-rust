// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(
    author,
    version,
    about = "Generate public API artifacts for a Rust crate"
)]
struct Args {
    /// Path to the Cargo.toml for the target package.
    #[arg(long, value_name = "PATH")]
    manifest_path: PathBuf,

    /// Output format to generate. Defaults to markdown.
    #[arg(long, value_enum, default_value_t = OutputFormat::Markdown)]
    format: OutputFormat,

    /// Emit Markdown API review metadata, source map, and documentation patch.
    #[arg(long)]
    review: bool,

    /// Check generated content against existing files without writing them.
    #[arg(long)]
    check: bool,

    /// Directory where generated files will be written. Defaults to the crate directory.
    #[arg(long, value_name = "DIR")]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub(crate) struct Request {
    pub(crate) manifest_path: PathBuf,
    pub(crate) format: OutputFormat,
    pub(crate) review: bool,
    pub(crate) check: bool,
    pub(crate) output_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
pub(crate) enum OutputFormat {
    Markdown,
    Apiview,
}

/// File name of the patch that adds documentation comments back to `api.md`.
pub(crate) const DOCUMENTATION_PATCH_FILE_NAME: &str = "api.documentation.patch";
pub(crate) const MARKDOWN_METADATA_FILE_NAME: &str = "api.metadata.yml";
pub(crate) const SOURCE_MAP_FILE_NAME: &str = "api.md.map";
pub(crate) const STATE_DIRECTORY_NAME: &str = "state";
pub(crate) const PACKAGE_RELATIVE_PATH_FILE_NAME: &str = "package-relative-path.txt";
pub(crate) const VERSION_FILE_NAME: &str = "version.txt";

impl OutputFormat {
    pub(crate) fn default_file_name(self) -> &'static str {
        match self {
            Self::Markdown => "api.md",
            Self::Apiview => "apiview.json",
        }
    }
}

pub(crate) fn parse() -> Result<Request, String> {
    let args = Args::parse();
    Request::try_from(args)
}

impl TryFrom<Args> for Request {
    type Error = String;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        if args.review && args.format != OutputFormat::Markdown {
            return Err("--review can only be used with --format markdown".to_string());
        }

        Ok(Self {
            manifest_path: args.manifest_path,
            format: args.format,
            review: args.review,
            check: args.check,
            output_dir: args.output,
        })
    }
}

#[cfg(test)]
mod tests;
